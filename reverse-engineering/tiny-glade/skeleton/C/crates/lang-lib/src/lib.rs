// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/lang-lib/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cff24a38eaade08ea3e1a91b269be34a3f4d1f84f6725b806d01648fbb5ea80e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 720, 2107, 2108, 3592, 3594, 4973
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use hashlink; // namespace candidate
//   package identities: [{"name":"hashlink","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.4"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use yaml_rust2; // namespace candidate
//   package identities: [{"name":"yaml-rust2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3592,"record_offset":968,"rva":40921472,"va":5409630592}
// 编译实例 RVAs: 0x2706980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141dd6540
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9635342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409630592,"raw_hex":"e8f2f96cff","size":5,"soft_fail":false,"target_va":5399995712,"va":5409631049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9635992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409630592,"raw_hex":"e868f76cff","size":5,"soft_fail":false,"target_va":5399994816,"va":5409630803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9636117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409630592,"raw_hex":"e8ebf66cff","size":5,"soft_fail":false,"target_va":5399994608,"va":5409630720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6991232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409630592,"raw_hex":"e880ad6a00","size":5,"soft_fail":false,"target_va":5416622240,"va":5409631003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3592,"record_offset":2336,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":7027,"source_line":166,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3592,"record_offset":1572,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":16674,"source_line":201,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3592,"record_offset":2616,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2392,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2796,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":6403,"source_line":1730,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2560,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1460,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2504,"parent_record_offset":968,"ancestor_record_offset":968,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3592,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":968,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3592,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":968,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":3592,"record_offset":1912,"parent_record_offset":1632,"ancestor_record_offset":968,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3592,"record_offset":2008,"parent_record_offset":1632,"ancestor_record_offset":968,"inlinee":4848,"source_line":86,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::inc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3592,"record_offset":1748,"parent_record_offset":1632,"ancestor_record_offset":968,"inlinee":7017,"source_line":73,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$2::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3592,"record_offset":1632,"parent_record_offset":1572,"ancestor_record_offset":968,"inlinee":7016,"source_line":135,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3592,"record_offset":1604,"parent_record_offset":1572,"ancestor_record_offset":968,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3592,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":968,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3592,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":968,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3592,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":968,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3592,"record_offset":1692,"parent_record_offset":1632,"ancestor_record_offset":968,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3592,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":968,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3592,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3592,"record_offset":2244,"parent_record_offset":1632,"ancestor_record_offset":968,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1940,"parent_record_offset":1912,"ancestor_record_offset":968,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1844,"parent_record_offset":1808,"ancestor_record_offset":968,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2140,"parent_record_offset":2104,"ancestor_record_offset":968,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":968,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1968,"parent_record_offset":1940,"ancestor_record_offset":968,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":968,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":968,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":968,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3592,"record_offset":2196,"parent_record_offset":2104,"ancestor_record_offset":968,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_from_yml
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2107,"record_offset":924,"rva":31288944,"va":5399998064}
// 编译实例 RVAs: 0x1dd6e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15353512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e858b915ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5399998419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-24319661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e853e98cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5399998360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31276265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e817c322fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5399998452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30338142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8a21331fe","size":5,"soft_fail":false,"target_va":5369660368,"va":5399998505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142706e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9633194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8aafd9200","size":5,"soft_fail":false,"target_va":5409631776,"va":5399998577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17002489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8f96f0301","size":5,"soft_fail":false,"target_va":5417000640,"va":5399998146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16622893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e82da5fd00","size":5,"soft_fail":false,"target_va":5416621600,"va":5399998702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$yaml_rust2..yaml..Yaml$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e82cf9ffff","size":5,"soft_fail":false,"target_va":5399996896,"va":5399998639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..collections..hash..map..HashMap$LT$alloc..string..String$C$$RF$str$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8bafaffff","size":5,"soft_fail":false,"target_va":5399997808,"va":5399999153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16999351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8b7630301","size":5,"soft_fail":false,"target_va":5416998080,"va":5399998724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15246858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8f65917ff","size":5,"soft_fail":false,"target_va":5384751264,"va":5399998117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15272052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e88cf716ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5399998735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16620585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8299cfd00","size":5,"soft_fail":false,"target_va":5416619360,"va":5399998770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] yaml_rust2::yaml::YamlLoader::load_from_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399998064,"raw_hex":"e8ef490000","size":5,"soft_fail":false,"target_va":5400017120,"va":5399998188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2107,"record_offset":1232,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2107,"record_offset":2836,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":1288,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":1384,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2107,"record_offset":1136,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2107,"record_offset":1168,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] hashlink::linked_hash_map::LinkedHashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashlink","version":"0.8.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashlink-0.8.4/src/linked_hash_map.rs
//     evidence={"module":2107,"record_offset":1324,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":46836,"source_line":134,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":2107,"record_offset":2636,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":96180,"source_line":12,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":1952,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":2107,"record_offset":1108,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":2107,"record_offset":1260,"parent_record_offset":924,"ancestor_record_offset":924,"inlinee":96179,"source_line":472,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":924,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2760,"parent_record_offset":2736,"ancestor_record_offset":924,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":924,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":924,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":924,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":924,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":924,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2107,"record_offset":1536,"parent_record_offset":1480,"ancestor_record_offset":924,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":924,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2107,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":924,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":924,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":924,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2107,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":924,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":2492,"parent_record_offset":2464,"ancestor_record_offset":924,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":2392,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2107,"record_offset":3124,"parent_record_offset":3068,"ancestor_record_offset":924,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2107,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":924,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2107,"record_offset":3412,"parent_record_offset":3284,"ancestor_record_offset":924,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2107,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":924,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2107,"record_offset":2520,"parent_record_offset":2464,"ancestor_record_offset":924,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2107,"record_offset":2100,"parent_record_offset":2064,"ancestor_record_offset":924,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2107,"record_offset":2220,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2107,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":924,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2107,"record_offset":2308,"parent_record_offset":2188,"ancestor_record_offset":924,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2107,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":924,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":924,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2107,"record_offset":3284,"parent_record_offset":3256,"ancestor_record_offset":924,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2107,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":924,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":3228,"parent_record_offset":2872,"ancestor_record_offset":924,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":2872,"parent_record_offset":2836,"ancestor_record_offset":924,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2107,"record_offset":3256,"parent_record_offset":2872,"ancestor_record_offset":924,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":2008,"parent_record_offset":1980,"ancestor_record_offset":924,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":924,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2107,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":924,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":2464,"parent_record_offset":2064,"ancestor_record_offset":924,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2107,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":924,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":924,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2107,"record_offset":2188,"parent_record_offset":2156,"ancestor_record_offset":924,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":924,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2107,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":924,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::lang_pack_path
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB6D0
// 首条位置: {"module":720,"inlinee":46800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":720,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":720,"record_offset":2808,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":2700,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2700,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2700,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2996,"parent_record_offset":2940,"ancestor_record_offset":2700,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":720,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2700,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2700,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":2700,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2700,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2700,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":2844,"parent_record_offset":2808,"ancestor_record_offset":2700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":1624,"rva":14421936,"va":5383131056}
// 编译实例 RVAs: 0xdc0fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1525108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e874451700","size":5,"soft_fail":false,"target_va":5384656240,"va":5383131127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7452479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e8c1488eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383131178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13792353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e89f8b2dff","size":5,"soft_fail":false,"target_va":5369338848,"va":5383131196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e8c0000000","size":5,"soft_fail":false,"target_va":5383131280,"va":5383131083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e8031e0000","size":5,"soft_fail":false,"target_va":5383138784,"va":5383131096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1621309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131056,"raw_hex":"e83dbd1800","size":5,"soft_fail":false,"target_va":5384752464,"va":5383131150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":2516,"parent_record_offset":1624,"ancestor_record_offset":1624,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":720,"record_offset":1768,"parent_record_offset":1624,"ancestor_record_offset":1624,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":720,"record_offset":1740,"parent_record_offset":1624,"ancestor_record_offset":1624,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1624,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1624,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1624,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2000,"parent_record_offset":1948,"ancestor_record_offset":1624,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1624,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1624,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":1924,"parent_record_offset":1904,"ancestor_record_offset":1624,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":1804,"parent_record_offset":1768,"ancestor_record_offset":1624,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":2548,"rva":14422160,"va":5383131280}
// 编译实例 RVAs: 0xdc1090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1524796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e83c441700","size":5,"soft_fail":false,"target_va":5384656240,"va":5383131439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1477462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8568b1600","size":5,"soft_fail":false,"target_va":5384609408,"va":5383131941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-7452859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e845478eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383131542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-7452771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e89d478eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383131470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14409336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8882124ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383131523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dd7870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16868427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e84b640101","size":5,"soft_fail":false,"target_va":5400000624,"va":5383132192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33925487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e86fa90502","size":5,"soft_fail":false,"target_va":5417057056,"va":5383131564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33487365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e805fafe01","size":5,"soft_fail":false,"target_va":5416620769,"va":5383133399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1663470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8ee611900","size":5,"soft_fail":false,"target_va":5384794864,"va":5383131389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33489289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e88901ff01","size":5,"soft_fail":false,"target_va":5416620794,"va":5383131500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16862205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8fd4b0101","size":5,"soft_fail":false,"target_va":5399994816,"va":5383132606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1491786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e84ac31600","size":5,"soft_fail":false,"target_va":5384624080,"va":5383132289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16861779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8534a0101","size":5,"soft_fail":false,"target_va":5399994352,"va":5383132568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..sync..ArcInner$LT$lang_lib..language_library..LanguageLibrary$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16863624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e888510101","size":5,"soft_fail":false,"target_va":5399997536,"va":5383133907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::try_from_yml
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16866424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8785c0101","size":5,"soft_fail":false,"target_va":5399998064,"va":5383131635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl::closure$1
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t33420417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e881f4fd01","size":5,"soft_fail":false,"target_va":5416553872,"va":5383133450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_var
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1579508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383131280,"raw_hex":"e8f4191800","size":5,"soft_fail":false,"target_va":5384712144,"va":5383132631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":720,"record_offset":6364,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":720,"record_offset":6604,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":18155,"source_line":466,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":720,"record_offset":4128,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":720,"record_offset":3420,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":5600,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":720,"record_offset":8968,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":720,"record_offset":5568,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::lang_pack_path
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":720,"record_offset":2700,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":46800,"source_line":17,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":4184,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":4044,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::env::var
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\env.rs
//     evidence={"module":720,"record_offset":6960,"parent_record_offset":2548,"ancestor_record_offset":2548,"inlinee":10542,"source_line":222,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":2548,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":2548,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":2548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":2548,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":3832,"parent_record_offset":3680,"ancestor_record_offset":2548,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":2996,"parent_record_offset":2940,"ancestor_record_offset":2548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":720,"record_offset":6444,"parent_record_offset":6400,"ancestor_record_offset":2548,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":720,"record_offset":6400,"parent_record_offset":6364,"ancestor_record_offset":2548,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":720,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2548,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":720,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2548,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":3624,"parent_record_offset":3600,"ancestor_record_offset":2548,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":2548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":3680,"parent_record_offset":3648,"ancestor_record_offset":2548,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":2548,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":3720,"parent_record_offset":3680,"ancestor_record_offset":2548,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":3524,"parent_record_offset":3492,"ancestor_record_offset":2548,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":2548,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":2548,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2548,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":720,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2548,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":720,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2548,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":720,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":2548,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":720,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":2548,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":720,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":2548,"inlinee":4846,"source_line":81,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":720,"record_offset":6748,"parent_record_offset":6604,"ancestor_record_offset":2548,"inlinee":4880,"source_line":169,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":720,"record_offset":6692,"parent_record_offset":6604,"ancestor_record_offset":2548,"inlinee":4844,"source_line":196,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":720,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":2548,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":720,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":2548,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":720,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":2548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":2548,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2548,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":720,"record_offset":3960,"parent_record_offset":3572,"ancestor_record_offset":2548,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":2548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=69
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":2844,"parent_record_offset":2808,"ancestor_record_offset":2548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":6636,"parent_record_offset":6604,"ancestor_record_offset":2548,"inlinee":6416,"source_line":1764,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":6864,"parent_record_offset":6828,"ancestor_record_offset":2548,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2548,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2548,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":2548,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":2548,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":6920,"parent_record_offset":6828,"ancestor_record_offset":2548,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2548,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":2548,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":2548,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":720,"record_offset":2808,"parent_record_offset":2700,"ancestor_record_offset":2548,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x177BB
// 首条位置: {"module":2108,"inlinee":96187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2108,"record_offset":1268,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2108,"record_offset":1720,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2108,"record_offset":1352,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::lang_pack_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":2108,"record_offset":552,"parent_record_offset":516,"ancestor_record_offset":516,"inlinee":46800,"source_line":17,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2108,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":516,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2108,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":516,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2108,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":516,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2108,"record_offset":844,"parent_record_offset":788,"ancestor_record_offset":516,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2108,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":516,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2108,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":516,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2108,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":516,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2108,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":516,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2108,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":516,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2108,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":516,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2108,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":2108,"record_offset":660,"parent_record_offset":552,"ancestor_record_offset":516,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/02108.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_locale_impl::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":11752,"rva":47844752,"va":5416553872}
// 编译实例 RVAs: 0x2da0d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-31929911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416553872,"raw_hex":"e8c9c918fe","size":5,"soft_fail":false,"target_va":5384624080,"va":5416553986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t68282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416553872,"raw_hex":"e8ba0a0100","size":5,"soft_fail":false,"target_va":5416622240,"va":5416553953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":11876,"parent_record_offset":11752,"ancestor_record_offset":11752,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":11904,"rva":14425040,"va":5383134160}
// 编译实例 RVAs: 0xdc1bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1521978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e83a391700","size":5,"soft_fail":false,"target_va":5384656240,"va":5383134257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7455687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e8393c8eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5383134370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-7455605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e88b3c8eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5383134304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14412168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e8781624ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5383134355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13795482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e8667f2dff","size":5,"soft_fail":false,"target_va":5369338848,"va":5383134325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33486267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e8bbf5fe01","size":5,"soft_fail":false,"target_va":5416620794,"va":5383134522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1489864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e8c8bb1600","size":5,"soft_fail":false,"target_va":5384624080,"va":5383134211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e88af3ffff","size":5,"soft_fail":false,"target_va":5383131280,"va":5383134465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::find_standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e81d120000","size":5,"soft_fail":false,"target_va":5383138832,"va":5383134190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1616980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134160,"raw_hex":"e854ac1800","size":5,"soft_fail":false,"target_va":5384751264,"va":5383134279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":720,"record_offset":14128,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":12112,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":14596,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":720,"record_offset":12032,"parent_record_offset":11904,"ancestor_record_offset":11904,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":11904,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":11904,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":11904,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12480,"parent_record_offset":12456,"ancestor_record_offset":11904,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":11904,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":11904,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":14392,"parent_record_offset":14364,"ancestor_record_offset":11904,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12432,"parent_record_offset":12380,"ancestor_record_offset":11904,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":14304,"parent_record_offset":14276,"ancestor_record_offset":11904,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12404,"parent_record_offset":12380,"ancestor_record_offset":11904,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":11904,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":14364,"parent_record_offset":14332,"ancestor_record_offset":11904,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":14332,"parent_record_offset":14304,"ancestor_record_offset":11904,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12356,"parent_record_offset":12336,"ancestor_record_offset":11904,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":14184,"parent_record_offset":14156,"ancestor_record_offset":11904,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":14212,"parent_record_offset":14184,"ancestor_record_offset":11904,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":11904,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":720,"record_offset":14156,"parent_record_offset":14128,"ancestor_record_offset":11904,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":720,"record_offset":14276,"parent_record_offset":14240,"ancestor_record_offset":11904,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":720,"record_offset":14520,"parent_record_offset":14240,"ancestor_record_offset":11904,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":11904,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":12060,"parent_record_offset":12032,"ancestor_record_offset":11904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":720,"record_offset":12948,"parent_record_offset":12144,"ancestor_record_offset":11904,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] lang_lib::load_last_used_locale::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/lib.rs
//     evidence={"module":720,"record_offset":12144,"parent_record_offset":12112,"ancestor_record_offset":11904,"inlinee":46802,"source_line":72,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":720,"record_offset":12204,"parent_record_offset":12144,"ancestor_record_offset":11904,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":720,"record_offset":12176,"parent_record_offset":12144,"ancestor_record_offset":11904,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::load_last_used_locale::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB6D2
// 首条位置: {"module":720,"inlinee":46802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":13016,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":720,"record_offset":12948,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":720,"record_offset":12204,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":720,"record_offset":12176,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":12144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12480,"parent_record_offset":12456,"ancestor_record_offset":12144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":12144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":720,"record_offset":12432,"parent_record_offset":12380,"ancestor_record_offset":12144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12404,"parent_record_offset":12380,"ancestor_record_offset":12144,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":12144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":720,"record_offset":12356,"parent_record_offset":12336,"ancestor_record_offset":12144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":12236,"parent_record_offset":12204,"ancestor_record_offset":12144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::lookup
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":15168,"rva":14425600,"va":5383134720}
// 编译实例 RVAs: 0xdc1e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1474005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e8d57d1600","size":5,"soft_fail":false,"target_va":5384609408,"va":5383135398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9653962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e836b16cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5383134853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dd6540
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16859912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e808430101","size":5,"soft_fail":false,"target_va":5399995712,"va":5383135795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33922062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e80e9c0502","size":5,"soft_fail":false,"target_va":5417057040,"va":5383134973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16859630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e8ee410101","size":5,"soft_fail":false,"target_va":5399994816,"va":5383135181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16859803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e89b420101","size":5,"soft_fail":false,"target_va":5399994608,"va":5383134800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::language_library::LanguageLibrary::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16864084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383134720,"raw_hex":"e854530101","size":5,"soft_fail":false,"target_va":5399999184,"va":5383135095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":720,"record_offset":16572,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":720,"record_offset":15296,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":18135,"source_line":453,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":720,"record_offset":17628,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":15352,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":16600,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::try_get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":720,"record_offset":15380,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":46803,"source_line":48,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":17684,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":17544,"parent_record_offset":15168,"ancestor_record_offset":15168,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":720,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":15168,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":720,"record_offset":16800,"parent_record_offset":16648,"ancestor_record_offset":15168,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":720,"record_offset":16744,"parent_record_offset":16648,"ancestor_record_offset":15168,"inlinee":7027,"source_line":166,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":720,"record_offset":16648,"parent_record_offset":16624,"ancestor_record_offset":15168,"inlinee":7026,"source_line":104,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":720,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":15168,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":720,"record_offset":16116,"parent_record_offset":16088,"ancestor_record_offset":15168,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":720,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":15168,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":720,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":15168,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":720,"record_offset":16284,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":720,"record_offset":16884,"parent_record_offset":16648,"ancestor_record_offset":15168,"inlinee":4744,"source_line":260,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":720,"record_offset":16708,"parent_record_offset":16684,"ancestor_record_offset":15168,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":720,"record_offset":15908,"parent_record_offset":15880,"ancestor_record_offset":15168,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":720,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":15168,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":16768,"parent_record_offset":16744,"ancestor_record_offset":15168,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":16456,"parent_record_offset":15380,"ancestor_record_offset":15168,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":17652,"parent_record_offset":17628,"ancestor_record_offset":15168,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":16684,"parent_record_offset":16648,"ancestor_record_offset":15168,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":15168,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":16624,"parent_record_offset":16600,"ancestor_record_offset":15168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":720,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":15168,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":720,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":15168,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":720,"record_offset":16228,"parent_record_offset":16200,"ancestor_record_offset":15168,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":720,"record_offset":16144,"parent_record_offset":16116,"ancestor_record_offset":15168,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":15168,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":16992,"parent_record_offset":16956,"ancestor_record_offset":15168,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":17568,"parent_record_offset":17544,"ancestor_record_offset":15168,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":16848,"parent_record_offset":16824,"ancestor_record_offset":15168,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":15168,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":17016,"parent_record_offset":16992,"ancestor_record_offset":15168,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":17048,"parent_record_offset":16956,"ancestor_record_offset":15168,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15820,"parent_record_offset":15792,"ancestor_record_offset":15168,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":15168,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":720,"record_offset":15792,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":15652,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":16312,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":720,"record_offset":15736,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":720,"record_offset":15596,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":720,"record_offset":16088,"parent_record_offset":16060,"ancestor_record_offset":15168,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":15432,"parent_record_offset":15408,"ancestor_record_offset":15168,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":16060,"parent_record_offset":15952,"ancestor_record_offset":15168,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":720,"record_offset":15464,"parent_record_offset":15432,"ancestor_record_offset":15168,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15168,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":16404,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":15168,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":15168,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15492,"parent_record_offset":15432,"ancestor_record_offset":15168,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":15168,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15624,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":720,"record_offset":15952,"parent_record_offset":15540,"ancestor_record_offset":15168,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":17732,"parent_record_offset":17708,"ancestor_record_offset":15168,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":17708,"parent_record_offset":17684,"ancestor_record_offset":15168,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":15168,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":720,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":15168,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::game_locale
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":720,"record_offset":264,"rva":14421568,"va":5383130688}
// 编译实例 RVAs: 0xdc0e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1478368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383130688,"raw_hex":"e8e08e1600","size":5,"soft_fail":false,"target_va":5384609408,"va":5383131035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16864004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383130688,"raw_hex":"e804530101","size":5,"soft_fail":false,"target_va":5399994816,"va":5383130807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16863889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383130688,"raw_hex":"e891520101","size":5,"soft_fail":false,"target_va":5399994608,"va":5383130714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":720,"record_offset":396,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":18135,"source_line":453,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":720,"record_offset":1452,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":452,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":508,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] lang_lib::language_library::LanguageLibrary::locale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/lang-lib/src/language_library.rs
//     evidence={"module":720,"record_offset":480,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":46798,"source_line":19,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":1508,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":1368,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":720,"record_offset":784,"parent_record_offset":764,"ancestor_record_offset":264,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":720,"record_offset":636,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":720,"record_offset":548,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":7026,"source_line":104,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":720,"record_offset":420,"parent_record_offset":396,"ancestor_record_offset":264,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":720,"record_offset":720,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4744,"source_line":260,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":720,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":1476,"parent_record_offset":1452,"ancestor_record_offset":264,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":720,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":264,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":720,"record_offset":528,"parent_record_offset":508,"ancestor_record_offset":264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":264,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":264,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":264,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":1416,"parent_record_offset":1392,"ancestor_record_offset":264,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":264,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":720,"record_offset":872,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":264,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":720,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":264,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00720.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":720,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":264,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00720.jsonl.gz"}
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
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >
// 信息 ID: fcd48080ed52c87a8e77f00c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0 -> ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::debt::helping::Slots::help -> ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > @ 0x1dd5ef8 -> 0x2706980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::game_locale
// 信息 ID: 5dcef8f1e0b2098631d99212
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::daily_welcome::ui::closure_env$0,ref$<str$> > @ 0x1190806 -> 0xdc0e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::daily_welcome::ui::closure_env$0,ref$<str$> > @ 0x119087c -> 0xdc0e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$7,ref$<str$> > @ 0x119ae5c -> 0xdc0e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$7,ref$<str$> > @ 0x119af12 -> 0xdc0e40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::lang_pack_path
// 信息 ID: ccd7c2b9f815385ff79d517a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=720, inline_record=2700; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=720, inline_record=4296; 不是独立调用边。
// [PDB 内联宿主] lang_lib::load_locale_impl; module=720, inline_record=7696; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; module=2108, inline_record=552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_from_yml
// 信息 ID: 16693596572e0e7a3de4179e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc11f3 -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc1713 -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc140c -> 0x1dd6e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale
// 信息 ID: 36e4a64800f51bc1d7448db0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x5420b1 -> 0xdc1bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_last_used_locale::closure$0
// 信息 ID: cab4e4c6328a84fc312beaa2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_last_used_locale", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::load_last_used_locale; module=720, inline_record=12144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale
// 信息 ID: f5e7c39c422161bd32bc2b09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2ad3 -> 0xdc0fb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0> @ 0x222dda -> 0xdc0fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl
// 信息 ID: 93adfc0363dea62b1889e4e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale @ 0xdc0fcb -> 0xdc1090 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_last_used_locale @ 0xdc1d01 -> 0xdc1090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$0
// 信息 ID: f9de7a500637a72e5162eb47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::load_locale_impl::closure_env$0 <- anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>; module=2108, inline_record=516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::load_locale_impl::closure$1
// 信息 ID: 97e175e325107f5c4ae63e8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "lang_lib::load_locale_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale_impl @ 0xdc190a -> 0x2da0d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::lookup
// 信息 ID: 47ff9471759e9388d1c1f966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 312 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10 @ 0x30f2d0 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10 @ 0x30f392 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14 @ 0x30f505 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14 @ 0x30f5bf -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x30fa77 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x30fb7c -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x30fc22 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x312588 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x3125f3 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x31272f -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$8 @ 0x3127c4 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$8 @ 0x3128b2 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$4 @ 0x312a33 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x313351 -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3 @ 0x3135bd -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$2 @ 0x3139eb -> 0xdc1e00 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
