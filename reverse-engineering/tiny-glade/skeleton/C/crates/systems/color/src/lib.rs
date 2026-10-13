// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/color/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0f0cf1d8b04bb5c75169b421dab19d8c1fcf0b1bf7ca18e2e1a3e3702c97a3d9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 317, 1118, 1122, 1130, 2695, 2699, 2702, 2704, 3985, 3987, 3992, 4023, 4794, 4800, 5019
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: [{"name":"system-color","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3985,"record_offset":952,"rva":42477552,"va":5411186672}
// 编译实例 RVAs: 0x28827f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140147ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41135646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411186672,"raw_hex":"e8e2518cfd","size":5,"soft_fail":false,"target_va":5370051488,"va":5411187129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-35246520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411186672,"raw_hex":"e8482ee6fd","size":5,"soft_fail":false,"target_va":5375940368,"va":5411186883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7789045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411186672,"raw_hex":"e80b2689ff","size":5,"soft_fail":false,"target_va":5403397760,"va":5411186800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5435152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411186672,"raw_hex":"e810ef5200","size":5,"soft_fail":false,"target_va":5416622240,"va":5411187083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] arc_swap::ref_cnt::impl$2::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3985,"record_offset":2276,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":7027,"source_line":166,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3985,"record_offset":1512,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":16674,"source_line":201,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3985,"record_offset":2556,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2332,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicPtr::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2736,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":6403,"source_line":1730,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2500,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1400,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2444,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":5319,"source_line":2870,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3985,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":952,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3985,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":952,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":3985,"record_offset":1852,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":7010,"source_line":65,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3985,"record_offset":1948,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4848,"source_line":86,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::inc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":3985,"record_offset":1688,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":7017,"source_line":73,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$2::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3985,"record_offset":1572,"parent_record_offset":1512,"ancestor_record_offset":952,"inlinee":7016,"source_line":135,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":3985,"record_offset":1544,"parent_record_offset":1512,"ancestor_record_offset":952,"inlinee":16675,"source_line":186,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3985,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":952,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3985,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":952,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3985,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":952,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3985,"record_offset":1632,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3985,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":952,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3985,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3985,"record_offset":2184,"parent_record_offset":1572,"ancestor_record_offset":952,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":952,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1784,"parent_record_offset":1748,"ancestor_record_offset":952,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2080,"parent_record_offset":2044,"ancestor_record_offset":952,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":952,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1908,"parent_record_offset":1880,"ancestor_record_offset":952,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":952,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":952,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":952,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3985,"record_offset":2136,"parent_record_offset":2044,"ancestor_record_offset":952,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03985.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: bevy_ecs::system::system_param::impl$110::validate_param<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1122,"record_offset":147720,"rva":17510624,"va":5386219744}
// 编译实例 RVAs: 0x10b30e0, 0x21580c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] system_color::style_changes::_::impl$0::validate_param
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386219744,"raw_hex":"e876e2ffff","size":5,"soft_fail":false,"target_va":5386212208,"va":5386219765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":148264,"parent_record_offset":147720,"ancestor_record_offset":147720,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1122,"record_offset":148224,"parent_record_offset":147720,"ancestor_record_offset":147720,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1122,"record_offset":150320,"parent_record_offset":147720,"ancestor_record_offset":147720,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::_::impl$0::validate_param
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1122,"record_offset":148876,"parent_record_offset":147720,"ancestor_record_offset":147720,"inlinee":5854,"source_line":336,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":148624,"parent_record_offset":148600,"ancestor_record_offset":147720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":148672,"parent_record_offset":148648,"ancestor_record_offset":147720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":148648,"parent_record_offset":148624,"ancestor_record_offset":147720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":148600,"parent_record_offset":148576,"ancestor_record_offset":147720,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":148576,"parent_record_offset":148536,"ancestor_record_offset":147720,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":148536,"parent_record_offset":148512,"ancestor_record_offset":147720,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1122,"record_offset":148832,"parent_record_offset":148808,"ancestor_record_offset":147720,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1122,"record_offset":148808,"parent_record_offset":148788,"ancestor_record_offset":147720,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1122,"record_offset":148292,"parent_record_offset":148264,"ancestor_record_offset":147720,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":148340,"parent_record_offset":148316,"ancestor_record_offset":147720,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":148316,"parent_record_offset":148292,"ancestor_record_offset":147720,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":148512,"parent_record_offset":148488,"ancestor_record_offset":147720,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":148900,"parent_record_offset":148876,"ancestor_record_offset":147720,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1122,"record_offset":148428,"parent_record_offset":148368,"ancestor_record_offset":147720,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":148448,"parent_record_offset":148428,"ancestor_record_offset":147720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":148764,"parent_record_offset":148264,"ancestor_record_offset":147720,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":148488,"parent_record_offset":148316,"ancestor_record_offset":147720,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1122,"record_offset":148368,"parent_record_offset":148340,"ancestor_record_offset":147720,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1122,"record_offset":148400,"parent_record_offset":148368,"ancestor_record_offset":147720,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1122,"record_offset":148716,"parent_record_offset":148536,"ancestor_record_offset":147720,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1122,"record_offset":149176,"parent_record_offset":149152,"ancestor_record_offset":147720,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4800,"record_offset":640,"rva":45210032,"va":5413919152}
// 编译实例 RVAs: 0x2b1d9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..option..Option$LT$$RF$$u5b$u8$u5d$$GT$$u20$as$u20$rhapsody..raw..buffer..BufferInitialData$GT$::byte_count
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-30455177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8774a2ffe","size":5,"soft_fail":false,"target_va":5383464096,"va":5413919268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$core..option..Option$LT$$RF$$u5b$u8$u5d$$GT$$u20$as$u20$rhapsody..raw..buffer..BufferInitialData$GT$::copy_to_slice
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30455397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e89b492ffe","size":5,"soft_fail":false,"target_va":5383464112,"va":5413919504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44614805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e86b3b57fd","size":5,"soft_fail":false,"target_va":5369305584,"va":5413920384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410a28a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27768013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8334b58fe","size":5,"soft_fail":false,"target_va":5386152096,"va":5413920104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2701257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8c9372900","size":5,"soft_fail":false,"target_va":5416621536,"va":5413920274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2701391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e84f382900","size":5,"soft_fail":false,"target_va":5416621600,"va":5413920204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2702044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8dc3a2900","size":5,"soft_fail":false,"target_va":5416622208,"va":5413920159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2702186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e86a3b2900","size":5,"soft_fail":false,"target_va":5416622320,"va":5413920129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2656606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e85e892800","size":5,"soft_fail":false,"target_va":5416576832,"va":5413920221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2655902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e89e862800","size":5,"soft_fail":false,"target_va":5416576144,"va":5413920237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29934818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e81e3b37fe","size":5,"soft_fail":false,"target_va":5383984848,"va":5413919661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29934352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8f03c37fe","size":5,"soft_fail":false,"target_va":5383985360,"va":5413919707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-30431652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e85ca62ffe","size":5,"soft_fail":false,"target_va":5383487952,"va":5413919599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-30202699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8b52433fe","size":5,"soft_fail":false,"target_va":5383716688,"va":5413919382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-30196085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e88b3e33fe","size":5,"soft_fail":false,"target_va":5383723216,"va":5413919296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::with_setup_cb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-30195609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e8674033fe","size":5,"soft_fail":false,"target_va":5383724400,"va":5413920004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-30394051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413919152,"raw_hex":"e83d3930fe","size":5,"soft_fail":false,"target_va":5383525648,"va":5413919694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4800,"record_offset":864,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4800,"record_offset":1656,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4800,"record_offset":1468,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4800,"record_offset":1040,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4800,"record_offset":1172,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4800,"record_offset":1076,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":4800,"record_offset":1412,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::hot
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4800,"record_offset":2004,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":8236,"source_line":146,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::dynamic_constants
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":4800,"record_offset":1200,"parent_record_offset":640,"ancestor_record_offset":640,"inlinee":16914,"source_line":919,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":4800,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":640,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4800,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":640,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4800,"record_offset":1136,"parent_record_offset":1104,"ancestor_record_offset":640,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":4800,"record_offset":2152,"parent_record_offset":2124,"ancestor_record_offset":640,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4800,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":640,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4800,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":640,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4800,"record_offset":2180,"parent_record_offset":2124,"ancestor_record_offset":640,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4800,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":640,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4800,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":640,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4800,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":640,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4800,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":640,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4800,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":640,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4800,"record_offset":1588,"parent_record_offset":1524,"ancestor_record_offset":640,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4800,"record_offset":1316,"parent_record_offset":1280,"ancestor_record_offset":640,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4800,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":640,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4800,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":640,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":4800,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":640,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":4800,"record_offset":1280,"parent_record_offset":1228,"ancestor_record_offset":640,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":4800,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":640,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":4800,"record_offset":1372,"parent_record_offset":1280,"ancestor_record_offset":640,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":4800,"record_offset":1560,"parent_record_offset":1524,"ancestor_record_offset":640,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::Handle::hot
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4800,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":640,"inlinee":8238,"source_line":859,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":4800,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":640,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/04800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_changes::StyleScene::wall_style
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2695,"record_offset":268,"rva":34877600,"va":5403586720}
// 编译实例 RVAs: 0x21430a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb800
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18381441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e87f85e7fe","size":5,"soft_fail":false,"target_va":5385205760,"va":5403587196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::WallColorTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22682500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e87ce4a5fe","size":5,"soft_fail":false,"target_va":5380904528,"va":5403587023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22389684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e84c5caafe","size":5,"soft_fail":false,"target_va":5381197456,"va":5403587135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22277590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e82a12acfe","size":5,"soft_fail":false,"target_va":5381309184,"va":5403586769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_offset_range
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22353988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e8bce7aafe","size":5,"soft_fail":false,"target_va":5381233184,"va":5403587167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22683575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e849e0a5fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5403587106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22684353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403586720,"raw_hex":"e83fdda5fe","size":5,"soft_fail":false,"target_va":5380902448,"va":5403586796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2695,"record_offset":3048,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":424,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":1924,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":3108,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":2000,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2695,"record_offset":2028,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2695,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":268,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":1028,"parent_record_offset":996,"ancestor_record_offset":268,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":268,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2695,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":268,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2695,"record_offset":784,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2695,"record_offset":564,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2695,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":268,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2695,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":268,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":2695,"record_offset":716,"parent_record_offset":696,"ancestor_record_offset":268,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2695,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":268,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2695,"record_offset":1204,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2695,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":268,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2695,"record_offset":1732,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2695,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":268,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2695,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":268,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2695,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2695,"record_offset":1304,"parent_record_offset":1280,"ancestor_record_offset":268,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2695,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":268,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2695,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2695,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":268,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2695,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":268,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2695,"record_offset":972,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2695,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":268,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2695,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":268,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2695,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":268,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2695,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":268,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":2695,"record_offset":820,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2695,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2695,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2695,"record_offset":2792,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2695,"record_offset":2524,"parent_record_offset":2496,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2695,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2695,"record_offset":1416,"parent_record_offset":1356,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":1440,"parent_record_offset":1416,"ancestor_record_offset":268,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":668,"parent_record_offset":588,"ancestor_record_offset":268,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":1480,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2695,"record_offset":756,"parent_record_offset":512,"ancestor_record_offset":268,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2695,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2695,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2695,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2695,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":268,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2695,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":268,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2695,"record_offset":1560,"parent_record_offset":1528,"ancestor_record_offset":268,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":2695,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2695,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":268,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2695,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":268,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2695,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":268,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2695,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2695,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2695,"record_offset":2408,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":2268,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":2820,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2695,"record_offset":2352,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2695,"record_offset":2212,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2695,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2695,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":268,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2695,"record_offset":2676,"parent_record_offset":2568,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2695,"record_offset":2080,"parent_record_offset":2052,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2912,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2108,"parent_record_offset":2052,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2240,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2695,"record_offset":2568,"parent_record_offset":2156,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2695,"record_offset":1836,"parent_record_offset":1804,"ancestor_record_offset":268,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2695,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":268,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::impl$1::wall_style::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":456,"parent_record_offset":424,"ancestor_record_offset":268,"inlinee":108214,"source_line":156,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::impl$1::wall_style::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":1968,"parent_record_offset":1924,"ancestor_record_offset":268,"inlinee":108215,"source_line":164,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::impl$1::wall_style::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":268,"inlinee":108216,"source_line":165,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::impl$1::wall_style::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":3076,"parent_record_offset":3048,"ancestor_record_offset":268,"inlinee":108217,"source_line":166,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   [pdb_inline_transitive] system_color::style_sample::impl$1::wall_style::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-color","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/style_sample.rs
//     evidence={"module":2695,"record_offset":3144,"parent_record_offset":3108,"ancestor_record_offset":268,"inlinee":108218,"source_line":167,"archive":"_pdb/C/modules/02695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::style_changes::_::impl$0::validate_param
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1122,"record_offset":109848,"rva":17503088,"va":5386212208}
// 编译实例 RVAs: 0x10b1370, 0x2156160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$11>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e8ae130000","size":5,"soft_fail":false,"target_va":5386218576,"va":5386213533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$12>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e84a150000","size":5,"soft_fail":false,"target_va":5386219152,"va":5386213697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$13>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e833150000","size":5,"soft_fail":false,"target_va":5386219296,"va":5386213864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$14>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e8dc120000","size":5,"soft_fail":false,"target_va":5386218864,"va":5386214031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$15>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e885100000","size":5,"soft_fail":false,"target_va":5386218432,"va":5386214198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$16>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e8d2130000","size":5,"soft_fail":false,"target_va":5386219440,"va":5386214361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$17>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e8ae0e0000","size":5,"soft_fail":false,"target_va":5386218288,"va":5386214525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$18>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e87a0d0000","size":5,"soft_fail":false,"target_va":5386218144,"va":5386214689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$19>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e836100000","size":5,"soft_fail":false,"target_va":5386219008,"va":5386214853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,system_color::style_changes::_::impl$0::validate_param::closure_env$20>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386212208,"raw_hex":"e8720e0000","size":5,"soft_fail":false,"target_va":5386218720,"va":5386215017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::_::impl$0::validate_param
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1122,"record_offset":116932,"parent_record_offset":109848,"ancestor_record_offset":109848,"inlinee":4240,"source_line":56,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":110616,"parent_record_offset":109848,"ancestor_record_offset":109848,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":110004,"parent_record_offset":109848,"ancestor_record_offset":109848,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1122,"record_offset":115872,"parent_record_offset":109848,"ancestor_record_offset":109848,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::_::impl$0::validate_param
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":1122,"record_offset":113908,"parent_record_offset":109848,"ancestor_record_offset":109848,"inlinee":5962,"source_line":56,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":110364,"parent_record_offset":110340,"ancestor_record_offset":109848,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":110412,"parent_record_offset":110388,"ancestor_record_offset":109848,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1122,"record_offset":110388,"parent_record_offset":110364,"ancestor_record_offset":109848,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":110340,"parent_record_offset":110316,"ancestor_record_offset":109848,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":110316,"parent_record_offset":110276,"ancestor_record_offset":109848,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1122,"record_offset":110276,"parent_record_offset":110252,"ancestor_record_offset":109848,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1122,"record_offset":110572,"parent_record_offset":110548,"ancestor_record_offset":109848,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1122,"record_offset":110548,"parent_record_offset":110528,"ancestor_record_offset":109848,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1122,"record_offset":110032,"parent_record_offset":110004,"ancestor_record_offset":109848,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":110080,"parent_record_offset":110056,"ancestor_record_offset":109848,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":110056,"parent_record_offset":110032,"ancestor_record_offset":109848,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1122,"record_offset":110252,"parent_record_offset":110228,"ancestor_record_offset":109848,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::SystemParamValidationError::invalid
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":115808,"parent_record_offset":115332,"ancestor_record_offset":109848,"inlinee":10626,"source_line":2814,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::SystemParamValidationError::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":115832,"parent_record_offset":115808,"ancestor_record_offset":109848,"inlinee":9599,"source_line":2820,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":113936,"parent_record_offset":113908,"ancestor_record_offset":109848,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1122,"record_offset":116956,"parent_record_offset":116932,"ancestor_record_offset":109848,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1122,"record_offset":110168,"parent_record_offset":110108,"ancestor_record_offset":109848,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":114524,"parent_record_offset":114500,"ancestor_record_offset":109848,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":110188,"parent_record_offset":110168,"ancestor_record_offset":109848,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":110504,"parent_record_offset":110004,"ancestor_record_offset":109848,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1122,"record_offset":110228,"parent_record_offset":110056,"ancestor_record_offset":109848,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1122,"record_offset":117580,"parent_record_offset":116932,"ancestor_record_offset":109848,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1122,"record_offset":110108,"parent_record_offset":110080,"ancestor_record_offset":109848,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1122,"record_offset":110140,"parent_record_offset":110108,"ancestor_record_offset":109848,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1122,"record_offset":110456,"parent_record_offset":110276,"ancestor_record_offset":109848,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1122,"record_offset":110884,"parent_record_offset":110860,"ancestor_record_offset":109848,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::impl$0::add_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":317,"record_offset":1520,"rva":7267856,"va":5375976976}
// 编译实例 RVAs: 0x6ee610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10242735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8af4a9c00","size":5,"soft_fail":false,"target_va":5386225280,"va":5375982540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=9
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6777951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8a19398ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375983610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd110
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5384588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e874d6adff","size":5,"soft_fail":false,"target_va":5370597648,"va":5375982231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5383970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8ded8adff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375981869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410b32f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10238450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8f2399c00","size":5,"soft_fail":false,"target_va":5386220272,"va":5375981817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410b35b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10238967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8f73b9c00","size":5,"soft_fail":false,"target_va":5386220976,"va":5375982004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410b5290
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10246313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8a9589c00","size":5,"soft_fail":false,"target_va":5386228368,"va":5375982050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410bb100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10270019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e843b59c00","size":5,"soft_fail":false,"target_va":5386252544,"va":5375982520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410c2630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10299642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8fa289d00","size":5,"soft_fail":false,"target_va":5386282544,"va":5375982897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::init_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10218846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e85eed9b00","size":5,"soft_fail":false,"target_va":5386196336,"va":5375977485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10216414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8dee39b00","size":5,"soft_fail":false,"target_va":5386198224,"va":5375981805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10239473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8f13d9c00","size":5,"soft_fail":false,"target_va":5386221328,"va":5375981850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10202198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e856ac9b00","size":5,"soft_fail":false,"target_va":5386179440,"va":5375977237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10243753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8a94e9c00","size":5,"soft_fail":false,"target_va":5386226416,"va":5375982658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10255490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375976976,"raw_hex":"e8827c9c00","size":5,"soft_fail":false,"target_va":5386237680,"va":5375982185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":317,"record_offset":1652,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":317,"record_offset":35008,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::after
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":317,"record_offset":35236,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":4537,"source_line":510,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::before
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":317,"record_offset":35136,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20461,"source_line":504,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":317,"record_offset":35036,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":317,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":1520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":317,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":1520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":317,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":1520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":317,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1520,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":317,"record_offset":2996,"parent_record_offset":2956,"ancestor_record_offset":1520,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":317,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":1520,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":317,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":1520,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":317,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":1520,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":317,"record_offset":35076,"parent_record_offset":35036,"ancestor_record_offset":1520,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":317,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":1520,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":317,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":1520,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":317,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1520,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":317,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1520,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":317,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1520,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":317,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":1520,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":317,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":1520,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":317,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":1520,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":317,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":1520,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":317,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":1520,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":317,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":1520,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":317,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":317,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":317,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":317,"record_offset":2180,"parent_record_offset":2152,"ancestor_record_offset":1520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":317,"record_offset":2844,"parent_record_offset":2780,"ancestor_record_offset":1520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":317,"record_offset":2656,"parent_record_offset":1704,"ancestor_record_offset":1520,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":317,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":1520,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":317,"record_offset":3192,"parent_record_offset":1704,"ancestor_record_offset":1520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":317,"record_offset":2908,"parent_record_offset":2728,"ancestor_record_offset":1520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":317,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":317,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":317,"record_offset":2780,"parent_record_offset":2752,"ancestor_record_offset":1520,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":317,"record_offset":2816,"parent_record_offset":2780,"ancestor_record_offset":1520,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":317,"record_offset":3136,"parent_record_offset":2956,"ancestor_record_offset":1520,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":317,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":1520,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":317,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":1520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":317,"record_offset":2152,"parent_record_offset":2120,"ancestor_record_offset":1520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":317,"record_offset":2092,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":317,"record_offset":1952,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":317,"record_offset":2508,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":317,"record_offset":2036,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":317,"record_offset":1796,"parent_record_offset":1760,"ancestor_record_offset":1520,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":317,"record_offset":2356,"parent_record_offset":2252,"ancestor_record_offset":1520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":2600,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":2272,"parent_record_offset":2252,"ancestor_record_offset":1520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":1520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":1924,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":317,"record_offset":2252,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00317.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":317,"record_offset":2480,"parent_record_offset":1872,"ancestor_record_offset":1520,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00317.jsonl.gz"}
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
// 文件级候选接口: 31; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>", "system_color::style_changes::StyleScene", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >", "tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>", "system_color::style_changes::StyleScene"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>,bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::WallStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FreehandHeightTypeToggleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::BoardLayoutToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofRimTypeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorMenuBrightnessIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ChimneySmokeToggle> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlatRoofStyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::PlasterPeelIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StairRailingIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::MagicColorIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<system_color::radial_menu::ColorRadialMenu> >,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::remember_color_customization::RememberColorCustomizationCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<tuple$<usize,usize> > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetRoofColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetDecoratorBrightnessCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetFreehandHeightTypeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetBoardLayoutCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_window_light::SetWindowLightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::SetFlatRoofRimStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::NotInDemoHint>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightLanternCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::LightWindowCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::decorator::SetChimneySmokeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_roof_customization::SetFlatRoofStyleCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetWallMaterialCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::set_wall_style::SetPlasterPeelAmountCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairWoodColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRailingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairRoofedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetStairMaterialCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_color::set_decorator_color_id::SetMagicColorIdCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<system_color::show_color_menu::ShowColorMenuCmd>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<system_color::show_color_menu::ShowColorMenuCmd>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > > >
// 信息 ID: 21a6e0d021d726aaa60b5899
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["alloc::string::String", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<alloc::string::String,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0 -> ["enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"]; 字段布局 = unknown。
// [类型关系] alloc::sync::Arc -> ["alloc::string::String", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["alloc::sync::Arc<alloc::string::String,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] arc_swap::debt::helping::Slots::help -> ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<alloc::string::String,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > @ 0x2114c88 -> 0x28827f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: bevy_ecs::system::system_param::impl$110::validate_param<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >
// 信息 ID: 75a00204f45a297da910349c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] db64fc5d69ab42a08c5ed32e: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [系统接口类型拼写] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::change_detection::Res -> ["country_core::systems::ui::signifiers::signifier_create::UiSignifierStream"]; 字段布局 = unknown。
// [类型关系] bevy_ecs::system::system_param::impl$110::validate_param -> ["system_color::style_changes::StyleScene", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0x10b10fa -> 0x10b30e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::BuildingPlacementMaterial> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState> >,tuple$<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::structure_outline::OutlineStairsCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0x215421a -> 0x21580c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 信息 ID: 7d215f4f047f611f4f35cf3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::walls::wall_render_slots::SlotWallIdSsbo> @ 0x28b78fa -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id_roof::RoofColorIdSsbo> @ 0x28b7aca -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::StoneFoundationColorIdSsbo> @ 0x28b7c9a -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::GateColorIdSsbo> @ 0x28b7e6a -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::WallColorIdSsbo> @ 0x28b803a -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::HalftimberColorIdSsbo> @ 0x28b820a -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::color_id::WoodenWallColorIdSsbo> @ 0x28b83da -> 0x2b1d9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::impl$0::add_color
// 信息 ID: 3ebf4b90f567d47032cb6ca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x42136e -> 0x6ee610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_changes::StyleScene::wall_style
// 信息 ID: 34a97e94e5ffde28deed1378
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::style_changes::StyleScene::sample @ 0x21435ce -> 0x21430a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_changes::StyleScene::changes @ 0x2144238 -> 0x21430a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::style_changes::_::impl$0::validate_param
// 信息 ID: a377ca90b22846d857928403
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5116eddf4575e12354c2795f: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene)
// [泛型宿主/闭包/直接引用方参数候选] 84d8ba7f6d025047007b2a0f: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene)> @ 0x10b0919 -> 0x10b1370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0x10b0be3 -> 0x10b1370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system_param::impl$110::validate_param<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> > @ 0x10b30f5 -> 0x10b1370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene)> @ 0x2151681 -> 0x2156160 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerCancelState>,system_color::style_changes::StyleScene,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> @ 0x2151943 -> 0x2156160 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system_param::impl$110::validate_param<system_color::style_changes::StyleScene,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> > @ 0x21580d5 -> 0x2156160 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<system_color::style_picker::StylePicker>,system_color::style_changes::StyleScene)> > @ 0x215b448 -> 0x2156160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
