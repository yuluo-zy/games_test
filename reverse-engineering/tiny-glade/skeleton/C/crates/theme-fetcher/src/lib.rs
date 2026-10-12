// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/theme-fetcher/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f2f84f1ae48471442feb246ee14c7db8d939d3516a7c4650800d012a68c4595c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 122, 258, 890, 893, 905, 909, 910, 916, 2327, 2346, 2351, 2352, 2354, 2359
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use bytes; // namespace candidate
//   package identities: [{"name":"bytes","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.6.0"}]
// use cache_control; // namespace candidate
//   package identities: [{"name":"cache_control","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use http; // namespace candidate
//   package identities: [{"name":"http","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.9"}]
// use httpdate; // namespace candidate
//   package identities: [{"name":"httpdate","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.3"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use reqwest; // namespace candidate
//   package identities: [{"name":"reqwest","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.22"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use theme_defs; // namespace candidate
//   package identities: [{"name":"theme-defs","source":"local_or_path","version":"0.1.0"}]
// use theme_fetcher; // namespace candidate
//   package identities: [{"name":"theme-fetcher","source":"local_or_path","version":"0.1.0"}]
// use theme_server_proto; // namespace candidate
//   package identities: [{"name":"theme-server-proto","source":"local_or_path","version":"0.1.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2346,"record_offset":1780,"rva":32345056,"va":5401054176}
// 编译实例 RVAs: 0x1ed8be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140147bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31003038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054176,"raw_hex":"e862ee26fe","size":5,"soft_fail":false,"target_va":5370051568,"va":5401054601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25353365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054176,"raw_hex":"e86b237dfe","size":5,"soft_fail":false,"target_va":5375701008,"va":5401054368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-15946720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054176,"raw_hex":"e820ac0cff","size":5,"soft_fail":false,"target_va":5385107584,"va":5401054299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15567687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054176,"raw_hex":"e8478bed00","size":5,"soft_fail":false,"target_va":5416622240,"va":5401054548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":3092,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":7028,"source_line":96,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":2388,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":16674,"source_line":201,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":3400,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3176,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3644,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6403,"source_line":1730,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3344,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2276,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3288,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":1780,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":1780,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1780,"inlinee":13798,"source_line":1667,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":1780,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1780,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":2346,"record_offset":2764,"parent_record_offset":2444,"ancestor_record_offset":1780,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":2848,"parent_record_offset":2444,"ancestor_record_offset":1780,"inlinee":4848,"source_line":86,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::inc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":2552,"parent_record_offset":2444,"ancestor_record_offset":1780,"inlinee":7017,"source_line":73,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":2676,"parent_record_offset":2552,"ancestor_record_offset":1780,"inlinee":13797,"source_line":93,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$2::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":2444,"parent_record_offset":2388,"ancestor_record_offset":1780,"inlinee":7016,"source_line":135,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":2416,"parent_record_offset":2388,"ancestor_record_offset":1780,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2346,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1780,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2346,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":1780,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2346,"record_offset":2496,"parent_record_offset":2444,"ancestor_record_offset":1780,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2346,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":1780,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":3056,"parent_record_offset":2444,"ancestor_record_offset":1780,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1780,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2616,"parent_record_offset":2580,"ancestor_record_offset":1780,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2956,"parent_record_offset":2920,"ancestor_record_offset":1780,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1780,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1780,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1780,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":1780,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":1780,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":3012,"parent_record_offset":2920,"ancestor_record_offset":1780,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2346,"record_offset":3932,"rva":32345504,"va":5401054624}
// 编译实例 RVAs: 0x1ed8da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140147c30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31003422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054624,"raw_hex":"e8e2ec26fe","size":5,"soft_fail":false,"target_va":5370051632,"va":5401055049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25353669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054624,"raw_hex":"e83b227dfe","size":5,"soft_fail":false,"target_va":5375701152,"va":5401054816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-15947808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054624,"raw_hex":"e8e0a70cff","size":5,"soft_fail":false,"target_va":5385106944,"va":5401054747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15567239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401054624,"raw_hex":"e88789ed00","size":5,"soft_fail":false,"target_va":5416622240,"va":5401054996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":5260,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":7028,"source_line":96,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":4556,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":16674,"source_line":201,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":5568,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5344,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5812,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":6403,"source_line":1730,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5512,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4444,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5456,"parent_record_offset":3932,"ancestor_record_offset":3932,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":3932,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":5628,"parent_record_offset":5592,"ancestor_record_offset":3932,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":3932,"inlinee":13798,"source_line":1667,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":3932,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2346,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":3932,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":2346,"record_offset":4932,"parent_record_offset":4612,"ancestor_record_offset":3932,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":5016,"parent_record_offset":4612,"ancestor_record_offset":3932,"inlinee":4848,"source_line":86,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::inc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":4720,"parent_record_offset":4612,"ancestor_record_offset":3932,"inlinee":7017,"source_line":73,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":2346,"record_offset":4844,"parent_record_offset":4720,"ancestor_record_offset":3932,"inlinee":13797,"source_line":93,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$2::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":4612,"parent_record_offset":4556,"ancestor_record_offset":3932,"inlinee":7016,"source_line":135,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":2346,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":3932,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2346,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":3932,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2346,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":3932,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2346,"record_offset":4664,"parent_record_offset":4612,"ancestor_record_offset":3932,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":3932,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2346,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":3932,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2346,"record_offset":5224,"parent_record_offset":4612,"ancestor_record_offset":3932,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":3932,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4784,"parent_record_offset":4748,"ancestor_record_offset":3932,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5124,"parent_record_offset":5088,"ancestor_record_offset":3932,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":3932,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":3932,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":3932,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":3932,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":3932,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2346,"record_offset":5180,"parent_record_offset":5088,"ancestor_record_offset":3932,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/02346.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":11996,"rva":1431696,"va":5370140816}
// 编译实例 RVAs: 0x15d890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46183525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e865b4c002","size":5,"soft_fail":false,"target_va":5416324848,"va":5370141318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9f040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46405336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370140816,"raw_hex":"e8d816c402","size":5,"soft_fail":false,"target_va":5416546368,"va":5370141027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":905,"record_offset":12584,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7635,"source_line":1894,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$2::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":905,"record_offset":12612,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7636,"source_line":1936,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":12264,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::f64_from_parts
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":12640,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7627,"source_line":639,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":12368,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":12176,"parent_record_offset":11996,"ancestor_record_offset":11996,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":905,"record_offset":12804,"parent_record_offset":12640,"ancestor_record_offset":11996,"inlinee":7631,"source_line":625,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::wrapping_abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":905,"record_offset":12708,"parent_record_offset":12640,"ancestor_record_offset":11996,"inlinee":7629,"source_line":2397,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":905,"record_offset":12764,"parent_record_offset":12708,"ancestor_record_offset":11996,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":11996,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":12284,"parent_record_offset":12264,"ancestor_record_offset":11996,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11996,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":11996,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":9272,"rva":14163936,"va":5382873056}
// 编译实例 RVAs: 0xd81fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33449980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e8fc67fe01","size":5,"soft_fail":false,"target_va":5416323136,"va":5382873151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33451354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e85a6dfe01","size":5,"soft_fail":false,"target_va":5416324848,"va":5382873489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9ed40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33672319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fcc0102","size":5,"soft_fail":false,"target_va":5416545600,"va":5382873276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-1018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e806fcffff","size":5,"soft_fail":false,"target_va":5382872496,"va":5382873509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t-12732545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382873056,"raw_hex":"e87fb73dff","size":5,"soft_fail":false,"target_va":5370140816,"va":5382873356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9684,"parent_record_offset":9272,"ancestor_record_offset":9272,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::next_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9440,"parent_record_offset":9272,"ancestor_record_offset":9272,"inlinee":7587,"source_line":231,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_number
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9740,"parent_record_offset":9272,"ancestor_record_offset":9272,"inlinee":7872,"source_line":509,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9496,"parent_record_offset":9272,"ancestor_record_offset":9272,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":9272,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek_or_null
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":9800,"parent_record_offset":9740,"ancestor_record_offset":9272,"inlinee":7625,"source_line":223,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":9272,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":9460,"parent_record_offset":9440,"ancestor_record_offset":9272,"inlinee":7588,"source_line":545,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":9552,"parent_record_offset":9520,"ancestor_record_offset":9272,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":31656,"rva":16387216,"va":5385096336}
// 编译实例 RVAs: 0xfa0c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-9417944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e8284b70ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385096643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15757899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e8b58d0fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385096742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14954372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e87cd01bff","size":5,"soft_fail":false,"target_va":5370142144,"va":5385096511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0100
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e84ef2ffff","size":5,"soft_fail":false,"target_va":5385093376,"va":5385096877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ed8730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15956471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e8f779f300","size":5,"soft_fail":false,"target_va":5401052976,"va":5385096500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31226674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e8327bdc01","size":5,"soft_fail":false,"target_va":5416323136,"va":5385096457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31257349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e805f3dc01","size":5,"soft_fail":false,"target_va":5416354128,"va":5385096774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db25b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31528764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385096336,"raw_hex":"e83c17e101","size":5,"soft_fail":false,"target_va":5416625584,"va":5385096815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":32144,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":32088,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":33456,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":31912,"parent_record_offset":31656,"ancestor_record_offset":31656,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":32348,"parent_record_offset":32324,"ancestor_record_offset":31656,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":32372,"parent_record_offset":32348,"ancestor_record_offset":31656,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":31656,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":32324,"parent_record_offset":32292,"ancestor_record_offset":31656,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":905,"record_offset":32292,"parent_record_offset":32264,"ancestor_record_offset":31656,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":32992,"parent_record_offset":32960,"ancestor_record_offset":31656,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":32960,"parent_record_offset":32932,"ancestor_record_offset":31656,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":32932,"parent_record_offset":32904,"ancestor_record_offset":31656,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":32764,"parent_record_offset":32740,"ancestor_record_offset":31656,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":33296,"parent_record_offset":33260,"ancestor_record_offset":31656,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":32176,"parent_record_offset":32144,"ancestor_record_offset":31656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":32028,"parent_record_offset":31912,"ancestor_record_offset":31656,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":31960,"parent_record_offset":31912,"ancestor_record_offset":31656,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":32052,"parent_record_offset":32028,"ancestor_record_offset":31656,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":31988,"parent_record_offset":31960,"ancestor_record_offset":31656,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":29520,"rva":16386544,"va":5385095664}
// 编译实例 RVAs: 0xfa09f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-9417272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e8c84d70ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385095971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15757269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e82b900fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385096112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14953700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e81cd31bff","size":5,"soft_fail":false,"target_va":5370142144,"va":5385095839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa03d0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e89ef7ffff","size":5,"soft_fail":false,"target_va":5385094096,"va":5385096237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ed84f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15956567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e8577af300","size":5,"soft_fail":false,"target_va":5401052400,"va":5385095828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31227346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e8d27ddc01","size":5,"soft_fail":false,"target_va":5416323136,"va":5385095785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31257979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e87bf5dc01","size":5,"soft_fail":false,"target_va":5416354128,"va":5385096144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db25b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31529394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385095664,"raw_hex":"e8b219e101","size":5,"soft_fail":false,"target_va":5416625584,"va":5385096185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":30028,"parent_record_offset":29520,"ancestor_record_offset":29520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":29972,"parent_record_offset":29520,"ancestor_record_offset":29520,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":31624,"parent_record_offset":29520,"ancestor_record_offset":29520,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":29796,"parent_record_offset":29520,"ancestor_record_offset":29520,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":30232,"parent_record_offset":30208,"ancestor_record_offset":29520,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":30256,"parent_record_offset":30232,"ancestor_record_offset":29520,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":30280,"parent_record_offset":30256,"ancestor_record_offset":29520,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":30208,"parent_record_offset":30176,"ancestor_record_offset":29520,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":905,"record_offset":30176,"parent_record_offset":30148,"ancestor_record_offset":29520,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":30876,"parent_record_offset":30844,"ancestor_record_offset":29520,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":30844,"parent_record_offset":30816,"ancestor_record_offset":29520,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":30816,"parent_record_offset":30788,"ancestor_record_offset":29520,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":29520,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":31464,"parent_record_offset":31428,"ancestor_record_offset":29520,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":30060,"parent_record_offset":30028,"ancestor_record_offset":29520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":29912,"parent_record_offset":29796,"ancestor_record_offset":29520,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":29844,"parent_record_offset":29796,"ancestor_record_offset":29520,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":29936,"parent_record_offset":29912,"ancestor_record_offset":29520,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":29520,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":34644,"rva":16388192,"va":5385097312}
// 编译实例 RVAs: 0xfa1060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..SliceRead$u20$as$u20$serde_json..read..Read$GT$::parse_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1842802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e88ee1e3ff","size":5,"soft_fail":false,"target_va":5383254800,"va":5385097597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t-9419628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8944470ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385098327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=4
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15867283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e86de20dff","size":5,"soft_fail":false,"target_va":5369231648,"va":5385098926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15760370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e80e840fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385099213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015dd10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14956633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8a7c71bff","size":5,"soft_fail":false,"target_va":5370141968,"va":5385098596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14956102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8bac91bff","size":5,"soft_fail":false,"target_va":5370142144,"va":5385098241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14943891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e86df91bff","size":5,"soft_fail":false,"target_va":5370153648,"va":5385097534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14943414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e84afb1bff","size":5,"soft_fail":false,"target_va":5370154720,"va":5385098129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa09f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8fff6ffff","size":5,"soft_fail":false,"target_va":5385095664,"va":5385097964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0f10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8c5fbffff","size":5,"soft_fail":false,"target_va":5385096976,"va":5385098054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=5
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31225692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e85c77dc01","size":5,"soft_fail":false,"target_va":5416323136,"va":5385097439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31254400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e880e7dc01","size":5,"soft_fail":false,"target_va":5416354128,"va":5385099723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d706e0
//     owner=unresolved_namespace; count=4
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31256795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8dbf0dc01","size":5,"soft_fail":false,"target_va":5416355552,"va":5385098752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db25b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31525794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8a20be101","size":5,"soft_fail":false,"target_va":5416625584,"va":5385099785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8dbe5ffff","size":5,"soft_fail":false,"target_va":5385093680,"va":5385100368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::Error::invalid_length
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31830886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e866b3e501","size":5,"soft_fail":false,"target_va":5416929920,"va":5385099029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::MapAccess::next_value<serde_json::de::MapAccess<serde_json::read::SliceRead>,serde_core::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385097312,"raw_hex":"e8d2e4ffff","size":5,"soft_fail":false,"target_va":5385090864,"va":5385097817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":40000,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35060,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":43960,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":34884,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35116,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":63897,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":38664,"parent_record_offset":34644,"ancestor_record_offset":34644,"inlinee":63901,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36940,"parent_record_offset":36916,"ancestor_record_offset":34644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36964,"parent_record_offset":36940,"ancestor_record_offset":34644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36988,"parent_record_offset":36964,"ancestor_record_offset":34644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36916,"parent_record_offset":36852,"ancestor_record_offset":34644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":905,"record_offset":41508,"parent_record_offset":41484,"ancestor_record_offset":34644,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36884,"parent_record_offset":36852,"ancestor_record_offset":34644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36852,"parent_record_offset":36824,"ancestor_record_offset":34644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36824,"parent_record_offset":36796,"ancestor_record_offset":34644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":35456,"parent_record_offset":35348,"ancestor_record_offset":34644,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":37072,"parent_record_offset":37048,"ancestor_record_offset":34644,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":905,"record_offset":35540,"parent_record_offset":35516,"ancestor_record_offset":34644,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":37856,"parent_record_offset":37828,"ancestor_record_offset":34644,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":905,"record_offset":35772,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":36740,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":905,"record_offset":36676,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":34644,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":34644,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":905,"record_offset":35516,"parent_record_offset":35484,"ancestor_record_offset":34644,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":38016,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":38088,"parent_record_offset":38064,"ancestor_record_offset":34644,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":38064,"parent_record_offset":38040,"ancestor_record_offset":34644,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":34644,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38296,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38112,"parent_record_offset":38088,"ancestor_record_offset":34644,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35176,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35800,"parent_record_offset":35116,"ancestor_record_offset":34644,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38712,"parent_record_offset":38664,"ancestor_record_offset":34644,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35264,"parent_record_offset":35212,"ancestor_record_offset":34644,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":36184,"parent_record_offset":36160,"ancestor_record_offset":34644,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":36636,"parent_record_offset":36612,"ancestor_record_offset":34644,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35000,"parent_record_offset":34884,"ancestor_record_offset":34644,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35884,"parent_record_offset":35840,"ancestor_record_offset":34644,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35928,"parent_record_offset":35884,"ancestor_record_offset":34644,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":34932,"parent_record_offset":34884,"ancestor_record_offset":34644,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35212,"parent_record_offset":35176,"ancestor_record_offset":34644,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35840,"parent_record_offset":35800,"ancestor_record_offset":34644,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35348,"parent_record_offset":35320,"ancestor_record_offset":34644,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":35320,"parent_record_offset":35292,"ancestor_record_offset":34644,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":38740,"parent_record_offset":38712,"ancestor_record_offset":34644,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":34644,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":34960,"parent_record_offset":34932,"ancestor_record_offset":34644,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35484,"parent_record_offset":35348,"ancestor_record_offset":34644,"inlinee":63900,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35292,"parent_record_offset":35264,"ancestor_record_offset":34644,"inlinee":63899,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":905,"record_offset":43992,"rva":16391552,"va":5385100672}
// 编译实例 RVAs: 0xfa1d80, 0xfa2500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..read..SliceRead$u20$as$u20$serde_json..read..Read$GT$::parse_str
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1846146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e87ed4e3ff","size":5,"soft_fail":false,"target_va":5383254800,"va":5385100941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-9422791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e8393870ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385101490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15762710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e8ea7a0fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385101553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015dd10
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14959957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e8abba1bff","size":5,"soft_fail":false,"target_va":5370141968,"va":5385101920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015ddc0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14959295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e841bd1bff","size":5,"soft_fail":false,"target_va":5370142144,"va":5385101434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015f3d0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14957417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385102592,"raw_hex":"e897c41bff","size":5,"soft_fail":false,"target_va":5370147792,"va":5385105204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14947243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e855ec1bff","size":5,"soft_fail":false,"target_va":5370153648,"va":5385100886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140160ee0
//     owner=unresolved_namespace; count=4
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14946505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e837ef1bff","size":5,"soft_fail":false,"target_va":5370154720,"va":5385101220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d510
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6508524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e814b09cff","size":5,"soft_fail":false,"target_va":5378594064,"va":5385102583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0100
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e89cdcffff","size":5,"soft_fail":false,"target_va":5385093376,"va":5385102431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e83aedffff","size":5,"soft_fail":false,"target_va":5385096336,"va":5385101137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0f10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385102592,"raw_hex":"e8d8e6ffff","size":5,"soft_fail":false,"target_va":5385096976,"va":5385103411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=10
//     target_aliases=11 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31222333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e83d6adc01","size":5,"soft_fail":false,"target_va":5416323136,"va":5385100798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70150
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31251938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e8e2dddc01","size":5,"soft_fail":false,"target_va":5416354128,"va":5385102185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d706e0
//     owner=unresolved_namespace; count=8
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31254156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e88ce6dc01","size":5,"soft_fail":false,"target_va":5416355552,"va":5385101391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142db25b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31523802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e8da03e101","size":5,"soft_fail":false,"target_va":5416625584,"va":5385101777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::Error::invalid_length
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31828335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e86fa9e501","size":5,"soft_fail":false,"target_va":5416929920,"va":5385101580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::MapAccess::next_value<serde_json::de::MapAccess<serde_json::read::SliceRead>,serde_core::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385100672,"raw_hex":"e866d8ffff","size":5,"soft_fail":false,"target_va":5385090864,"va":5385100997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > >
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385102592,"raw_hex":"e80ce0ffff","size":5,"soft_fail":false,"target_va":5385095296,"va":5385103471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":46532,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44400,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":49020,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44224,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] theme_server_proto::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-server-proto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":905,"record_offset":44456,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":63903,"source_line":3,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] theme_server_proto::_::impl$0::deserialize::impl$2::visit_seq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-server-proto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":905,"record_offset":46348,"parent_record_offset":43992,"ancestor_record_offset":43992,"inlinee":63908,"source_line":3,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":45876,"parent_record_offset":45852,"ancestor_record_offset":43992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":45900,"parent_record_offset":45876,"ancestor_record_offset":43992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":45924,"parent_record_offset":45900,"ancestor_record_offset":43992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":45852,"parent_record_offset":45788,"ancestor_record_offset":43992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":905,"record_offset":47412,"parent_record_offset":47388,"ancestor_record_offset":43992,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":45820,"parent_record_offset":45788,"ancestor_record_offset":43992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":45788,"parent_record_offset":45760,"ancestor_record_offset":43992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":45760,"parent_record_offset":45732,"ancestor_record_offset":43992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":44752,"parent_record_offset":44652,"ancestor_record_offset":43992,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":45592,"parent_record_offset":45564,"ancestor_record_offset":43992,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":905,"record_offset":44828,"parent_record_offset":44804,"ancestor_record_offset":43992,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":46100,"parent_record_offset":46072,"ancestor_record_offset":43992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":905,"record_offset":44948,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":45564,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":905,"record_offset":45392,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":44852,"parent_record_offset":44828,"ancestor_record_offset":43992,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":44880,"parent_record_offset":44852,"ancestor_record_offset":43992,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":905,"record_offset":44804,"parent_record_offset":44780,"ancestor_record_offset":43992,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":45424,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":45496,"parent_record_offset":45472,"ancestor_record_offset":43992,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":51712,"parent_record_offset":51688,"ancestor_record_offset":49052,"inlinee":63909,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":45472,"parent_record_offset":45448,"ancestor_record_offset":43992,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":51572,"parent_record_offset":51548,"ancestor_record_offset":49052,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":46260,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":43992,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":44504,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":44976,"parent_record_offset":44456,"ancestor_record_offset":43992,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":46380,"parent_record_offset":46348,"ancestor_record_offset":43992,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":44580,"parent_record_offset":44532,"ancestor_record_offset":43992,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":45352,"parent_record_offset":45328,"ancestor_record_offset":43992,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$58::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":51688,"parent_record_offset":51664,"ancestor_record_offset":49052,"inlinee":63892,"source_line":132,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":51452,"parent_record_offset":51428,"ancestor_record_offset":49052,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44340,"parent_record_offset":44224,"ancestor_record_offset":43992,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":45052,"parent_record_offset":45012,"ancestor_record_offset":43992,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":45096,"parent_record_offset":45052,"ancestor_record_offset":43992,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44272,"parent_record_offset":44224,"ancestor_record_offset":43992,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44532,"parent_record_offset":44504,"ancestor_record_offset":43992,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":45012,"parent_record_offset":44976,"ancestor_record_offset":43992,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":44652,"parent_record_offset":44628,"ancestor_record_offset":43992,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":44628,"parent_record_offset":44604,"ancestor_record_offset":43992,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":46408,"parent_record_offset":46380,"ancestor_record_offset":43992,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=16
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":44364,"parent_record_offset":44340,"ancestor_record_offset":43992,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":44300,"parent_record_offset":44272,"ancestor_record_offset":43992,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_server_proto::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-server-proto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":905,"record_offset":44780,"parent_record_offset":44652,"ancestor_record_offset":43992,"inlinee":63907,"source_line":3,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_server_proto::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-server-proto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-server-proto/src/lib.rs
//     evidence={"module":905,"record_offset":44604,"parent_record_offset":44580,"ancestor_record_offset":43992,"inlinee":63905,"source_line":3,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: std::io::cursor::impl$5::read_to_string<ref$<bytes::bytes::Bytes> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2351,"record_offset":3528,"rva":32378800,"va":5401087920}
// 编译实例 RVAs: 0x1ee0fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String>::try_reserve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16303028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401087920,"raw_hex":"e84c3c07ff","size":5,"soft_fail":false,"target_va":5384784992,"va":5401088015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15264889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401087920,"raw_hex":"e879ece800","size":5,"soft_fail":false,"target_va":5416353040,"va":5401088146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15968983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401087920,"raw_hex":"e8d7aaf300","size":5,"soft_fail":false,"target_va":5417057056,"va":5401088068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16257898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401087920,"raw_hex":"e896ec07ff","size":5,"soft_fail":false,"target_va":5384830080,"va":5401087973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2351,"record_offset":3952,"parent_record_offset":3528,"ancestor_record_offset":3528,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2351,"record_offset":3892,"parent_record_offset":3528,"ancestor_record_offset":3528,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2351,"record_offset":3924,"parent_record_offset":3528,"ancestor_record_offset":3528,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2351,"record_offset":3692,"parent_record_offset":3528,"ancestor_record_offset":3528,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2351,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3528,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2351,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":3528,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2351,"record_offset":4172,"parent_record_offset":4140,"ancestor_record_offset":3528,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2351,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3528,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2351,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":3528,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2351,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":3528,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2351,"record_offset":4288,"parent_record_offset":4036,"ancestor_record_offset":3528,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2351,"record_offset":4080,"parent_record_offset":4036,"ancestor_record_offset":3528,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2351,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":3528,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2351,"record_offset":3748,"parent_record_offset":3692,"ancestor_record_offset":3528,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2351,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":3528,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2351,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3528,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2351,"record_offset":4252,"parent_record_offset":4036,"ancestor_record_offset":3528,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2351,"record_offset":3776,"parent_record_offset":3692,"ancestor_record_offset":3528,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2351,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3528,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2351,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":3528,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/02351.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: std::io::default_read_to_end<zip::read::ZipFile>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2354,"record_offset":1260,"rva":32384048,"va":5401093168}
// 编译实例 RVAs: 0x1ee2430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$zip..read..ZipFile$u20$as$u20$std..io..Read$GT$::read
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-68107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e8f5f5feff","size":5,"soft_fail":false,"target_va":5401025376,"va":5401093478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31754745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e807761bfe","size":5,"soft_fail":false,"target_va":5369338848,"va":5401093588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=33 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15259092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e8d4d5e800","size":5,"soft_fail":false,"target_va":5416352848,"va":5401093751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15963624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e8e895f300","size":5,"soft_fail":false,"target_va":5417057088,"va":5401093459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15528336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e890f1ec00","size":5,"soft_fail":false,"target_va":5416622208,"va":5401093867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::default_read_to_end::small_probe_read<zip::read::ZipFile>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401093168,"raw_hex":"e825feffff","size":5,"soft_fail":false,"target_va":5401092832,"va":5401093302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2354,"record_offset":1440,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2354,"record_offset":1524,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2354,"record_offset":2752,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2354,"record_offset":2064,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2354,"record_offset":2908,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":15830,"source_line":1512,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2354,"record_offset":2124,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2354,"record_offset":2788,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":10914,"source_line":2389,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2354,"record_offset":1552,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2354,"record_offset":1704,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2354,"record_offset":1736,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2354,"record_offset":1828,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":2354,"record_offset":2180,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":23719,"source_line":1057,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":2354,"record_offset":2528,"parent_record_offset":1260,"ancestor_record_offset":1260,"inlinee":8738,"source_line":1015,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":1464,"parent_record_offset":1440,"ancestor_record_offset":1260,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":2936,"parent_record_offset":2908,"ancestor_record_offset":1260,"inlinee":15831,"source_line":344,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":1260,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::grow_amortized
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":2992,"parent_record_offset":2964,"ancestor_record_offset":1260,"inlinee":13392,"source_line":501,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::set_ptr_and_cap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":3156,"parent_record_offset":2992,"ancestor_record_offset":1260,"inlinee":13393,"source_line":770,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2354,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":1260,"inlinee":13402,"source_line":682,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2354,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":1260,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2354,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1260,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2354,"record_offset":3068,"parent_record_offset":2992,"ancestor_record_offset":1260,"inlinee":10723,"source_line":1681,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2354,"record_offset":1636,"parent_record_offset":1604,"ancestor_record_offset":1260,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedCursor::advance
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\io\borrowed_buf.rs
//     evidence={"module":2354,"record_offset":2488,"parent_record_offset":2212,"ancestor_record_offset":1260,"inlinee":24831,"source_line":275,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedCursor::ensure_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\io\borrowed_buf.rs
//     evidence={"module":2354,"record_offset":2248,"parent_record_offset":2212,"ancestor_record_offset":1260,"inlinee":23721,"source_line":302,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2354,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":1260,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2354,"record_offset":2816,"parent_record_offset":2788,"ancestor_record_offset":1260,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_rem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2354,"record_offset":1664,"parent_record_offset":1604,"ancestor_record_offset":1260,"inlinee":59309,"source_line":1449,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2354,"record_offset":2844,"parent_record_offset":2816,"ancestor_record_offset":1260,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2354,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1260,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2354,"record_offset":2392,"parent_record_offset":2248,"ancestor_record_offset":1260,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2354,"record_offset":2456,"parent_record_offset":2212,"ancestor_record_offset":1260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2354,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":1260,"inlinee":4871,"source_line":684,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2354,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":1260,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2354,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1260,"inlinee":11640,"source_line":417,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2354,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1260,"inlinee":15853,"source_line":561,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_buf::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":2354,"record_offset":2424,"parent_record_offset":2212,"ancestor_record_offset":1260,"inlinee":23723,"source_line":1058,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::default_read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":2354,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":1260,"inlinee":23720,"source_line":586,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::default_read_to_end::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":2354,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":1260,"inlinee":59308,"source_line":419,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":2354,"record_offset":2664,"parent_record_offset":2528,"ancestor_record_offset":1260,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2354,"record_offset":2568,"parent_record_offset":2528,"ancestor_record_offset":1260,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2354,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":1260,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2354,"record_offset":2628,"parent_record_offset":2596,"ancestor_record_offset":1260,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/02354.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":893,"record_offset":1692,"rva":16177056,"va":5384886176}
// 编译实例 RVAs: 0xf6d7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-227081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8f788fcff","size":5,"soft_fail":false,"target_va":5384659264,"va":5384886340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-184045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81331fdff","size":5,"soft_fail":false,"target_va":5384703536,"va":5384887576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-230005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88b7dfcff","size":5,"soft_fail":false,"target_va":5384656240,"va":5384886240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t226191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88f730300","size":5,"soft_fail":false,"target_va":5385113168,"va":5384886972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11654681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e819d6b100","size":5,"soft_fail":false,"target_va":5396542816,"va":5384888130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=29
//     evidence={"assembly":"\tcallq\t-9207596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8d48073ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5384886295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15657856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8801411ff","size":5,"soft_fail":false,"target_va":5369231648,"va":5384889499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15549265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8afbc12ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5384888108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15545905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8cfc912ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5384889404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972470
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6273944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e86844a0ff","size":5,"soft_fail":false,"target_va":5378614384,"va":5384888323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9fee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t206362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81a260300","size":5,"soft_fail":false,"target_va":5385092832,"va":5384886465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ed24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16140163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88347f600","size":5,"soft_fail":false,"target_va":5401027760,"va":5384887592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32170009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e819e0ea01","size":5,"soft_fail":false,"target_va":5417057040,"va":5384887026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-93663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e82192feff","size":5,"soft_fail":false,"target_va":5384794864,"va":5384888522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6c4ab46d71a042a1 (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31737111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81745e401","size":5,"soft_fail":false,"target_va":5416623536,"va":5384886420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hb429360875c1aa7a (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31737548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8cc46e401","size":5,"soft_fail":false,"target_va":5416624144,"va":5384886591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31735939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88340e401","size":5,"soft_fail":false,"target_va":5416623024,"va":5384887080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31736557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8ed42e401","size":5,"soft_fail":false,"target_va":5416625264,"va":5384888702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t234731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8eb940300","size":5,"soft_fail":false,"target_va":5385121616,"va":5384886880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t232852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8948d0300","size":5,"soft_fail":false,"target_va":5385119680,"va":5384886823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t228908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e82c7e0300","size":5,"soft_fail":false,"target_va":5385116768,"va":5384887855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<memmap2::Mmap,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e808010000","size":5,"soft_fail":false,"target_va":5384890208,"va":5384889939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31732566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e85633e401","size":5,"soft_fail":false,"target_va":5416621632,"va":5384889061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11655278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e86ed8b100","size":5,"soft_fail":false,"target_va":5396542944,"va":5384887661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2764047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8f1d2d5ff","size":5,"soft_fail":false,"target_va":5382123584,"va":5384887626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-134336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e840f3fdff","size":5,"soft_fail":false,"target_va":5384751936,"va":5384886267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rdi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384886176,"raw_hex":"ffd7","size":2,"soft_fail":false,"va":5384887712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":893,"record_offset":9440,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3220,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":893,"record_offset":3016,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":5952,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":893,"record_offset":4132,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":893,"record_offset":2676,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2924,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":893,"record_offset":3692,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":893,"record_offset":9524,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":893,"record_offset":6444,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":3848,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":8588,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8050,"source_line":768,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6640,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":9780,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":893,"record_offset":3128,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6228,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":15362,"source_line":579,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6112,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":1928,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":893,"record_offset":1900,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":1692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2160,"parent_record_offset":2108,"ancestor_record_offset":1692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":893,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":1692,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2084,"parent_record_offset":2064,"ancestor_record_offset":1692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3308,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3256,"parent_record_offset":3220,"ancestor_record_offset":1692,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":1692,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":893,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":1692,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10652,"parent_record_offset":10624,"ancestor_record_offset":1692,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10624,"parent_record_offset":10600,"ancestor_record_offset":1692,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":1692,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":1692,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":893,"record_offset":3748,"parent_record_offset":3720,"ancestor_record_offset":1692,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":893,"record_offset":2764,"parent_record_offset":2716,"ancestor_record_offset":1692,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":893,"record_offset":2716,"parent_record_offset":2676,"ancestor_record_offset":1692,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":1692,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":1692,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12264,"parent_record_offset":12236,"ancestor_record_offset":1692,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":893,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":1692,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":893,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":1692,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":1692,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":1692,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":893,"record_offset":3336,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":893,"record_offset":10708,"parent_record_offset":10680,"ancestor_record_offset":1692,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":893,"record_offset":10680,"parent_record_offset":10652,"ancestor_record_offset":1692,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":893,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":1692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":893,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":1692,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":893,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":1692,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":893,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":1692,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=87
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":893,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":1692,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":893,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":1692,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":3188,"parent_record_offset":3128,"ancestor_record_offset":1692,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":1692,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":6052,"parent_record_offset":5952,"ancestor_record_offset":1692,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":1692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":1692,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":3364,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6400,"parent_record_offset":6256,"ancestor_record_offset":1692,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":1692,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":893,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":1692,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":893,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":1692,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":893,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1692,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":893,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1692,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6288,"parent_record_offset":6256,"ancestor_record_offset":1692,"inlinee":11999,"source_line":228,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":1692,"inlinee":12011,"source_line":338,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":1692,"inlinee":12001,"source_line":229,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":1692,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6196,"parent_record_offset":6112,"ancestor_record_offset":1692,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":893,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":1692,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":893,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":1692,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zip::read::find_content<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2352,"record_offset":268,"rva":32379264,"va":5401088384}
// 编译实例 RVAs: 0x1ee1180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b98a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7502258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5401088384,"raw_hex":"e84e868dff","size":5,"soft_fail":false,"target_va":5393586336,"va":5401088589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u16
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2352,"record_offset":972,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":45571,"source_line":112,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2352,"record_offset":444,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2352,"record_offset":804,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2352,"record_offset":832,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2352,"record_offset":564,"parent_record_offset":500,"ancestor_record_offset":268,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":2352,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":268,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2352,"record_offset":936,"parent_record_offset":856,"ancestor_record_offset":268,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2352,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":268,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2352,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":268,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2352,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":268,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2352,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2352,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":268,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2352,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":268,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2352,"record_offset":592,"parent_record_offset":500,"ancestor_record_offset":268,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2352,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":268,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2352,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":268,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2352,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":2352,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":268,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":2352,"record_offset":680,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/02352.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":909,"record_offset":268,"rva":16412688,"va":5385121808}
// 编译实例 RVAs: 0xfa7010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-327256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8a801fbff","size":5,"soft_fail":false,"target_va":5384795984,"va":5385123235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-477981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8e3b4f8ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5385122888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9444151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8c9e46fff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385122834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-9443715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e87de66fff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385122414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16400628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e80cbf05ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385122815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15783839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e861280fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385122682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a770
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15782238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8a22e0fff","size":5,"soft_fail":false,"target_va":5369341808,"va":5385124041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15682614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8cab310ff","size":5,"soft_fail":false,"target_va":5369441232,"va":5385123841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6466277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e81b559dff","size":5,"soft_fail":false,"target_va":5378657872,"va":5385124144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ee12b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15965548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e86c9df300","size":5,"soft_fail":false,"target_va":5401088688,"va":5385123135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31933971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e81346e701","size":5,"soft_fail":false,"target_va":5417057056,"va":5385123080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31496855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8979ae001","size":5,"soft_fail":false,"target_va":5416620769,"va":5385123909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31278171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e85b44dd01","size":5,"soft_fail":false,"target_va":5416401488,"va":5385123312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31497993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8099fe001","size":5,"soft_fail":false,"target_va":5416620794,"va":5385122796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..zip_archive..Shared$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-242984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8d84afcff","size":5,"soft_fail":false,"target_va":5384880992,"va":5385123971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15968829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e83daaf300","size":5,"soft_fail":false,"target_va":5401092096,"va":5385123262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::with_capacity_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15904468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e8d4aef200","size":5,"soft_fail":false,"target_va":5401027408,"va":5385122935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-397182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e882f0f9ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5385123865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15963686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e82696f300","size":5,"soft_fail":false,"target_va":5401085568,"va":5385121877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::spec::Zip64CentralDirectoryEnd::find_and_parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15964050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385121808,"raw_hex":"e89297f300","size":5,"soft_fail":false,"target_va":5401086576,"va":5385122521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":909,"record_offset":4060,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":909,"record_offset":4024,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":909,"record_offset":3748,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":909,"record_offset":2564,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":909,"record_offset":3896,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":909,"record_offset":2280,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":909,"record_offset":492,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":909,"record_offset":524,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":909,"record_offset":2984,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":15377,"source_line":290,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] zip::read::unsupported_zip_error
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":909,"record_offset":2252,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":64039,"source_line":641,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::get_directory_counts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":909,"record_offset":584,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":64034,"source_line":297,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_direct] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":909,"record_offset":552,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":64033,"source_line":29,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":268,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":268,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":268,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2852,"parent_record_offset":2668,"ancestor_record_offset":268,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":909,"record_offset":2432,"parent_record_offset":2376,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":909,"record_offset":4132,"parent_record_offset":4092,"ancestor_record_offset":268,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":909,"record_offset":4092,"parent_record_offset":4060,"ancestor_record_offset":268,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":268,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2404,"parent_record_offset":2376,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2668,"parent_record_offset":2636,"ancestor_record_offset":268,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":268,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":909,"record_offset":2708,"parent_record_offset":2668,"ancestor_record_offset":268,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":909,"record_offset":3784,"parent_record_offset":3748,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":909,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":268,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":909,"record_offset":888,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":64036,"source_line":217,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":909,"record_offset":1480,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":64037,"source_line":321,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":909,"record_offset":2732,"parent_record_offset":2708,"ancestor_record_offset":268,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":909,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":268,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":909,"record_offset":3444,"parent_record_offset":3240,"ancestor_record_offset":268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":909,"record_offset":1004,"parent_record_offset":948,"ancestor_record_offset":268,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":909,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":268,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":909,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":268,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":909,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":268,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":909,"record_offset":808,"parent_record_offset":720,"ancestor_record_offset":268,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":909,"record_offset":3924,"parent_record_offset":3896,"ancestor_record_offset":268,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":909,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":268,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":268,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":2024,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":5654,"source_line":761,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":268,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":1952,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":8773,"source_line":917,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":776,"parent_record_offset":748,"ancestor_record_offset":268,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":268,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":909,"record_offset":3572,"parent_record_offset":3516,"ancestor_record_offset":268,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":909,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":909,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":909,"record_offset":2136,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":909,"record_offset":3272,"parent_record_offset":3240,"ancestor_record_offset":268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":909,"record_offset":4676,"parent_record_offset":4592,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":909,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":909,"record_offset":2080,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":909,"record_offset":3832,"parent_record_offset":3784,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":909,"record_offset":5132,"parent_record_offset":5108,"ancestor_record_offset":268,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":909,"record_offset":3360,"parent_record_offset":3240,"ancestor_record_offset":268,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":909,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":909,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":268,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":909,"record_offset":3860,"parent_record_offset":3784,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":909,"record_offset":1988,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":909,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":268,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":909,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":268,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":909,"record_offset":1032,"parent_record_offset":948,"ancestor_record_offset":268,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":909,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":268,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":909,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":268,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":909,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":909,"record_offset":4840,"parent_record_offset":4808,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":909,"record_offset":4780,"parent_record_offset":4740,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":909,"record_offset":4620,"parent_record_offset":4592,"ancestor_record_offset":268,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_capacity_and_hasher
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":909,"record_offset":3680,"parent_record_offset":3016,"ancestor_record_offset":268,"inlinee":13442,"source_line":499,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":5108,"parent_record_offset":4504,"ancestor_record_offset":268,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4960,"parent_record_offset":4740,"ancestor_record_offset":268,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4592,"parent_record_offset":4536,"ancestor_record_offset":268,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":268,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":3708,"parent_record_offset":3680,"ancestor_record_offset":268,"inlinee":5350,"source_line":643,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":5512,"parent_record_offset":5480,"ancestor_record_offset":268,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4564,"parent_record_offset":4536,"ancestor_record_offset":268,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4504,"parent_record_offset":4444,"ancestor_record_offset":268,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":268,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":5480,"parent_record_offset":4444,"ancestor_record_offset":268,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4476,"parent_record_offset":4444,"ancestor_record_offset":268,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4536,"parent_record_offset":4504,"ancestor_record_offset":268,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":5544,"parent_record_offset":5512,"ancestor_record_offset":268,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":268,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":909,"record_offset":4712,"parent_record_offset":4504,"ancestor_record_offset":268,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":909,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":268,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":909,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":268,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":909,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":268,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":909,"record_offset":3516,"parent_record_offset":3116,"ancestor_record_offset":268,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":909,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":268,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":909,"record_offset":948,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":23683,"source_line":233,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":909,"record_offset":692,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":909,"record_offset":916,"parent_record_offset":888,"ancestor_record_offset":268,"inlinee":23821,"source_line":360,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":909,"record_offset":1120,"parent_record_offset":916,"ancestor_record_offset":268,"inlinee":23684,"source_line":347,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":909,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":909,"record_offset":3240,"parent_record_offset":3208,"ancestor_record_offset":268,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":909,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":909,"record_offset":3088,"parent_record_offset":3060,"ancestor_record_offset":268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] zip::read::impl$4::get_directory_counts::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":909,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":268,"inlinee":64041,"source_line":337,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::CentralDirectoryEnd::record_too_small
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":909,"record_offset":1916,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":64033,"source_line":29,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   [pdb_inline_transitive] zip::spec::Zip64CentralDirectoryEndLocator::parse
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/spec.rs
//     evidence={"module":909,"record_offset":844,"parent_record_offset":584,"ancestor_record_offset":268,"inlinee":64035,"source_line":118,"archive":"_pdb/C/modules/00909.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":47900,"rva":4172992,"va":5372882112}
// 编译实例 RVAs: 0x3facc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11807393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e8a12ab400","size":5,"soft_fail":false,"target_va":5384689616,"va":5372882218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2796078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e82eaa2a00","size":5,"soft_fail":false,"target_va":5375678688,"va":5372882605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2796418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e882ab2a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372882281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4160403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e86d84c0ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5372882590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3642289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e84f6cc8ff","size":5,"soft_fail":false,"target_va":5369240768,"va":5372883052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3636372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e86c83c8ff","size":5,"soft_fail":false,"target_va":5369246768,"va":5372883135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089af0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3609226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e876edc8ff","size":5,"soft_fail":false,"target_va":5369273072,"va":5372882293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3544721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e86fe9c9ff","size":5,"soft_fail":false,"target_va":5369338576,"va":5372883292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407bdf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3944885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e8b5313c00","size":5,"soft_fail":false,"target_va":5376827200,"va":5372882310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43439612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e8fcd59602","size":5,"soft_fail":false,"target_va":5416321856,"va":5372882239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44174694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e8660da202","size":5,"soft_fail":false,"target_va":5417057056,"va":5372882357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43737952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e860639b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5372882812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3808054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e8361b3a00","size":5,"soft_fail":false,"target_va":5376691136,"va":5372883077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43738761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e889669b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5372882866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::ClientBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3942278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372882112,"raw_hex":"e886273c00","size":5,"soft_fail":false,"target_va":5376824736,"va":5372882453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":122,"record_offset":48732,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":13049,"source_line":691,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":122,"record_offset":48064,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":48232,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":48688,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":48204,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::ClientBuilder::pool_max_idle_per_host
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":122,"record_offset":48632,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":13046,"source_line":405,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::ClientBuilder::timeout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":122,"record_offset":48600,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":13045,"source_line":357,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":48120,"parent_record_offset":47900,"ancestor_record_offset":47900,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48904,"parent_record_offset":48880,"ancestor_record_offset":47900,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48928,"parent_record_offset":48904,"ancestor_record_offset":47900,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48480,"parent_record_offset":48456,"ancestor_record_offset":47900,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48504,"parent_record_offset":48480,"ancestor_record_offset":47900,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":47900,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48528,"parent_record_offset":48504,"ancestor_record_offset":47900,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48880,"parent_record_offset":48848,"ancestor_record_offset":47900,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":48456,"parent_record_offset":48400,"ancestor_record_offset":47900,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":122,"record_offset":48848,"parent_record_offset":48804,"ancestor_record_offset":47900,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":122,"record_offset":48804,"parent_record_offset":48768,"ancestor_record_offset":47900,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":48428,"parent_record_offset":48400,"ancestor_record_offset":47900,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":48400,"parent_record_offset":48376,"ancestor_record_offset":47900,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":48376,"parent_record_offset":48352,"ancestor_record_offset":47900,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":122,"record_offset":49488,"parent_record_offset":49452,"ancestor_record_offset":47900,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":122,"record_offset":48768,"parent_record_offset":48732,"ancestor_record_offset":47900,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":122,"record_offset":49452,"parent_record_offset":49428,"ancestor_record_offset":47900,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$4::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":122,"record_offset":48088,"parent_record_offset":48064,"ancestor_record_offset":47900,"inlinee":7328,"source_line":788,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":48256,"parent_record_offset":48232,"ancestor_record_offset":47900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":122,"record_offset":49520,"parent_record_offset":49488,"ancestor_record_offset":47900,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":49548,"parent_record_offset":49488,"ancestor_record_offset":47900,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":49580,"parent_record_offset":49452,"ancestor_record_offset":47900,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::ClientBuilder::with_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":122,"record_offset":48656,"parent_record_offset":48632,"ancestor_record_offset":47900,"inlinee":13047,"source_line":839,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":48172,"parent_record_offset":48120,"ancestor_record_offset":47900,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":48144,"parent_record_offset":48120,"ancestor_record_offset":47900,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::fetch_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46C9
// 首条位置: {"module":258,"inlinee":18121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":17088,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":13616,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5433,"source_line":1880,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16408,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18668,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18936,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":18154,"source_line":461,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":14560,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":17452,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":258,"record_offset":17172,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":18584,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":17480,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":15232,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":13644,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":13680,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":18500,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":13956,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":18125,"source_line":676,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] http::header::value::HeaderValue::to_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/value.rs
//     evidence={"module":258,"record_offset":14348,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":18130,"source_line":251,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":14616,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":14476,"parent_record_offset":13272,"ancestor_record_offset":13272,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":13272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":13272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":13272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":17680,"parent_record_offset":17656,"ancestor_record_offset":13272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":13272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":13272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13828,"parent_record_offset":13796,"ancestor_record_offset":13272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":17632,"parent_record_offset":17576,"ancestor_record_offset":13272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":13796,"parent_record_offset":13752,"ancestor_record_offset":13272,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$15::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":258,"record_offset":13716,"parent_record_offset":13680,"ancestor_record_offset":13272,"inlinee":18123,"source_line":518,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":13752,"parent_record_offset":13716,"ancestor_record_offset":13272,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":13272,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16668,"parent_record_offset":16640,"ancestor_record_offset":13272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":13272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":17576,"parent_record_offset":17552,"ancestor_record_offset":13272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":13272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":13272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":13272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":13272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":17552,"parent_record_offset":17528,"ancestor_record_offset":13272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":13272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16548,"parent_record_offset":16520,"ancestor_record_offset":13272,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16576,"parent_record_offset":16548,"ancestor_record_offset":13272,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16604,"parent_record_offset":16576,"ancestor_record_offset":13272,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16520,"parent_record_offset":16492,"ancestor_record_offset":13272,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":258,"record_offset":16492,"parent_record_offset":16464,"ancestor_record_offset":13272,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":13272,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16464,"parent_record_offset":16436,"ancestor_record_offset":13272,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":19128,"parent_record_offset":19104,"ancestor_record_offset":13272,"inlinee":4851,"source_line":1644,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":19152,"parent_record_offset":19128,"ancestor_record_offset":13272,"inlinee":4852,"source_line":1888,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":13272,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14180,"parent_record_offset":14152,"ancestor_record_offset":13272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14152,"parent_record_offset":14100,"ancestor_record_offset":13272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":16640,"parent_record_offset":16604,"ancestor_record_offset":13272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14072,"parent_record_offset":13984,"ancestor_record_offset":13272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":13272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":19792,"parent_record_offset":19768,"ancestor_record_offset":13272,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14100,"parent_record_offset":14072,"ancestor_record_offset":13272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":13272,"inlinee":18155,"source_line":466,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":13272,"inlinee":4846,"source_line":81,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":19104,"parent_record_offset":18960,"ancestor_record_offset":13272,"inlinee":4850,"source_line":121,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":19048,"parent_record_offset":18960,"ancestor_record_offset":13272,"inlinee":4844,"source_line":196,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":13272,"inlinee":18159,"source_line":546,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":13272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":258,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":13272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":258,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":13272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":19248,"parent_record_offset":18936,"ancestor_record_offset":13272,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":13272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":20292,"parent_record_offset":20264,"ancestor_record_offset":13272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":13272,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":13272,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":13272,"inlinee":4853,"source_line":993,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":17000,"parent_record_offset":16604,"ancestor_record_offset":13272,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":13272,"inlinee":4854,"source_line":935,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":13272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":13272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":258,"record_offset":18548,"parent_record_offset":18524,"ancestor_record_offset":13272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":14408,"parent_record_offset":14380,"ancestor_record_offset":13272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13272,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14296,"parent_record_offset":14100,"ancestor_record_offset":13272,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":258,"record_offset":18524,"parent_record_offset":18500,"ancestor_record_offset":13272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":258,"record_offset":14380,"parent_record_offset":14348,"ancestor_record_offset":13272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18992,"parent_record_offset":18960,"ancestor_record_offset":13272,"inlinee":6416,"source_line":1764,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19332,"parent_record_offset":19296,"ancestor_record_offset":13272,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":13272,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":13272,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19356,"parent_record_offset":19332,"ancestor_record_offset":13272,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":13272,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19388,"parent_record_offset":19296,"ancestor_record_offset":13272,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":13984,"parent_record_offset":13956,"ancestor_record_offset":13272,"inlinee":18126,"source_line":683,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14016,"parent_record_offset":13984,"ancestor_record_offset":13272,"inlinee":18128,"source_line":3422,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::value::is_visible_ascii
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/value.rs
//     evidence={"module":258,"record_offset":14444,"parent_record_offset":14348,"ancestor_record_offset":13272,"inlinee":18132,"source_line":583,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":13272,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":13272,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":13272,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::fetch_events::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1983E
// 首条位置: {"module":2359,"inlinee":104510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arc_swap::ArcSwapAny::from_pointee
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":2359,"record_offset":1296,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":13049,"source_line":691,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2359,"record_offset":1240,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2359,"record_offset":1556,"parent_record_offset":1180,"ancestor_record_offset":1180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":1180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2359,"record_offset":1708,"parent_record_offset":1652,"ancestor_record_offset":1180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2359,"record_offset":1392,"parent_record_offset":1352,"ancestor_record_offset":1180,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2359,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1180,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2359,"record_offset":1680,"parent_record_offset":1652,"ancestor_record_offset":1180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2359,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2359,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2359,"record_offset":1324,"parent_record_offset":1296,"ancestor_record_offset":1180,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2359,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02359.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::download_blob
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46D8
// 首条位置: {"module":258,"inlinee":18136}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":26312,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":29260,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$93::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":29760,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":18157,"source_line":2670,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":258,"record_offset":30232,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":25976,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":258,"record_offset":26396,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":27920,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":27236,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":27272,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":26032,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":25892,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26424,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":18138,"source_line":892,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":25864,"parent_record_offset":25620,"ancestor_record_offset":25620,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27532,"parent_record_offset":27508,"ancestor_record_offset":25620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":25620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":26872,"parent_record_offset":26844,"ancestor_record_offset":25620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":26900,"parent_record_offset":26872,"ancestor_record_offset":25620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27580,"parent_record_offset":27556,"ancestor_record_offset":25620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":26928,"parent_record_offset":26900,"ancestor_record_offset":25620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":27508,"parent_record_offset":27476,"ancestor_record_offset":25620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":26844,"parent_record_offset":26784,"ancestor_record_offset":25620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":27476,"parent_record_offset":27432,"ancestor_record_offset":25620,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":27432,"parent_record_offset":27396,"ancestor_record_offset":25620,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":27728,"parent_record_offset":27704,"ancestor_record_offset":25620,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":25620,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":25620,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":26784,"parent_record_offset":26756,"ancestor_record_offset":25620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":26756,"parent_record_offset":26732,"ancestor_record_offset":25620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":29348,"parent_record_offset":29296,"ancestor_record_offset":25620,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":29296,"parent_record_offset":29260,"ancestor_record_offset":25620,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":27092,"parent_record_offset":27060,"ancestor_record_offset":25620,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":27060,"parent_record_offset":27036,"ancestor_record_offset":25620,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27396,"parent_record_offset":27360,"ancestor_record_offset":25620,"inlinee":6353,"source_line":319,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27360,"parent_record_offset":27308,"ancestor_record_offset":25620,"inlinee":6291,"source_line":151,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27308,"parent_record_offset":27272,"ancestor_record_offset":25620,"inlinee":6355,"source_line":739,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":30284,"parent_record_offset":30260,"ancestor_record_offset":25620,"inlinee":18159,"source_line":546,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":29788,"parent_record_offset":29760,"ancestor_record_offset":25620,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":25620,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":29376,"parent_record_offset":29296,"ancestor_record_offset":25620,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":26000,"parent_record_offset":25976,"ancestor_record_offset":25620,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":26336,"parent_record_offset":26312,"ancestor_record_offset":25620,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":26532,"parent_record_offset":26492,"ancestor_record_offset":25620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":27128,"parent_record_offset":27092,"ancestor_record_offset":25620,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":29404,"parent_record_offset":29296,"ancestor_record_offset":25620,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":26492,"parent_record_offset":26456,"ancestor_record_offset":25620,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":25620,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":25620,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":29816,"parent_record_offset":29788,"ancestor_record_offset":25620,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":25916,"parent_record_offset":25892,"ancestor_record_offset":25620,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27156,"parent_record_offset":27092,"ancestor_record_offset":25620,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":25940,"parent_record_offset":25916,"ancestor_record_offset":25620,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":26080,"parent_record_offset":26056,"ancestor_record_offset":25620,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":26056,"parent_record_offset":26032,"ancestor_record_offset":25620,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":26104,"parent_record_offset":26080,"ancestor_record_offset":25620,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":258,"record_offset":26624,"parent_record_offset":26588,"ancestor_record_offset":25620,"inlinee":18145,"source_line":43,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26456,"parent_record_offset":26424,"ancestor_record_offset":25620,"inlinee":18139,"source_line":949,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27012,"parent_record_offset":26456,"ancestor_record_offset":25620,"inlinee":18147,"source_line":49,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27036,"parent_record_offset":27012,"ancestor_record_offset":25620,"inlinee":18149,"source_line":987,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$14::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27192,"parent_record_offset":27036,"ancestor_record_offset":25620,"inlinee":18151,"source_line":1157,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26560,"parent_record_offset":26492,"ancestor_record_offset":25620,"inlinee":18141,"source_line":950,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":258,"record_offset":26588,"parent_record_offset":26560,"ancestor_record_offset":25620,"inlinee":18143,"source_line":37,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::trim_cas
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46BE
// 首条位置: {"module":258,"inlinee":18110}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":258,"record_offset":2796,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":2524,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":2908,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":2580,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2440,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":2692,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":2936,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":18111,"source_line":226,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2284,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":2284,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":2284,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":2284,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2284,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2284,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2284,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":10652,"parent_record_offset":10616,"ancestor_record_offset":2284,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":10616,"parent_record_offset":10592,"ancestor_record_offset":2284,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":2868,"parent_record_offset":2828,"ancestor_record_offset":2284,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":258,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":2284,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":3312,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":5140,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":2724,"parent_record_offset":2692,"ancestor_record_offset":2284,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":2284,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2284,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=103
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":3436,"parent_record_offset":3400,"ancestor_record_offset":2284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":10684,"parent_record_offset":10652,"ancestor_record_offset":2284,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":2752,"parent_record_offset":2692,"ancestor_record_offset":2284,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":4200,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":2284,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2284,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":10712,"parent_record_offset":10652,"ancestor_record_offset":2284,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":10744,"parent_record_offset":10616,"ancestor_record_offset":2284,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":258,"record_offset":3368,"parent_record_offset":3312,"ancestor_record_offset":2284,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2284,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":5196,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2284,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2284,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":5056,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3076,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2284,"inlinee":18118,"source_line":1955,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3196,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":18117,"source_line":1978,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3400,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":4228,"parent_record_offset":2936,"ancestor_record_offset":2284,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":2284,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileAttr::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2284,"inlinee":18120,"source_line":1088,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2284,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::impl$24::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2284,"inlinee":18115,"source_line":1133,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46BF
// 首条位置: {"module":258,"inlinee":18111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":3312,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":5140,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":5308,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":4200,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":5196,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":5056,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3076,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3196,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":18117,"source_line":1978,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3400,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":4228,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2936,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":2936,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":2936,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":2936,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2936,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2936,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2936,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":2936,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2936,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=75
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":3436,"parent_record_offset":3400,"ancestor_record_offset":2936,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":2936,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":2936,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":258,"record_offset":3368,"parent_record_offset":3312,"ancestor_record_offset":2936,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":2936,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":2936,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":2936,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2936,"inlinee":18118,"source_line":1955,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":2936,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileAttr::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2936,"inlinee":18120,"source_line":1088,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2936,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::impl$24::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2936,"inlinee":18115,"source_line":1133,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::ThemeFetcher::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 261
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":258,"record_offset":2080,"rva":6969728,"va":5375678848}
// 编译实例 RVAs: 0x6a5980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::boxed::Box<dyn core::error::Error> as core::convert::From<&str>>::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9111836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e81c098b00","size":5,"soft_fail":false,"target_va":5384797008,"va":5375685167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::boxed::Box<dyn core::error::Error> as core::convert::From<alloc::string::String>>::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9112104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8280a8b00","size":5,"soft_fail":false,"target_va":5384796832,"va":5375684723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t9105085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8bdee8a00","size":5,"soft_fail":false,"target_va":5384795984,"va":5375690894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8980070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e866068900","size":5,"soft_fail":false,"target_va":5384659264,"va":5375679189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::file_name
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9009426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e812798900","size":5,"soft_fail":false,"target_va":5384689584,"va":5375680153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9009471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e83f798900","size":5,"soft_fail":false,"target_va":5384689312,"va":5375679836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9088683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8abae8a00","size":5,"soft_fail":false,"target_va":5384768304,"va":5375679616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8969859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e883de8800","size":5,"soft_fail":false,"target_va":5384656240,"va":5375686376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::components
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8961275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8fbbc8800","size":5,"soft_fail":false,"target_va":5384653152,"va":5375691872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_dir
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8969982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8fede8800","size":5,"soft_fail":false,"target_va":5384656400,"va":5375686413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9082938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e83a988a00","size":5,"soft_fail":false,"target_va":5384764464,"va":5375681521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9084069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8a59c8a00","size":5,"soft_fail":false,"target_va":5384768864,"va":5375684790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8966291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e893d08800","size":5,"soft_fail":false,"target_va":5384648032,"va":5375681736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::duration_since
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8981356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e86c0b8900","size":5,"soft_fail":false,"target_va":5384661120,"va":5375679759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::now
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8982089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8490e8900","size":5,"soft_fail":false,"target_va":5384661200,"va":5375679106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$reqwest..into_url..IntoUrlSealed$GT$::into_url
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t955207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e847930e00","size":5,"soft_fail":false,"target_va":5376642352,"va":5375687140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9438380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ac049000","size":5,"soft_fail":false,"target_va":5385131264,"va":5375692879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..debt..list..LocalNode$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8370524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e85cb97f00","size":5,"soft_fail":false,"target_va":5384056768,"va":5375686239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$arc_swap..strategy..hybrid..HybridStrategy$LT$Cfg$GT$$u20$as$u20$arc_swap..strategy..sealed..InnerStrategy$LT$T$GT$$GT$::load::_$u7b$$u7b$closure$u7d$$u7d$::h43b9f09ab5499ca2 (.llvm.1235898149996101210)
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t9419228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8dcb98f00","size":5,"soft_fail":false,"target_va":5385105376,"va":5375686143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9423590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8e6ca8f00","size":5,"soft_fail":false,"target_va":5385113168,"va":5375689573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t8930303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ff438800","size":5,"soft_fail":false,"target_va":5384609408,"va":5375679100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-3488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e860f2ffff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375682171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=41
//     evidence={"assembly":"\tcallq\t-1198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e852fbffff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375679897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6959969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e89fcc95ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375682156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6449391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e811979dff","size":5,"soft_fail":false,"target_va":5369248944,"va":5375698330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6340761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8673f9fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375679604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6353342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8420e9fff","size":5,"soft_fail":false,"target_va":5369343504,"va":5375696841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140147bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5648779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e875cea9ff","size":5,"soft_fail":false,"target_va":5370051568,"va":5375700342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140147c30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5648606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e822cfa9ff","size":5,"soft_fail":false,"target_va":5370051632,"va":5375700233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401581a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5576190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e802eaaaff","size":5,"soft_fail":false,"target_va":5370118560,"va":5375694745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14078d660
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t930843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e81b340e00","size":5,"soft_fail":false,"target_va":5376628320,"va":5375697472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14078d6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t929507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8e32e0e00","size":5,"soft_fail":false,"target_va":5376628432,"va":5375698920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d510
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2897481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e849362c00","size":5,"soft_fail":false,"target_va":5378594064,"va":5375696578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9fd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9407292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e83c8b8f00","size":5,"soft_fail":false,"target_va":5385092352,"va":5375685055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa7010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9431785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8e9ea8f00","size":5,"soft_fail":false,"target_va":5385121808,"va":5375690018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa7ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9434873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8f9f68f00","size":5,"soft_fail":false,"target_va":5385125024,"va":5375690146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41367408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e870377702","size":5,"soft_fail":false,"target_va":5417057040,"va":5375689627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41377501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8dd5e7702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375679550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t40926657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8c17d7002","size":5,"soft_fail":false,"target_va":5416620769,"va":5375694107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9110424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e898038b00","size":5,"soft_fail":false,"target_va":5384794864,"va":5375684435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40927130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e89a7f7002","size":5,"soft_fail":false,"target_va":5416620794,"va":5375693659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t1160111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8afb31100","size":5,"soft_fail":false,"target_va":5376841312,"va":5375681196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6c4ab46d71a042a1 (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40944238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e86ec27002","size":5,"soft_fail":false,"target_va":5416623536,"va":5375679293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40933547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ab987002","size":5,"soft_fail":false,"target_va":5416622848,"va":5375689296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40945345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8c1c67002","size":5,"soft_fail":false,"target_va":5416625264,"va":5375679914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8935489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e841588800","size":5,"soft_fail":false,"target_va":5384624064,"va":5375688570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8944556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ac7b8800","size":5,"soft_fail":false,"target_va":5384624080,"va":5375679519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9421646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e84ec38f00","size":5,"soft_fail":false,"target_va":5385107584,"va":5375685933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::Node::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8370815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e87fba7f00","size":5,"soft_fail":false,"target_va":5384056944,"va":5375686124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::THREAD_HEAD::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8370808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e878ba7f00","size":5,"soft_fail":false,"target_va":5384056912,"va":5375686099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9427274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e84ad98f00","size":5,"soft_fail":false,"target_va":5385116768,"va":5375689489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] cache_control::CacheControl::from_value
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"cache_control","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9330346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8aa5e8e00","size":5,"soft_fail":false,"target_va":5385013920,"va":5375683569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9438257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e831049000","size":5,"soft_fail":false,"target_va":5385130368,"va":5375692106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40927470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ee807002","size":5,"soft_fail":false,"target_va":5416621600,"va":5375694125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40927229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8fd7f7002","size":5,"soft_fail":false,"target_va":5416621762,"va":5375694528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$theme_fetcher..MaybePendingActiveEvent$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9189115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8fb368c00","size":5,"soft_fail":false,"target_va":5384885440,"va":5375696320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$reqwest..blocking..response..Response$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t9198340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8045b8c00","size":5,"soft_fail":false,"target_va":5384883104,"va":5375684759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..backtrace..Backtrace$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9180386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8e2148c00","size":5,"soft_fail":false,"target_va":5384879296,"va":5375698905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..fs..ReadDir$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9180930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e802178c00","size":5,"soft_fail":false,"target_va":5384878800,"va":5375697865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..ThemeFetcher$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9185471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8bf288c00","size":5,"soft_fail":false,"target_va":5384879504,"va":5375694028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e87e0b0000","size":5,"soft_fail":false,"target_va":5375700672,"va":5375697725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t40927453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8dd807002","size":5,"soft_fail":false,"target_va":5416621632,"va":5375694174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap$LT$T$GT$::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9328961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e841598e00","size":5,"soft_fail":false,"target_va":5385012448,"va":5375683482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::map::HeaderMap::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9422339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e803c68f00","size":5,"soft_fail":false,"target_va":5385109552,"va":5375687208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] http::header::name::HdrName::from_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9319092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8b4328e00","size":5,"soft_fail":false,"target_va":5385001392,"va":5375682295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] httpdate::parse_http_date
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"httpdate","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9325768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8c84c8e00","size":5,"soft_fail":false,"target_va":5385008240,"va":5375682467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::client::Client::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9208235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8ab818c00","size":5,"soft_fail":false,"target_va":5384890240,"va":5375682000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::header
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9208863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e81f848c00","size":5,"soft_fail":false,"target_va":5384890944,"va":5375682076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9290278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e826c28d00","size":5,"soft_fail":false,"target_va":5384977920,"va":5375687637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::request::RequestBuilder::send
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9296496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e870da8d00","size":5,"soft_fail":false,"target_va":5384978624,"va":5375682123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::bytes
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9401749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e895758f00","size":5,"soft_fail":false,"target_va":5385086720,"va":5375684966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] reqwest::blocking::response::Response::content_length
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9400924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e85c728f00","size":5,"soft_fail":false,"target_va":5385085840,"va":5375684911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9438479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e80f059000","size":5,"soft_fail":false,"target_va":5385130848,"va":5375692364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9046009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8f9078a00","size":5,"soft_fail":false,"target_va":5384725152,"va":5375679138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9043364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8a4fd8900","size":5,"soft_fail":false,"target_va":5384723232,"va":5375679863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::remove_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9042742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e836fb8900","size":5,"soft_fail":false,"target_va":5384722736,"va":5375679989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40936097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e8a1a27002","size":5,"soft_fail":false,"target_va":5416626560,"va":5375690458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9035596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e84cdf8900","size":5,"soft_fail":false,"target_va":5384728160,"va":5375692559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] theme_fetcher::checked_zip::verify_folder_checksums
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9206070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375678848,"raw_hex":"e836798c00","size":5,"soft_fail":false,"target_va":5384886176,"va":5375680101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t*32(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375678848,"raw_hex":"ff5020","size":3,"soft_fail":false,"va":5375685605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":21396,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":33004,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":24980,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":23500,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":18135,"source_line":453,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":33520,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":18154,"source_line":461,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":21368,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":38320,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":13128,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":12308,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":34268,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":12476,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":25008,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":36352,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":32360,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":34996,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":15676,"source_line":1893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":23556,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":258,"record_offset":23640,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":12364,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":12224,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":258,"record_offset":39136,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::download_blob
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":25620,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":18136,"source_line":164,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::fetch_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":13272,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":18121,"source_line":59,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] theme_fetcher::ThemeFetcher::trim_cas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":2284,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":18110,"source_line":204,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":2080,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":2080,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":2080,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":2080,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":2080,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":2080,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":13828,"parent_record_offset":13796,"ancestor_record_offset":2080,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":3660,"parent_record_offset":3604,"ancestor_record_offset":2080,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":13796,"parent_record_offset":13752,"ancestor_record_offset":2080,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$15::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":258,"record_offset":13716,"parent_record_offset":13680,"ancestor_record_offset":2080,"inlinee":18123,"source_line":518,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":13752,"parent_record_offset":13716,"ancestor_record_offset":2080,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":22976,"parent_record_offset":22948,"ancestor_record_offset":2080,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$24::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":258,"record_offset":21540,"parent_record_offset":21432,"ancestor_record_offset":2080,"inlinee":18134,"source_line":2191,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":17088,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":258,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":2080,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":2080,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16668,"parent_record_offset":16640,"ancestor_record_offset":2080,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2080,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2080,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":2080,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":2080,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":2080,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":2080,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2080,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":2080,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16548,"parent_record_offset":16520,"ancestor_record_offset":2080,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16576,"parent_record_offset":16548,"ancestor_record_offset":2080,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16604,"parent_record_offset":16576,"ancestor_record_offset":2080,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":258,"record_offset":16520,"parent_record_offset":16492,"ancestor_record_offset":2080,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":258,"record_offset":16492,"parent_record_offset":16464,"ancestor_record_offset":2080,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":13616,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":5433,"source_line":1880,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":21484,"parent_record_offset":21432,"ancestor_record_offset":2080,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":2080,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16408,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":21432,"parent_record_offset":21396,"ancestor_record_offset":2080,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":16464,"parent_record_offset":16436,"ancestor_record_offset":2080,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":38460,"parent_record_offset":38436,"ancestor_record_offset":2080,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":29760,"parent_record_offset":25620,"ancestor_record_offset":2080,"inlinee":18157,"source_line":2670,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":24176,"parent_record_offset":24152,"ancestor_record_offset":2080,"inlinee":7029,"source_line":1810,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":19128,"parent_record_offset":19104,"ancestor_record_offset":2080,"inlinee":4851,"source_line":1644,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":19152,"parent_record_offset":19128,"ancestor_record_offset":2080,"inlinee":4852,"source_line":1888,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":10652,"parent_record_offset":10616,"ancestor_record_offset":2080,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":24152,"parent_record_offset":24128,"ancestor_record_offset":2080,"inlinee":13798,"source_line":1667,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":18668,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":27060,"parent_record_offset":27036,"ancestor_record_offset":2080,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":258,"record_offset":10616,"parent_record_offset":10592,"ancestor_record_offset":2080,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14180,"parent_record_offset":14152,"ancestor_record_offset":2080,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14152,"parent_record_offset":14100,"ancestor_record_offset":2080,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":16640,"parent_record_offset":16604,"ancestor_record_offset":2080,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":35208,"parent_record_offset":35176,"ancestor_record_offset":2080,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14072,"parent_record_offset":13984,"ancestor_record_offset":2080,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":2080,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":19792,"parent_record_offset":19768,"ancestor_record_offset":2080,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":14100,"parent_record_offset":14072,"ancestor_record_offset":2080,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":258,"record_offset":38488,"parent_record_offset":38460,"ancestor_record_offset":2080,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":258,"record_offset":35176,"parent_record_offset":35144,"ancestor_record_offset":2080,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27396,"parent_record_offset":27360,"ancestor_record_offset":2080,"inlinee":6353,"source_line":319,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":2868,"parent_record_offset":2828,"ancestor_record_offset":2080,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_std
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27360,"parent_record_offset":27308,"ancestor_record_offset":2080,"inlinee":6291,"source_line":151,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":258,"record_offset":30232,"parent_record_offset":25620,"ancestor_record_offset":2080,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":258,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":2080,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":258,"record_offset":2796,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::error::impl$1::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":258,"record_offset":27308,"parent_record_offset":27272,"ancestor_record_offset":2080,"inlinee":6355,"source_line":739,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::store
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18936,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":18154,"source_line":461,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ArcSwapAny::swap
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":258,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":2080,"inlinee":18155,"source_line":466,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":24212,"parent_record_offset":23768,"ancestor_record_offset":2080,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":258,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":2080,"inlinee":4846,"source_line":81,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::LocalNode::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":24388,"parent_record_offset":24348,"ancestor_record_offset":2080,"inlinee":7060,"source_line":223,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::THREAD_HEAD::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":258,"record_offset":24500,"parent_record_offset":24472,"ancestor_record_offset":2080,"inlinee":7046,"source_line":92,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":24572,"parent_record_offset":24436,"ancestor_record_offset":2080,"inlinee":7048,"source_line":226,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::list::impl$3::with::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/list.rs
//     evidence={"module":258,"record_offset":24816,"parent_record_offset":24776,"ancestor_record_offset":2080,"inlinee":7049,"source_line":239,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":24128,"parent_record_offset":23768,"ancestor_record_offset":2080,"inlinee":7028,"source_line":96,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":258,"record_offset":19104,"parent_record_offset":18960,"ancestor_record_offset":2080,"inlinee":4850,"source_line":121,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$1::drop
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":23768,"parent_record_offset":23740,"ancestor_record_offset":2080,"inlinee":7026,"source_line":104,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":2080,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":258,"record_offset":19048,"parent_record_offset":18960,"ancestor_record_offset":2080,"inlinee":4844,"source_line":196,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] bytes::bytes::impl$3::drop
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bytes","version":"1.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.6.0/src/bytes.rs
//     evidence={"module":258,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":2080,"inlinee":18159,"source_line":546,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":23008,"parent_record_offset":22976,"ancestor_record_offset":2080,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":258,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":2080,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":258,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":2080,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":258,"record_offset":24656,"parent_record_offset":24572,"ancestor_record_offset":2080,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":29788,"parent_record_offset":29760,"ancestor_record_offset":2080,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":3312,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":2524,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":2080,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":2724,"parent_record_offset":2692,"ancestor_record_offset":2080,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":258,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":2080,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":21512,"parent_record_offset":21432,"ancestor_record_offset":2080,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":258,"record_offset":17172,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":258,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":2080,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":258,"record_offset":39052,"parent_record_offset":39024,"ancestor_record_offset":2080,"inlinee":16078,"source_line":20,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":258,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":2080,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":18584,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":258,"record_offset":39024,"parent_record_offset":38788,"ancestor_record_offset":2080,"inlinee":16076,"source_line":3422,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":23072,"parent_record_offset":23036,"ancestor_record_offset":2080,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":19248,"parent_record_offset":18936,"ancestor_record_offset":2080,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":258,"record_offset":23812,"parent_record_offset":23768,"ancestor_record_offset":2080,"inlinee":4744,"source_line":260,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":2080,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":258,"record_offset":23036,"parent_record_offset":23008,"ancestor_record_offset":2080,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":2080,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":20292,"parent_record_offset":20264,"ancestor_record_offset":2080,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":258,"record_offset":24472,"parent_record_offset":24436,"ancestor_record_offset":2080,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":38880,"parent_record_offset":38856,"ancestor_record_offset":2080,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":24600,"parent_record_offset":24572,"ancestor_record_offset":2080,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":2080,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2080,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":17112,"parent_record_offset":17088,"ancestor_record_offset":2080,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":24068,"parent_record_offset":23768,"ancestor_record_offset":2080,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":258,"record_offset":24868,"parent_record_offset":24816,"ancestor_record_offset":2080,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":24744,"parent_record_offset":24436,"ancestor_record_offset":2080,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":2080,"inlinee":4853,"source_line":993,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":17000,"parent_record_offset":16604,"ancestor_record_offset":2080,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":258,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":2080,"inlinee":4854,"source_line":935,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":2080,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=276
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":2908,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":258,"record_offset":18548,"parent_record_offset":18524,"ancestor_record_offset":2080,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":10684,"parent_record_offset":10652,"ancestor_record_offset":2080,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":258,"record_offset":14408,"parent_record_offset":14380,"ancestor_record_offset":2080,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":41512,"parent_record_offset":41484,"ancestor_record_offset":2080,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":21568,"parent_record_offset":21432,"ancestor_record_offset":2080,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":25032,"parent_record_offset":25008,"ancestor_record_offset":2080,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":2752,"parent_record_offset":2692,"ancestor_record_offset":2080,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":15232,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":2080,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":24776,"parent_record_offset":24388,"ancestor_record_offset":2080,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":13644,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":4200,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":38348,"parent_record_offset":38320,"ancestor_record_offset":2080,"inlinee":8879,"source_line":553,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":2080,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":2080,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":258,"record_offset":18500,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":258,"record_offset":14296,"parent_record_offset":14100,"ancestor_record_offset":2080,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":258,"record_offset":18524,"parent_record_offset":18500,"ancestor_record_offset":2080,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":258,"record_offset":14380,"parent_record_offset":14348,"ancestor_record_offset":2080,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":29816,"parent_record_offset":29788,"ancestor_record_offset":2080,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":18992,"parent_record_offset":18960,"ancestor_record_offset":2080,"inlinee":6416,"source_line":1764,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":24236,"parent_record_offset":24212,"ancestor_record_offset":2080,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19332,"parent_record_offset":19296,"ancestor_record_offset":2080,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":2080,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":27156,"parent_record_offset":27092,"ancestor_record_offset":2080,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":2080,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2080,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":10712,"parent_record_offset":10652,"ancestor_record_offset":2080,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":2080,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":258,"record_offset":10744,"parent_record_offset":10616,"ancestor_record_offset":2080,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":258,"record_offset":3368,"parent_record_offset":3312,"ancestor_record_offset":2080,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":13956,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":18125,"source_line":676,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::HeaderMap::get2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":13984,"parent_record_offset":13956,"ancestor_record_offset":2080,"inlinee":18126,"source_line":683,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::map::as_header_name::impl$4::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/map.rs
//     evidence={"module":258,"record_offset":14016,"parent_record_offset":13984,"ancestor_record_offset":2080,"inlinee":18128,"source_line":3422,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::value::HeaderValue::to_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/value.rs
//     evidence={"module":258,"record_offset":14348,"parent_record_offset":13272,"ancestor_record_offset":2080,"inlinee":18130,"source_line":251,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] http::header::value::is_visible_ascii
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"http","version":"0.2.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/http-0.2.9/src/header/value.rs
//     evidence={"module":258,"record_offset":14444,"parent_record_offset":14348,"ancestor_record_offset":2080,"inlinee":18132,"source_line":583,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2080,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":2580,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":258,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2080,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2080,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":258,"record_offset":2440,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::async_impl::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/async_impl/request.rs
//     evidence={"module":258,"record_offset":26624,"parent_record_offset":26588,"ancestor_record_offset":2080,"inlinee":18145,"source_line":43,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26424,"parent_record_offset":25620,"ancestor_record_offset":2080,"inlinee":18138,"source_line":892,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::Client::request
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26456,"parent_record_offset":26424,"ancestor_record_offset":2080,"inlinee":18139,"source_line":949,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27012,"parent_record_offset":26456,"ancestor_record_offset":2080,"inlinee":18147,"source_line":49,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27036,"parent_record_offset":27012,"ancestor_record_offset":2080,"inlinee":18149,"source_line":987,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$14::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":27192,"parent_record_offset":27036,"ancestor_record_offset":2080,"inlinee":18151,"source_line":1157,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::client::impl$4::request::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/client.rs
//     evidence={"module":258,"record_offset":26560,"parent_record_offset":26492,"ancestor_record_offset":2080,"inlinee":18141,"source_line":950,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] reqwest::blocking::request::Request::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"reqwest","version":"0.11.22","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.11.22/src/blocking/request.rs
//     evidence={"module":258,"record_offset":26588,"parent_record_offset":26560,"ancestor_record_offset":2080,"inlinee":18143,"source_line":37,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":258,"record_offset":35112,"parent_record_offset":35080,"ancestor_record_offset":2080,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3076,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":18112,"source_line":2575,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2080,"inlinee":18118,"source_line":1955,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::Metadata::is_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3196,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":18117,"source_line":1978,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":2692,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":3400,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":13843,"source_line":3232,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":258,"record_offset":4228,"parent_record_offset":2936,"ancestor_record_offset":2080,"inlinee":5286,"source_line":2698,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":38856,"parent_record_offset":38788,"ancestor_record_offset":2080,"inlinee":16075,"source_line":667,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":25864,"parent_record_offset":25620,"ancestor_record_offset":2080,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":38828,"parent_record_offset":38788,"ancestor_record_offset":2080,"inlinee":16073,"source_line":389,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":38788,"parent_record_offset":38764,"ancestor_record_offset":2080,"inlinee":16071,"source_line":1017,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":35080,"parent_record_offset":34996,"ancestor_record_offset":2080,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":258,"record_offset":38764,"parent_record_offset":38620,"ancestor_record_offset":2080,"inlinee":16069,"source_line":2218,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::DirEntry::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":2080,"inlinee":18113,"source_line":188,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileAttr::file_type
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2080,"inlinee":18120,"source_line":1088,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::FileType::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2080,"inlinee":13258,"source_line":1187,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::impl$24::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":258,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2080,"inlinee":18115,"source_line":1133,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":258,"record_offset":35144,"parent_record_offset":35112,"ancestor_record_offset":2080,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":258,"record_offset":24528,"parent_record_offset":24500,"ancestor_record_offset":2080,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":258,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":2080,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$2::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":258,"record_offset":38676,"parent_record_offset":38652,"ancestor_record_offset":2080,"inlinee":18170,"source_line":115,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":258,"record_offset":24436,"parent_record_offset":24388,"ancestor_record_offset":2080,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":258,"record_offset":13184,"parent_record_offset":13128,"ancestor_record_offset":2080,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":258,"record_offset":38652,"parent_record_offset":38620,"ancestor_record_offset":2080,"inlinee":18168,"source_line":248,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$0::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":35040,"parent_record_offset":34996,"ancestor_record_offset":2080,"inlinee":18161,"source_line":3,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":38408,"parent_record_offset":38384,"ancestor_record_offset":2080,"inlinee":18164,"source_line":3,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":37576,"parent_record_offset":37488,"ancestor_record_offset":2080,"inlinee":18172,"source_line":10,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_defs::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-defs","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-defs/src/lib.rs
//     evidence={"module":258,"record_offset":38620,"parent_record_offset":38596,"ancestor_record_offset":2080,"inlinee":18166,"source_line":10,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::ThemeFetcher::validate_cas_cache_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":2936,"parent_record_offset":2284,"ancestor_record_offset":2080,"inlinee":18111,"source_line":226,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":32404,"parent_record_offset":32360,"ancestor_record_offset":2080,"inlinee":18153,"source_line":294,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::impl$0::run::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/lib.rs
//     evidence={"module":258,"record_offset":36380,"parent_record_offset":36352,"ancestor_record_offset":2080,"inlinee":18162,"source_line":305,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46E9
// 首条位置: {"module":258,"inlinee":18153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":32440,"parent_record_offset":32404,"ancestor_record_offset":32404,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":32940,"parent_record_offset":32404,"ancestor_record_offset":32404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":32812,"parent_record_offset":32784,"ancestor_record_offset":32404,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":32840,"parent_record_offset":32812,"ancestor_record_offset":32404,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":32868,"parent_record_offset":32840,"ancestor_record_offset":32404,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":32784,"parent_record_offset":32720,"ancestor_record_offset":32404,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":32752,"parent_record_offset":32720,"ancestor_record_offset":32404,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":32720,"parent_record_offset":32692,"ancestor_record_offset":32404,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":258,"record_offset":32692,"parent_record_offset":32668,"ancestor_record_offset":32404,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":32528,"parent_record_offset":32476,"ancestor_record_offset":32404,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":258,"record_offset":32476,"parent_record_offset":32440,"ancestor_record_offset":32404,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":258,"record_offset":32556,"parent_record_offset":32476,"ancestor_record_offset":32404,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":32620,"parent_record_offset":32476,"ancestor_record_offset":32404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":258,"record_offset":32584,"parent_record_offset":32476,"ancestor_record_offset":32404,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::impl$0::run::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x46F2
// 首条位置: {"module":258,"inlinee":18162}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::MaybePendingActiveEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >", "constructor": "alloc::sync::ArcInner", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::ArcSwapAny", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::ArcSwapAny<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<theme_fetcher::MaybePendingActiveEvent,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["theme_fetcher::MaybePendingActiveEvent", "core::slice::iter::Iter<theme_server_proto::ActiveEvent>", "theme_fetcher::impl$0::fetch_events::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::ThemeFetcher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::ThemeFetcher>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::ThemeFetcher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "theme_fetcher::ThemeFetcher::new<std::path::PathBuf>", "constructor": "theme_fetcher::ThemeFetcher::new", "arguments": ["std::path::PathBuf"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 函数: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global> >
// 信息 ID: d013da4e4ac0f6bb260ebb6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0 -> ["alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["theme_fetcher::MaybePendingActiveEvent", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] arc_swap::debt::helping::Slots::help -> ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<alloc::vec::Vec<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global>,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > @ 0xfa3318 -> 0x1ed8be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>,alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global> >
// 信息 ID: ea883ea3942402c07ec89ebb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["theme_defs::EventDataError"]; 字段布局 = unknown。
// [类型关系] core::result::Result -> ["std::path::PathBuf", "enum2$<theme_defs::EventDataError>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> >"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0 -> ["alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>", "arc_swap::strategy::hybrid::DefaultConfig"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["theme_defs::EventDataError"]; 字段布局 = unknown。
// [类型关系] core::result::Result -> ["std::path::PathBuf", "enum2$<theme_defs::EventDataError>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> >"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] arc_swap::debt::helping::Slots::help -> ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig>", "alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] arc_swap::debt::impl$2::pay_all::closure$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<alloc::sync::Arc<enum2$<core::result::Result<std::path::PathBuf,enum2$<theme_defs::EventDataError> > >,alloc::alloc::Global>,arc_swap::strategy::hybrid::DefaultConfig> > @ 0xfa3198 -> 0x1ed8da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_exponent<serde_json::read::SliceRead>
// 信息 ID: c2741493b418804ff25d7b76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_exponent -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d047 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_number<serde_json::read::SliceRead> @ 0x15d191 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal<serde_json::read::SliceRead> @ 0xd81e81 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd821ff -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead> @ 0xd8210c -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_long_integer<serde_json::read::SliceRead> @ 0x2d9eda7 -> 0x15d890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> | serde_json::de::Deserializer::parse_decimal_overflow<serde_json::read::SliceRead> @ 0x2d9ef33 -> 0x15d890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::Deserializer::parse_integer<serde_json::read::SliceRead>
// 信息 ID: f282a6b86a913fdf1ac23c39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer::parse_integer -> ["serde_json::read::SliceRead"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd83376 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xd833bb -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa090f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<i64,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0xfa0944 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impl$5::deserialize<u32,ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x115c9ca -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f856 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::SliceRead> > > @ 0x1d5f89b -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e8ef -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2d9e992 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2809 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db2879 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8af9 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2db8b69 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de230f -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::Deserializer::peek_invalid_type<serde_json::read::SliceRead> @ 0x2de23b2 -> 0xd81fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent> >
// 信息 ID: a61366d2604ed6e5a56f5115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_core::de::impls::impl$19::deserialize::VecVisitor -> ["theme_server_proto::ActiveEvent"]; 字段布局 = unknown。
// [类型关系] serde_json::de::impl$7::deserialize_seq -> ["serde_json::read::SliceRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<theme_server_proto::ActiveEvent>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor> @ 0xfa1f51 -> 0xfa0c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor> @ 0xfa1fc6 -> 0xfa0c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> > >
// 信息 ID: 56ce0127c8bcb9a771056580
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] serde_core::de::impls::impl$19::deserialize::VecVisitor -> ["tuple$<alloc::string::String,alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::impl$7::deserialize_seq -> ["serde_json::read::SliceRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<tuple$<alloc::string::String,alloc::string::String> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor> @ 0xfa12ec -> 0xfa09f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor> @ 0xfa13b4 -> 0xfa09f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>
// 信息 ID: 23410c163e366043f64b90fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::impl$7::deserialize_struct -> ["serde_json::read::SliceRead", "theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo> @ 0xf9ff2f -> 0xfa1060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_server_proto::_::impl$0::deserialize::__Visitor>
// 信息 ID: 6f47dce5312704d8dad715aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::impl$7::deserialize_struct -> ["serde_json::read::SliceRead", "theme_server_proto::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::from_slice<theme_server_proto::EventsResponse> @ 0xf9fd4f -> 0xfa1d80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<theme_server_proto::ActiveEvent,serde_json::de::SeqAccess<serde_json::read::SliceRead> > @ 0x1ed880c -> 0xfa2500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: std::io::cursor::impl$5::read_to_string<ref$<bytes::bytes::Bytes> >
// 信息 ID: 45bd34e982f679f7b92b3f36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::impl$5::read_to_string -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: std::io::default_read_to_end<zip::read::ZipFile>
// 信息 ID: cc49734bd9586bd5e1b1f6be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::default_read_to_end -> ["zip::read::ZipFile"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > > @ 0xfa80bf -> 0x1ee2430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::download_blob
// 信息 ID: 8957abd05276cc4913273908
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=25620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::fetch_events
// 信息 ID: d3d26607e7875b741e3c0b75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=13272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::new<std::path::PathBuf>
// 信息 ID: 209b6b8b80c743b58cce8f22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] theme_fetcher::ThemeFetcher::new -> ["std::path::PathBuf"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::theme_fetcher_plugin::impl$0::build @ 0x15aec3 -> 0x3facc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::run
// 信息 ID: 7a7ac927363a0357a57b3259
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > @ 0x5e6561 -> 0x6a5980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::trim_cas
// 信息 ID: 9f985da78ff7cc2367903188
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=2284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::ThemeFetcher::validate_cas_cache_entry
// 信息 ID: 9c3d079cfd9619a4ca1453a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=2936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums
// 信息 ID: 7bcaf5a6b13861709409136f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x6a5e65 -> 0xf6d7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::fetch_events::closure$0
// 信息 ID: b8e0d1605f0ee9cb68c03f2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::fetch_events", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<theme_fetcher::MaybePendingActiveEvent,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] theme_fetcher::impl$0::fetch_events::closure_env$0 <- core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<theme_fetcher::MaybePendingActiveEvent,core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::vec::impl$21::extend_trusted::closure_env$0<theme_fetcher::MaybePendingActiveEvent,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<theme_server_proto::ActiveEvent>,theme_fetcher::impl$0::fetch_events::closure_env$0> > > >; module=2359, inline_record=1180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$0
// 信息 ID: 2a95fa23e2e35601dedc22ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=32404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::impl$0::run::closure$1
// 信息 ID: cae692b9e2601ae004268785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "theme_fetcher::impl$0::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::ThemeFetcher::run; module=258, inline_record=36380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zip::read::find_content<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: fa4cd7fbb6c00c189edc0205
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] zip::read::find_content -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zip::read::zip_archive::ZipArchive::by_index<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > > @ 0xfa7aea -> 0x1ee1180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zip::read::zip_archive::ZipArchive::new<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: 7201c84e474cc1b0433c647d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] zip::read::zip_archive::ZipArchive::new -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x6a8522 -> 0xfa7010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
