// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_clutter_packs.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b123e5c167f43fc081377e1ab871fe26ffec87e68935f90651222091178aef7d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_clutter_packs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 515, 565, 586, 1643, 1651, 1654, 1658, 1673, 1682, 1715, 1770, 3252
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use async_lock; // namespace candidate
//   package identities: [{"name":"async-lock","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.2"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use clutter_pack_pipe; // namespace candidate
//   package identities: [{"name":"clutter-pack-pipe","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::InFlightPackBuild::poll
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":23700,"rva":28166496,"va":5396875616}
// 编译实例 RVAs: 0x1adc960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12231123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e82d5e45ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5396876030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-12266566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8bad344ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396875969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21198246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e85a8abcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396876945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27653311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8410b5afe","size":5,"soft_fail":false,"target_va":5369224048,"va":5396877354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409911c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18136559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e81142ebfe","size":5,"soft_fail":false,"target_va":5378740672,"va":5396877226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416fdf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4057729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e87f15c2ff","size":5,"soft_fail":false,"target_va":5392817984,"va":5396875708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ab2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3349594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8a6e3ccff","size":5,"soft_fail":false,"target_va":5393527488,"va":5396877077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7f170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19538564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e884222a01","size":5,"soft_fail":false,"target_va":5416415600,"va":5396877031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12251904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8000d45ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396875979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::ClutterPackObject::id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14974830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e892801bff","size":5,"soft_fail":false,"target_va":5381902240,"va":5396877065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::install_built_clutter_packs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t23254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8d65a0000","size":5,"soft_fail":false,"target_va":5396899440,"va":5396876181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12150299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875616,"raw_hex":"e8e59946ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5396876982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":24024,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":24220,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":24192,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":1770,"record_offset":23872,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":24080,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":23940,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":26500,"parent_record_offset":26472,"ancestor_record_offset":23700,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":26528,"parent_record_offset":26500,"ancestor_record_offset":23700,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":26560,"parent_record_offset":26528,"ancestor_record_offset":23700,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":26472,"parent_record_offset":26316,"ancestor_record_offset":23700,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":24816,"parent_record_offset":24788,"ancestor_record_offset":23700,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":24716,"parent_record_offset":24512,"ancestor_record_offset":23700,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":25132,"parent_record_offset":25100,"ancestor_record_offset":23700,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1770,"record_offset":25192,"parent_record_offset":25168,"ancestor_record_offset":23700,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1770,"record_offset":25300,"parent_record_offset":25240,"ancestor_record_offset":23700,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":25168,"parent_record_offset":25040,"ancestor_record_offset":23700,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":23700,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1770,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":23700,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":26408,"parent_record_offset":26380,"ancestor_record_offset":23700,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":25440,"parent_record_offset":25240,"ancestor_record_offset":23700,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":24844,"parent_record_offset":24788,"ancestor_record_offset":23700,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":25324,"parent_record_offset":25300,"ancestor_record_offset":23700,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":24424,"parent_record_offset":24388,"ancestor_record_offset":23700,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":24048,"parent_record_offset":24024,"ancestor_record_offset":23700,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":24544,"parent_record_offset":24512,"ancestor_record_offset":23700,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1770,"record_offset":23908,"parent_record_offset":23872,"ancestor_record_offset":23700,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":26164,"parent_record_offset":26140,"ancestor_record_offset":23700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":24632,"parent_record_offset":24512,"ancestor_record_offset":23700,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":26440,"parent_record_offset":26348,"ancestor_record_offset":23700,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":25272,"parent_record_offset":25240,"ancestor_record_offset":23700,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":24656,"parent_record_offset":24632,"ancestor_record_offset":23700,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":25240,"parent_record_offset":25216,"ancestor_record_offset":23700,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":23700,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":23700,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":25072,"parent_record_offset":25040,"ancestor_record_offset":23700,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":25040,"parent_record_offset":25012,"ancestor_record_offset":23700,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":25400,"parent_record_offset":25376,"ancestor_record_offset":23700,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":25100,"parent_record_offset":25072,"ancestor_record_offset":23700,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26348,"parent_record_offset":26316,"ancestor_record_offset":23700,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":23700,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26316,"parent_record_offset":26260,"ancestor_record_offset":23700,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26288,"parent_record_offset":26260,"ancestor_record_offset":23700,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26380,"parent_record_offset":26348,"ancestor_record_offset":23700,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":23700,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":25012,"parent_record_offset":24984,"ancestor_record_offset":23700,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":23700,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":24104,"parent_record_offset":24080,"ancestor_record_offset":23700,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":24152,"parent_record_offset":24128,"ancestor_record_offset":23700,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":24956,"parent_record_offset":24256,"ancestor_record_offset":23700,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":24984,"parent_record_offset":24256,"ancestor_record_offset":23700,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":24256,"parent_record_offset":24220,"ancestor_record_offset":23700,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":23700,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1770,"record_offset":24592,"parent_record_offset":24568,"ancestor_record_offset":23700,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1770,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":23700,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":24788,"parent_record_offset":24388,"ancestor_record_offset":23700,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":24304,"parent_record_offset":24256,"ancestor_record_offset":23700,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1770,"record_offset":24480,"parent_record_offset":24452,"ancestor_record_offset":23700,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1770,"record_offset":24512,"parent_record_offset":24480,"ancestor_record_offset":23700,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1770,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":23700,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1770,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":23700,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::discover_clutter_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":27548,"rva":28168912,"va":5396878032}
// 编译实例 RVAs: 0x1add2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12082302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e882a347ff","size":5,"soft_fail":false,"target_va":5384795984,"va":5396878281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12269196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e874c944ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396878599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21199535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e85185bcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396878218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-21199460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e89c85bcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396878159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28156012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e8945f52fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396878199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18088213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e8ebfeebfe","size":5,"soft_fail":false,"target_va":5378790752,"va":5396878960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20178807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e877e73301","size":5,"soft_fail":false,"target_va":5417057056,"va":5396878244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19742611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e8933f2d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396878178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12254550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e8aa0245ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396878625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::discover_mod_clutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14975913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e8577c1bff","size":5,"soft_fail":false,"target_va":5381902480,"va":5396878388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::mod_parts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13928651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396878032,"raw_hex":"e835772bff","size":5,"soft_fail":false,"target_va":5382949456,"va":5396878102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":29176,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":27720,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":27832,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":29232,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":29092,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1770,"record_offset":28228,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1770,"record_offset":29064,"parent_record_offset":27548,"ancestor_record_offset":27548,"inlinee":29159,"source_line":47,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28876,"parent_record_offset":28852,"ancestor_record_offset":27548,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28900,"parent_record_offset":28876,"ancestor_record_offset":27548,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28100,"parent_record_offset":28076,"ancestor_record_offset":27548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28124,"parent_record_offset":28100,"ancestor_record_offset":27548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28924,"parent_record_offset":28900,"ancestor_record_offset":27548,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28148,"parent_record_offset":28124,"ancestor_record_offset":27548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28852,"parent_record_offset":28696,"ancestor_record_offset":27548,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":28076,"parent_record_offset":28024,"ancestor_record_offset":27548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":27548,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28632,"parent_record_offset":28604,"ancestor_record_offset":27548,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28048,"parent_record_offset":28024,"ancestor_record_offset":27548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28024,"parent_record_offset":28000,"ancestor_record_offset":27548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28524,"parent_record_offset":28500,"ancestor_record_offset":27548,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28500,"parent_record_offset":28476,"ancestor_record_offset":27548,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28696,"parent_record_offset":28660,"ancestor_record_offset":27548,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28660,"parent_record_offset":28632,"ancestor_record_offset":27548,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28000,"parent_record_offset":27980,"ancestor_record_offset":27548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":28740,"parent_record_offset":28696,"ancestor_record_offset":27548,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1770,"record_offset":28568,"parent_record_offset":28388,"ancestor_record_offset":27548,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":27548,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":28428,"parent_record_offset":28388,"ancestor_record_offset":27548,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":28604,"parent_record_offset":28568,"ancestor_record_offset":27548,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":28356,"parent_record_offset":28324,"ancestor_record_offset":27548,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":28388,"parent_record_offset":28356,"ancestor_record_offset":27548,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1770,"record_offset":28324,"parent_record_offset":28292,"ancestor_record_offset":27548,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1770,"record_offset":28764,"parent_record_offset":28740,"ancestor_record_offset":27548,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":28812,"parent_record_offset":28788,"ancestor_record_offset":27548,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":28788,"parent_record_offset":28764,"ancestor_record_offset":27548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":27744,"parent_record_offset":27720,"ancestor_record_offset":27548,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":29200,"parent_record_offset":29176,"ancestor_record_offset":27548,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":28980,"parent_record_offset":28568,"ancestor_record_offset":27548,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":29004,"parent_record_offset":28980,"ancestor_record_offset":27548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":27856,"parent_record_offset":27832,"ancestor_record_offset":27548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":29116,"parent_record_offset":29092,"ancestor_record_offset":27548,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":29140,"parent_record_offset":29116,"ancestor_record_offset":27548,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":27548,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":27548,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":29304,"parent_record_offset":29280,"ancestor_record_offset":27548,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1770,"record_offset":28260,"parent_record_offset":28228,"ancestor_record_offset":27548,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1770,"record_offset":28292,"parent_record_offset":28260,"ancestor_record_offset":27548,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::load_clutter_packs_at_startup
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":79252,"rva":28190960,"va":5396900080}
// 编译实例 RVAs: 0x1ae28f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12291917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8b37044ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396901320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21222429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8e32bbcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396901112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-21222305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e85f2cbcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396901004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28178912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8200652fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396901099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27560537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8a7755bfe","size":5,"soft_fail":false,"target_va":5369339792,"va":5396900324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27390920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8380c5efe","size":5,"soft_fail":false,"target_va":5369512480,"va":5396903395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096cc40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18311174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8fa97e8fe","size":5,"soft_fail":false,"target_va":5378591808,"va":5396902977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140974340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18278516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e88c17e9fe","size":5,"soft_fail":false,"target_va":5378622272,"va":5396900783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18101560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8c8caebfe","size":5,"soft_fail":false,"target_va":5378801568,"va":5396903123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f9ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4100738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e87e6dc1ff","size":5,"soft_fail":false,"target_va":5392800928,"va":5396901661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417febc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3030888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e898c0d1ff","size":5,"soft_fail":false,"target_va":5393869760,"va":5396900643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180dca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2970098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e80eaed2ff","size":5,"soft_fail":false,"target_va":5393931424,"va":5396901517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a86010
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-380700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8e430faff","size":5,"soft_fail":false,"target_va":5396520976,"va":5396901671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141add6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e897a2ffff","size":5,"soft_fail":false,"target_va":5396879056,"va":5396902964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19452182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e816d12801","size":5,"soft_fail":false,"target_va":5416353040,"va":5396900853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19478824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e828392901","size":5,"soft_fail":false,"target_va":5416382848,"va":5396904019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19496949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8f57f2901","size":5,"soft_fail":false,"target_va":5416398672,"va":5396901718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20156679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e807913301","size":5,"soft_fail":false,"target_va":5417057056,"va":5396900372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19719655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8e7e52c01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396901134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13695638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e86a052fff","size":5,"soft_fail":false,"target_va":5383206064,"va":5396901697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12277357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e893a944ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396901432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::store::locked_store
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2998414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8723fd2ff","size":5,"soft_fail":false,"target_va":5393902624,"va":5396901033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::WorkshopSidecar::read
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14998820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8dc221bff","size":5,"soft_fail":false,"target_va":5381901760,"va":5396900575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15589890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8fe1d12ff","size":5,"soft_fail":false,"target_va":5381312992,"va":5396902877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::apply_built_clutter_packs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e825a9ffff","size":5,"soft_fail":false,"target_va":5396880032,"va":5396902262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::discover_clutter_part
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e887a8ffff","size":5,"soft_fail":false,"target_va":5396878032,"va":5396900420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::set_status
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15658478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8121211ff","size":5,"soft_fail":false,"target_va":5381242016,"va":5396900489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13996313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396900080,"raw_hex":"e8e76e2aff","size":5,"soft_fail":false,"target_va":5382903888,"va":5396900196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":82376,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":81376,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":79480,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":79508,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":79972,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":79624,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1770,"record_offset":79832,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] clutter_pack_pipe::build_clutter_packs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":1770,"record_offset":82460,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":35538,"source_line":490,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":82080,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":79772,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":80444,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":79888,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1770,"record_offset":83428,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::build_requests
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":81404,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":35536,"source_line":473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::mark_workshop_clutter_failed
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":82248,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":85984,"source_line":450,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::published_item_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":79800,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":85983,"source_line":466,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":1770,"record_offset":82488,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":82136,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":81996,"parent_record_offset":79252,"ancestor_record_offset":79252,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":81756,"parent_record_offset":81732,"ancestor_record_offset":79252,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":81780,"parent_record_offset":81756,"ancestor_record_offset":79252,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":80944,"parent_record_offset":80920,"ancestor_record_offset":79252,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":80968,"parent_record_offset":80944,"ancestor_record_offset":79252,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":81804,"parent_record_offset":81780,"ancestor_record_offset":79252,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":80992,"parent_record_offset":80968,"ancestor_record_offset":79252,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":81732,"parent_record_offset":81700,"ancestor_record_offset":79252,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":80920,"parent_record_offset":80832,"ancestor_record_offset":79252,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80188,"parent_record_offset":80160,"ancestor_record_offset":79252,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":81648,"parent_record_offset":81620,"ancestor_record_offset":79252,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80284,"parent_record_offset":80256,"ancestor_record_offset":79252,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80860,"parent_record_offset":80832,"ancestor_record_offset":79252,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80832,"parent_record_offset":80808,"ancestor_record_offset":79252,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80256,"parent_record_offset":80220,"ancestor_record_offset":79252,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80220,"parent_record_offset":80188,"ancestor_record_offset":79252,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":81700,"parent_record_offset":81672,"ancestor_record_offset":79252,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":81672,"parent_record_offset":81648,"ancestor_record_offset":79252,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":80808,"parent_record_offset":80784,"ancestor_record_offset":79252,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":82656,"parent_record_offset":82620,"ancestor_record_offset":79252,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":83456,"parent_record_offset":83428,"ancestor_record_offset":79252,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":82620,"parent_record_offset":82596,"ancestor_record_offset":79252,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":80112,"parent_record_offset":80068,"ancestor_record_offset":79252,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":81888,"parent_record_offset":81864,"ancestor_record_offset":79252,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":80376,"parent_record_offset":80112,"ancestor_record_offset":79252,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":80160,"parent_record_offset":80112,"ancestor_record_offset":79252,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":81592,"parent_record_offset":81548,"ancestor_record_offset":79252,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":81620,"parent_record_offset":81592,"ancestor_record_offset":79252,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":81476,"parent_record_offset":81440,"ancestor_record_offset":79252,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":80008,"parent_record_offset":79972,"ancestor_record_offset":79252,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":80508,"parent_record_offset":80472,"ancestor_record_offset":79252,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::forget_remaining_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":80408,"parent_record_offset":80068,"ancestor_record_offset":79252,"inlinee":6152,"source_line":162,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":80472,"parent_record_offset":80444,"ancestor_record_offset":79252,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":80760,"parent_record_offset":80736,"ancestor_record_offset":79252,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":80580,"parent_record_offset":80556,"ancestor_record_offset":79252,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1770,"record_offset":81864,"parent_record_offset":81548,"ancestor_record_offset":79252,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1770,"record_offset":80068,"parent_record_offset":79972,"ancestor_record_offset":79252,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1770,"record_offset":81512,"parent_record_offset":81476,"ancestor_record_offset":79252,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1770,"record_offset":81548,"parent_record_offset":81512,"ancestor_record_offset":79252,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1770,"record_offset":82832,"parent_record_offset":82808,"ancestor_record_offset":79252,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":1770,"record_offset":82808,"parent_record_offset":82784,"ancestor_record_offset":79252,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1770,"record_offset":79856,"parent_record_offset":79832,"ancestor_record_offset":79252,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1770,"record_offset":80556,"parent_record_offset":80532,"ancestor_record_offset":79252,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1770,"record_offset":80532,"parent_record_offset":80508,"ancestor_record_offset":79252,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":81440,"parent_record_offset":81404,"ancestor_record_offset":79252,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":81912,"parent_record_offset":81888,"ancestor_record_offset":79252,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":81936,"parent_record_offset":81912,"ancestor_record_offset":79252,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1770,"record_offset":79536,"parent_record_offset":79508,"ancestor_record_offset":79252,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1770,"record_offset":79560,"parent_record_offset":79536,"ancestor_record_offset":79252,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":84168,"parent_record_offset":84140,"ancestor_record_offset":79252,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":80888,"parent_record_offset":80860,"ancestor_record_offset":79252,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":82104,"parent_record_offset":82080,"ancestor_record_offset":79252,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1770,"record_offset":82536,"parent_record_offset":82488,"ancestor_record_offset":79252,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":80652,"parent_record_offset":80628,"ancestor_record_offset":79252,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":80704,"parent_record_offset":80472,"ancestor_record_offset":79252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":79592,"parent_record_offset":79508,"ancestor_record_offset":79252,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":80628,"parent_record_offset":80604,"ancestor_record_offset":79252,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":84200,"parent_record_offset":84108,"ancestor_record_offset":79252,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":79684,"parent_record_offset":79624,"ancestor_record_offset":79252,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":82688,"parent_record_offset":82656,"ancestor_record_offset":79252,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":80604,"parent_record_offset":80580,"ancestor_record_offset":79252,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":79712,"parent_record_offset":79624,"ancestor_record_offset":79252,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":79656,"parent_record_offset":79624,"ancestor_record_offset":79252,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":79736,"parent_record_offset":79712,"ancestor_record_offset":79252,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1770,"record_offset":79912,"parent_record_offset":79888,"ancestor_record_offset":79252,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":82284,"parent_record_offset":82248,"ancestor_record_offset":79252,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82400,"parent_record_offset":82376,"ancestor_record_offset":79252,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82860,"parent_record_offset":82832,"ancestor_record_offset":79252,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82020,"parent_record_offset":81996,"ancestor_record_offset":79252,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82424,"parent_record_offset":82400,"ancestor_record_offset":79252,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82044,"parent_record_offset":82020,"ancestor_record_offset":79252,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82716,"parent_record_offset":82656,"ancestor_record_offset":79252,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":82748,"parent_record_offset":82620,"ancestor_record_offset":79252,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":84108,"parent_record_offset":84076,"ancestor_record_offset":79252,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":84020,"parent_record_offset":83996,"ancestor_record_offset":79252,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":84076,"parent_record_offset":84020,"ancestor_record_offset":79252,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":84048,"parent_record_offset":84020,"ancestor_record_offset":79252,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":84140,"parent_record_offset":84108,"ancestor_record_offset":79252,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":83996,"parent_record_offset":83972,"ancestor_record_offset":79252,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":82184,"parent_record_offset":82160,"ancestor_record_offset":79252,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":82160,"parent_record_offset":82136,"ancestor_record_offset":79252,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":82208,"parent_record_offset":82184,"ancestor_record_offset":79252,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::resync_clutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":15360,"rva":28164640,"va":5396873760}
// 编译实例 RVAs: 0x1adc220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3314911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e8216bcdff","size":5,"soft_fail":false,"target_va":5393558928,"va":5396873834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141add6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e89c0f0000","size":5,"soft_fail":false,"target_va":5396879056,"va":5396875055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19747114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e82a512d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5396875121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17806861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e8f349f0fe","size":5,"soft_fail":false,"target_va":5379067728,"va":5396874584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::mesh_atlas::MeshAtlas::compact
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17261640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e8b89bf8fe","size":5,"soft_fail":false,"target_va":5379612352,"va":5396873987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15561376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873760,"raw_hex":"e8608d12ff","size":5,"soft_fail":false,"target_va":5381312992,"va":5396874363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":21660,"parent_record_offset":15360,"ancestor_record_offset":15360,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::resync_clutter_atlases
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":15508,"parent_record_offset":15360,"ancestor_record_offset":15360,"inlinee":85977,"source_line":239,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":15360,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":15360,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":15360,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":16364,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":15360,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":16068,"parent_record_offset":16040,"ancestor_record_offset":15360,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":19456,"parent_record_offset":18484,"ancestor_record_offset":15360,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":15888,"parent_record_offset":15864,"ancestor_record_offset":15360,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":16212,"parent_record_offset":16188,"ancestor_record_offset":15360,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1770,"record_offset":16320,"parent_record_offset":16296,"ancestor_record_offset":15360,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::compact_world_subset
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":15572,"parent_record_offset":15508,"ancestor_record_offset":15360,"inlinee":33175,"source_line":152,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":18484,"parent_record_offset":15508,"ancestor_record_offset":15360,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":15360,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":15360,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":15360,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":15980,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":15840,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":16392,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":15924,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1770,"record_offset":15784,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1770,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":15360,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":15360,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":15360,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":16248,"parent_record_offset":16140,"ancestor_record_offset":15360,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":15656,"parent_record_offset":15624,"ancestor_record_offset":15360,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":15360,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16484,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16164,"parent_record_offset":16140,"ancestor_record_offset":15360,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15360,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":18600,"parent_record_offset":18540,"ancestor_record_offset":15360,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15684,"parent_record_offset":15624,"ancestor_record_offset":15360,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":15360,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15812,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16140,"parent_record_offset":15732,"ancestor_record_offset":15360,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":18516,"parent_record_offset":18484,"ancestor_record_offset":15360,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":15360,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::resync_clutter_atlases
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FD9
// 首条位置: {"module":1770,"inlinee":85977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::compact_world_subset
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":15572,"parent_record_offset":15508,"ancestor_record_offset":15508,"inlinee":33175,"source_line":152,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":18484,"parent_record_offset":15508,"ancestor_record_offset":15508,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":15508,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":15508,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":15508,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":16364,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":15508,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":16068,"parent_record_offset":16040,"ancestor_record_offset":15508,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":19456,"parent_record_offset":18484,"ancestor_record_offset":15508,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":15888,"parent_record_offset":15864,"ancestor_record_offset":15508,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":16212,"parent_record_offset":16188,"ancestor_record_offset":15508,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1770,"record_offset":16320,"parent_record_offset":16296,"ancestor_record_offset":15508,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":15508,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":15508,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":15508,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":15980,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":15840,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":16392,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":15924,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1770,"record_offset":15784,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1770,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":15508,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":15508,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":15508,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":16248,"parent_record_offset":16140,"ancestor_record_offset":15508,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":15656,"parent_record_offset":15624,"ancestor_record_offset":15508,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":15508,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16484,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16164,"parent_record_offset":16140,"ancestor_record_offset":15508,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15508,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":18600,"parent_record_offset":18540,"ancestor_record_offset":15508,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15684,"parent_record_offset":15624,"ancestor_record_offset":15508,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":15508,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":15812,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":16140,"parent_record_offset":15732,"ancestor_record_offset":15508,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":18516,"parent_record_offset":18484,"ancestor_record_offset":15508,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":15508,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::resync_clutter_materials
// 记录形式: compiled_procedure; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":31116,"rva":28169936,"va":5396879056}
// 编译实例 RVAs: 0x1add6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21201204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e8cc7ebcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396879903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141542910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5877909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e86b4fa6ff","size":5,"soft_fail":false,"target_va":5391001872,"va":5396879776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ac8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3346380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e834f0ccff","size":5,"soft_fail":false,"target_va":5393533152,"va":5396879527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b4650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3314199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e8e96dcdff","size":5,"soft_fail":false,"target_va":5393565264,"va":5396879458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19472531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e893202901","size":5,"soft_fail":false,"target_va":5416352400,"va":5396879864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19473400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e8f8232901","size":5,"soft_fail":false,"target_va":5416353040,"va":5396879635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20177473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e841e23301","size":5,"soft_fail":false,"target_va":5417057056,"va":5396879578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20177930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e80ae43301","size":5,"soft_fail":false,"target_va":5417057088,"va":5396879153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19520762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396879056,"raw_hex":"e8fadc2901","size":5,"soft_fail":false,"target_va":5416400480,"va":5396879713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::nani::uber_materials::UberMaterialPool::rebuild
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/uber_materials.rs
//     evidence={"module":1770,"record_offset":31300,"parent_record_offset":31116,"ancestor_record_offset":31116,"inlinee":85978,"source_line":32,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterRegistry::material_meshes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":31272,"parent_record_offset":31116,"ancestor_record_offset":31116,"inlinee":32522,"source_line":335,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":34960,"parent_record_offset":34936,"ancestor_record_offset":31116,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":34984,"parent_record_offset":34960,"ancestor_record_offset":31116,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":35008,"parent_record_offset":34984,"ancestor_record_offset":31116,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":34936,"parent_record_offset":34844,"ancestor_record_offset":31116,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33664,"parent_record_offset":33624,"ancestor_record_offset":31116,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33488,"parent_record_offset":33460,"ancestor_record_offset":31116,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33572,"parent_record_offset":33548,"ancestor_record_offset":31116,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":34872,"parent_record_offset":34844,"ancestor_record_offset":31116,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":34844,"parent_record_offset":34820,"ancestor_record_offset":31116,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33548,"parent_record_offset":33516,"ancestor_record_offset":31116,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33712,"parent_record_offset":33688,"ancestor_record_offset":31116,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33688,"parent_record_offset":33664,"ancestor_record_offset":31116,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":33516,"parent_record_offset":33488,"ancestor_record_offset":31116,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":34820,"parent_record_offset":34796,"ancestor_record_offset":31116,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33416,"parent_record_offset":33388,"ancestor_record_offset":31116,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33908,"parent_record_offset":33872,"ancestor_record_offset":31116,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33360,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33792,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33752,"parent_record_offset":33416,"ancestor_record_offset":31116,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":31808,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33848,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33872,"parent_record_offset":33848,"ancestor_record_offset":31116,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":33460,"parent_record_offset":33416,"ancestor_record_offset":31116,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1770,"record_offset":33388,"parent_record_offset":33360,"ancestor_record_offset":31116,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1770,"record_offset":34504,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1770,"record_offset":34528,"parent_record_offset":34504,"ancestor_record_offset":31116,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1770,"record_offset":34592,"parent_record_offset":34552,"ancestor_record_offset":31116,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":32756,"parent_record_offset":32732,"ancestor_record_offset":31116,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":32324,"parent_record_offset":32300,"ancestor_record_offset":31116,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":32408,"parent_record_offset":32384,"ancestor_record_offset":31116,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":32880,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1770,"record_offset":31864,"parent_record_offset":31836,"ancestor_record_offset":31116,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1770,"record_offset":32068,"parent_record_offset":31948,"ancestor_record_offset":31116,"inlinee":5470,"source_line":49,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1770,"record_offset":31920,"parent_record_offset":31892,"ancestor_record_offset":31116,"inlinee":6890,"source_line":132,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1770,"record_offset":31836,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":31892,"parent_record_offset":31864,"ancestor_record_offset":31116,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":33112,"parent_record_offset":32068,"ancestor_record_offset":31116,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":31948,"parent_record_offset":31920,"ancestor_record_offset":31116,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1770,"record_offset":34616,"parent_record_offset":34592,"ancestor_record_offset":31116,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":34728,"parent_record_offset":34704,"ancestor_record_offset":31116,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":34904,"parent_record_offset":34872,"ancestor_record_offset":31116,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":32556,"parent_record_offset":32528,"ancestor_record_offset":31116,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":32528,"parent_record_offset":32500,"ancestor_record_offset":31116,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":33136,"parent_record_offset":33112,"ancestor_record_offset":31116,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":33268,"parent_record_offset":33244,"ancestor_record_offset":31116,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1770,"record_offset":34704,"parent_record_offset":34552,"ancestor_record_offset":31116,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1770,"record_offset":34552,"parent_record_offset":34528,"ancestor_record_offset":31116,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":33244,"parent_record_offset":31836,"ancestor_record_offset":31116,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":34640,"parent_record_offset":34616,"ancestor_record_offset":31116,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":32348,"parent_record_offset":32324,"ancestor_record_offset":31116,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":31452,"parent_record_offset":31424,"ancestor_record_offset":31116,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":33624,"parent_record_offset":33416,"ancestor_record_offset":31116,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":31116,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":31684,"parent_record_offset":31660,"ancestor_record_offset":31116,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":32024,"parent_record_offset":31988,"ancestor_record_offset":31116,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":34664,"parent_record_offset":34640,"ancestor_record_offset":31116,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":34020,"parent_record_offset":33872,"ancestor_record_offset":31116,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":31708,"parent_record_offset":31684,"ancestor_record_offset":31116,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1770,"record_offset":34324,"parent_record_offset":34300,"ancestor_record_offset":31116,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1770,"record_offset":34432,"parent_record_offset":34348,"ancestor_record_offset":31116,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":34300,"parent_record_offset":34056,"ancestor_record_offset":31116,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":31116,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":34348,"parent_record_offset":34324,"ancestor_record_offset":31116,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":31988,"parent_record_offset":31948,"ancestor_record_offset":31116,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorld::mark_material_args_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1770,"record_offset":34056,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":30850,"source_line":585,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1770,"record_offset":34456,"parent_record_offset":34432,"ancestor_record_offset":31116,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::instance_data::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/instance_data.rs
//     evidence={"module":1770,"record_offset":33820,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":35556,"source_line":325,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":32780,"parent_record_offset":32756,"ancestor_record_offset":31116,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$4::material_meshes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":32100,"parent_record_offset":32068,"ancestor_record_offset":31116,"inlinee":85979,"source_line":340,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$4::material_meshes::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":33160,"parent_record_offset":33136,"ancestor_record_offset":31116,"inlinee":81021,"source_line":341,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$4::material_meshes::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":33292,"parent_record_offset":33268,"ancestor_record_offset":31116,"inlinee":85980,"source_line":342,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":32468,"parent_record_offset":32440,"ancestor_record_offset":31116,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":32500,"parent_record_offset":32468,"ancestor_record_offset":31116,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":32440,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":32300,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":32908,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":32384,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1770,"record_offset":33032,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1770,"record_offset":31660,"parent_record_offset":31600,"ancestor_record_offset":31116,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1770,"record_offset":32732,"parent_record_offset":32708,"ancestor_record_offset":31116,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":31116,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":32156,"parent_record_offset":32128,"ancestor_record_offset":31116,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":32708,"parent_record_offset":32600,"ancestor_record_offset":31116,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":33072,"parent_record_offset":32156,"ancestor_record_offset":31116,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":31116,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":33000,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":31116,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31424,"parent_record_offset":31400,"ancestor_record_offset":31116,"inlinee":12661,"source_line":872,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31516,"parent_record_offset":31496,"ancestor_record_offset":31116,"inlinee":12651,"source_line":866,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32216,"parent_record_offset":32188,"ancestor_record_offset":31116,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32188,"parent_record_offset":32156,"ancestor_record_offset":31116,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31116,"inlinee":5023,"source_line":3207,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31600,"parent_record_offset":31540,"ancestor_record_offset":31116,"inlinee":5024,"source_line":2669,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32244,"parent_record_offset":32216,"ancestor_record_offset":31116,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31572,"parent_record_offset":31540,"ancestor_record_offset":31116,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31632,"parent_record_offset":31600,"ancestor_record_offset":31116,"inlinee":5025,"source_line":2691,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31748,"parent_record_offset":31540,"ancestor_record_offset":31116,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":31116,"inlinee":12650,"source_line":878,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":32600,"parent_record_offset":32244,"ancestor_record_offset":31116,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\scopeguard.rs
//     evidence={"module":1770,"record_offset":31472,"parent_record_offset":31452,"ancestor_record_offset":31116,"inlinee":12554,"source_line":69,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":31376,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":32128,"parent_record_offset":32100,"ancestor_record_offset":31116,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":33332,"parent_record_offset":31300,"ancestor_record_offset":31116,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1770,"record_offset":32804,"parent_record_offset":32780,"ancestor_record_offset":31116,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::install_built_clutter_packs
// 记录形式: compiled_procedure; instances=1
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":76724,"rva":28190320,"va":5396899440}
// 编译实例 RVAs: 0x1ae2670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21221293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e85330bcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396899992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27676006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e89ab259fe","size":5,"soft_fail":false,"target_va":5369224048,"va":5396900049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14179c570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3433212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e8049dcbff","size":5,"soft_fail":false,"target_va":5393466736,"va":5396899943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14179d6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3428643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e8ddaecbff","size":5,"soft_fail":false,"target_va":5393471152,"va":5396899790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::apply_built_clutter_packs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e8b2b3ffff","size":5,"soft_fail":false,"target_va":5396880032,"va":5396899561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_clutter_packs::resync_clutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396899440,"raw_hex":"e8d19affff","size":5,"soft_fail":false,"target_va":5396873760,"va":5396899658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":76900,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":78000,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":16567,"source_line":225,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":78760,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1770,"record_offset":76928,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::intersection
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":76956,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":12124,"source_line":713,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":78112,"parent_record_offset":76724,"ancestor_record_offset":76724,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":79120,"parent_record_offset":79092,"ancestor_record_offset":76724,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":79148,"parent_record_offset":79120,"ancestor_record_offset":76724,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":79180,"parent_record_offset":79148,"ancestor_record_offset":76724,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":79092,"parent_record_offset":78936,"ancestor_record_offset":76724,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":77880,"parent_record_offset":77852,"ancestor_record_offset":76724,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":77932,"parent_record_offset":77908,"ancestor_record_offset":76724,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":78640,"parent_record_offset":78612,"ancestor_record_offset":76724,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":77308,"parent_record_offset":77284,"ancestor_record_offset":76724,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":78784,"parent_record_offset":78760,"ancestor_record_offset":76724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":78700,"parent_record_offset":78580,"ancestor_record_offset":76724,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":77852,"parent_record_offset":77764,"ancestor_record_offset":76724,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":77908,"parent_record_offset":77852,"ancestor_record_offset":76724,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":77180,"parent_record_offset":77152,"ancestor_record_offset":76724,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":77152,"parent_record_offset":77128,"ancestor_record_offset":76724,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":77040,"parent_record_offset":77016,"ancestor_record_offset":76724,"inlinee":12127,"source_line":828,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":78208,"parent_record_offset":78184,"ancestor_record_offset":76724,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":77284,"parent_record_offset":77228,"ancestor_record_offset":76724,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78580,"parent_record_offset":78548,"ancestor_record_offset":76724,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78548,"parent_record_offset":78232,"ancestor_record_offset":76724,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":77204,"parent_record_offset":77180,"ancestor_record_offset":76724,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":77064,"parent_record_offset":77040,"ancestor_record_offset":76724,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78968,"parent_record_offset":78936,"ancestor_record_offset":76724,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":77256,"parent_record_offset":77228,"ancestor_record_offset":76724,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78880,"parent_record_offset":78856,"ancestor_record_offset":76724,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78936,"parent_record_offset":78880,"ancestor_record_offset":76724,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78908,"parent_record_offset":78880,"ancestor_record_offset":76724,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":77228,"parent_record_offset":77204,"ancestor_record_offset":76724,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78612,"parent_record_offset":78580,"ancestor_record_offset":76724,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78856,"parent_record_offset":78832,"ancestor_record_offset":76724,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":78232,"parent_record_offset":78208,"ancestor_record_offset":76724,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":77128,"parent_record_offset":77104,"ancestor_record_offset":76724,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":77016,"parent_record_offset":76992,"ancestor_record_offset":76724,"inlinee":12126,"source_line":309,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":78136,"parent_record_offset":78112,"ancestor_record_offset":76724,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":78184,"parent_record_offset":78160,"ancestor_record_offset":76724,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::fold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":78072,"parent_record_offset":78048,"ancestor_record_offset":76724,"inlinee":27448,"source_line":1806,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":77104,"parent_record_offset":76956,"ancestor_record_offset":76724,"inlinee":12128,"source_line":349,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":76992,"parent_record_offset":76956,"ancestor_record_offset":76724,"inlinee":12125,"source_line":367,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":78160,"parent_record_offset":78136,"ancestor_record_offset":76724,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":78048,"parent_record_offset":78024,"ancestor_record_offset":76724,"inlinee":27447,"source_line":1688,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$39::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":78024,"parent_record_offset":78000,"ancestor_record_offset":76724,"inlinee":85982,"source_line":1857,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::unload_clutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":21692,"rva":28166016,"va":5396875136}
// 编译实例 RVAs: 0x1adc780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-21196622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8b290bcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396875321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c05040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15562716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8248812ff","size":5,"soft_fail":false,"target_va":5381312576,"va":5396875287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca5350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14906666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8d68a1cff","size":5,"soft_fail":false,"target_va":5381968720,"va":5396875381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160d480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5043018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8b60cb3ff","size":5,"soft_fail":false,"target_va":5391832192,"va":5396875205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b4650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3310100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8ec7dcdff","size":5,"soft_fail":false,"target_va":5393565264,"va":5396875359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::unregister_external
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17462236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396875136,"raw_hex":"e8248cf5fe","size":5,"soft_fail":false,"target_va":5379412992,"va":5396875223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":22024,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":21940,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":22172,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":21856,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":22544,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":22600,"parent_record_offset":21692,"ancestor_record_offset":21692,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":22428,"parent_record_offset":22404,"ancestor_record_offset":21692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":22452,"parent_record_offset":22428,"ancestor_record_offset":21692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":21692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":22404,"parent_record_offset":22316,"ancestor_record_offset":21692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":22344,"parent_record_offset":22316,"ancestor_record_offset":21692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":22316,"parent_record_offset":22292,"ancestor_record_offset":21692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":21692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":21964,"parent_record_offset":21940,"ancestor_record_offset":21692,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":22196,"parent_record_offset":22172,"ancestor_record_offset":21692,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1770,"record_offset":22244,"parent_record_offset":22220,"ancestor_record_offset":21692,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1770,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":21692,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":22372,"parent_record_offset":22344,"ancestor_record_offset":21692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":21692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":21904,"parent_record_offset":21880,"ancestor_record_offset":21692,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":22084,"parent_record_offset":22024,"ancestor_record_offset":21692,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":22112,"parent_record_offset":22024,"ancestor_record_offset":21692,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":22056,"parent_record_offset":22024,"ancestor_record_offset":21692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":22136,"parent_record_offset":22112,"ancestor_record_offset":21692,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1770,"record_offset":21880,"parent_record_offset":21856,"ancestor_record_offset":21692,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":22624,"parent_record_offset":22600,"ancestor_record_offset":21692,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":21692,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":22676,"parent_record_offset":22648,"ancestor_record_offset":21692,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::unload_clutter::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 353
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DC7
// 首条位置: {"module":1643,"inlinee":81351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterRegistry::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":603712,"parent_record_offset":603688,"ancestor_record_offset":603688,"inlinee":32530,"source_line":395,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":604368,"parent_record_offset":604344,"ancestor_record_offset":603688,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":603936,"parent_record_offset":603912,"ancestor_record_offset":603688,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":604020,"parent_record_offset":603996,"ancestor_record_offset":603688,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":604492,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":604168,"parent_record_offset":604140,"ancestor_record_offset":603688,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":604140,"parent_record_offset":604112,"ancestor_record_offset":603688,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":603960,"parent_record_offset":603936,"ancestor_record_offset":603688,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":604284,"parent_record_offset":604260,"ancestor_record_offset":603688,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":604440,"parent_record_offset":604416,"ancestor_record_offset":603688,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":604392,"parent_record_offset":604368,"ancestor_record_offset":603688,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":604080,"parent_record_offset":604052,"ancestor_record_offset":603688,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":604112,"parent_record_offset":604080,"ancestor_record_offset":603688,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":604052,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":603912,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":604520,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":603996,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1643,"record_offset":604644,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1643,"record_offset":604344,"parent_record_offset":604320,"ancestor_record_offset":603688,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":603768,"parent_record_offset":603740,"ancestor_record_offset":603688,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":604320,"parent_record_offset":604212,"ancestor_record_offset":603688,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":604712,"parent_record_offset":603768,"ancestor_record_offset":603688,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":604260,"parent_record_offset":604236,"ancestor_record_offset":603688,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":604612,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":604236,"parent_record_offset":604212,"ancestor_record_offset":603688,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":603828,"parent_record_offset":603800,"ancestor_record_offset":603688,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":603800,"parent_record_offset":603768,"ancestor_record_offset":603688,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":603856,"parent_record_offset":603828,"ancestor_record_offset":603688,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":604672,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":604212,"parent_record_offset":603856,"ancestor_record_offset":603688,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1643,"record_offset":603740,"parent_record_offset":603712,"ancestor_record_offset":603688,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1643,"record_offset":604416,"parent_record_offset":604392,"ancestor_record_offset":603688,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::unload_clutter::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DC8
// 首条位置: {"module":1643,"inlinee":81352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::InFlightPackBuild::spawn
// 记录形式: compiled_procedure; instances=1
// 源码行: 390
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":26688,"rva":28168336,"va":5396877456}
// 编译实例 RVAs: 0x1add090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21198859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e8f587bcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396877542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28155342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e8326252fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396877529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27365369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e807705efe","size":5,"soft_fail":false,"target_va":5369512480,"va":5396877844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096cc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18286212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e87cf9e8fe","size":5,"soft_fail":false,"target_va":5378591808,"va":5396878015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141744a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3770289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e84f78c6ff","size":5,"soft_fail":false,"target_va":5393107472,"va":5396877756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180dca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2946227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e84d0bd3ff","size":5,"soft_fail":false,"target_va":5393931424,"va":5396877646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19743222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e8f6412d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396877567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13240376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e8c8f735ff","size":5,"soft_fail":false,"target_va":5383637632,"va":5396878003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::AsyncComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12294460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396877456,"raw_hex":"e8c46644ff","size":5,"soft_fail":false,"target_va":5384583232,"va":5396877687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":27392,"parent_record_offset":26688,"ancestor_record_offset":26688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::build_requests
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":26844,"parent_record_offset":26688,"ancestor_record_offset":26688,"inlinee":35536,"source_line":473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":27152,"parent_record_offset":27128,"ancestor_record_offset":26688,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":27176,"parent_record_offset":27152,"ancestor_record_offset":26688,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":27200,"parent_record_offset":27176,"ancestor_record_offset":26688,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":27128,"parent_record_offset":27096,"ancestor_record_offset":26688,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":27044,"parent_record_offset":27020,"ancestor_record_offset":26688,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":27096,"parent_record_offset":27068,"ancestor_record_offset":26688,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":27068,"parent_record_offset":27044,"ancestor_record_offset":26688,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1770,"record_offset":27416,"parent_record_offset":27392,"ancestor_record_offset":26688,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":26688,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":26996,"parent_record_offset":26956,"ancestor_record_offset":26688,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":27020,"parent_record_offset":26996,"ancestor_record_offset":26688,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":26900,"parent_record_offset":26872,"ancestor_record_offset":26688,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1770,"record_offset":27260,"parent_record_offset":26956,"ancestor_record_offset":26688,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1770,"record_offset":26928,"parent_record_offset":26900,"ancestor_record_offset":26688,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1770,"record_offset":26956,"parent_record_offset":26928,"ancestor_record_offset":26688,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":26872,"parent_record_offset":26844,"ancestor_record_offset":26688,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":26688,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":27332,"parent_record_offset":27308,"ancestor_record_offset":26688,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":27452,"parent_record_offset":27416,"ancestor_record_offset":26688,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":27476,"parent_record_offset":27452,"ancestor_record_offset":26688,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":27508,"parent_record_offset":27416,"ancestor_record_offset":26688,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CEDD
// 首条位置: {"module":3252,"inlinee":118493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] clutter_pack_pipe::build_clutter_packs::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":3252,"record_offset":115260,"parent_record_offset":115148,"ancestor_record_offset":115148,"inlinee":81604,"source_line":495,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":150944,"parent_record_offset":115148,"ancestor_record_offset":115148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":116152,"parent_record_offset":116128,"ancestor_record_offset":115148,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":116176,"parent_record_offset":116152,"ancestor_record_offset":115148,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":119744,"parent_record_offset":119720,"ancestor_record_offset":115148,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":119768,"parent_record_offset":119744,"ancestor_record_offset":115148,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":116200,"parent_record_offset":116176,"ancestor_record_offset":115148,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":119792,"parent_record_offset":119768,"ancestor_record_offset":115148,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":116128,"parent_record_offset":116096,"ancestor_record_offset":115148,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3252,"record_offset":119720,"parent_record_offset":119664,"ancestor_record_offset":115148,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3252,"record_offset":116096,"parent_record_offset":116060,"ancestor_record_offset":115148,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3252,"record_offset":116060,"parent_record_offset":116032,"ancestor_record_offset":115148,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":3252,"record_offset":120684,"parent_record_offset":120656,"ancestor_record_offset":115148,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3252,"record_offset":118916,"parent_record_offset":118892,"ancestor_record_offset":115148,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":3252,"record_offset":118964,"parent_record_offset":118940,"ancestor_record_offset":115148,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118568,"parent_record_offset":118536,"ancestor_record_offset":115148,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118168,"parent_record_offset":118136,"ancestor_record_offset":115148,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":119692,"parent_record_offset":119664,"ancestor_record_offset":115148,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":119664,"parent_record_offset":119640,"ancestor_record_offset":115148,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118616,"parent_record_offset":118592,"ancestor_record_offset":115148,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118592,"parent_record_offset":118568,"ancestor_record_offset":115148,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118228,"parent_record_offset":118196,"ancestor_record_offset":115148,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":118196,"parent_record_offset":118168,"ancestor_record_offset":115148,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":119640,"parent_record_offset":119616,"ancestor_record_offset":115148,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3252,"record_offset":119252,"parent_record_offset":119212,"ancestor_record_offset":115148,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3252,"record_offset":132620,"parent_record_offset":132592,"ancestor_record_offset":115148,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3252,"record_offset":132648,"parent_record_offset":132620,"ancestor_record_offset":115148,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3252,"record_offset":138816,"parent_record_offset":138756,"ancestor_record_offset":115148,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3252,"record_offset":132676,"parent_record_offset":132648,"ancestor_record_offset":115148,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3252,"record_offset":132592,"parent_record_offset":132564,"ancestor_record_offset":115148,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3252,"record_offset":132564,"parent_record_offset":132536,"ancestor_record_offset":115148,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3252,"record_offset":121328,"parent_record_offset":121276,"ancestor_record_offset":115148,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3252,"record_offset":121408,"parent_record_offset":121384,"ancestor_record_offset":115148,"inlinee":4606,"source_line":2711,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3252,"record_offset":121240,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3252,"record_offset":121276,"parent_record_offset":121240,"ancestor_record_offset":115148,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3252,"record_offset":122500,"parent_record_offset":122464,"ancestor_record_offset":115148,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3252,"record_offset":116032,"parent_record_offset":115988,"ancestor_record_offset":115148,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3252,"record_offset":115904,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3252,"record_offset":141416,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":3252,"record_offset":116280,"parent_record_offset":116256,"ancestor_record_offset":115148,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":118536,"parent_record_offset":118484,"ancestor_record_offset":115148,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":138788,"parent_record_offset":138756,"ancestor_record_offset":115148,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":116832,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":118448,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":118484,"parent_record_offset":118448,"ancestor_record_offset":115148,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":118104,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":118136,"parent_record_offset":118104,"ancestor_record_offset":115148,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":138732,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":116772,"parent_record_offset":116748,"ancestor_record_offset":115148,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":117024,"parent_record_offset":117000,"ancestor_record_offset":115148,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":126028,"parent_record_offset":126004,"ancestor_record_offset":115148,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3252,"record_offset":138756,"parent_record_offset":138732,"ancestor_record_offset":115148,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3252,"record_offset":123172,"parent_record_offset":123140,"ancestor_record_offset":115148,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3252,"record_offset":123140,"parent_record_offset":123112,"ancestor_record_offset":115148,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3252,"record_offset":123392,"parent_record_offset":123368,"ancestor_record_offset":115148,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3252,"record_offset":118692,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":3252,"record_offset":123244,"parent_record_offset":123220,"ancestor_record_offset":115148,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":3252,"record_offset":119028,"parent_record_offset":119004,"ancestor_record_offset":115148,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":3252,"record_offset":119004,"parent_record_offset":118852,"ancestor_record_offset":115148,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":3252,"record_offset":118852,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::build_texture_assets::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3252,"record_offset":117348,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":81612,"source_line":231,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::build_texture_assets::async_fn$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3252,"record_offset":118892,"parent_record_offset":118852,"ancestor_record_offset":115148,"inlinee":70353,"source_line":239,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":3252,"record_offset":152456,"parent_record_offset":152432,"ancestor_record_offset":115148,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":3252,"record_offset":152432,"parent_record_offset":152408,"ancestor_record_offset":115148,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::build_object_textures::async_fn$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":3252,"record_offset":128852,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":81616,"source_line":630,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/lib.rs
//     evidence={"module":3252,"record_offset":134864,"parent_record_offset":134840,"ancestor_record_offset":115148,"inlinee":81569,"source_line":71,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3252,"record_offset":120716,"parent_record_offset":120684,"ancestor_record_offset":115148,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3252,"record_offset":119276,"parent_record_offset":119252,"ancestor_record_offset":115148,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":3252,"record_offset":133736,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3252,"record_offset":133760,"parent_record_offset":133736,"ancestor_record_offset":115148,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":3252,"record_offset":133960,"parent_record_offset":133784,"ancestor_record_offset":115148,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3252,"record_offset":129684,"parent_record_offset":129656,"ancestor_record_offset":115148,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3252,"record_offset":129584,"parent_record_offset":129380,"ancestor_record_offset":115148,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3252,"record_offset":116496,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3252,"record_offset":134816,"parent_record_offset":134792,"ancestor_record_offset":115148,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3252,"record_offset":131068,"parent_record_offset":131044,"ancestor_record_offset":115148,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3252,"record_offset":134384,"parent_record_offset":134360,"ancestor_record_offset":115148,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3252,"record_offset":134468,"parent_record_offset":134444,"ancestor_record_offset":115148,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":3252,"record_offset":121356,"parent_record_offset":121276,"ancestor_record_offset":115148,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":3252,"record_offset":117084,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":3252,"record_offset":140636,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":3252,"record_offset":132508,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3252,"record_offset":134912,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3252,"record_offset":119324,"parent_record_offset":119300,"ancestor_record_offset":115148,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3252,"record_offset":139276,"parent_record_offset":139244,"ancestor_record_offset":115148,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":3252,"record_offset":139244,"parent_record_offset":138816,"ancestor_record_offset":115148,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3252,"record_offset":137468,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":3252,"record_offset":137496,"parent_record_offset":137468,"ancestor_record_offset":115148,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":3252,"record_offset":116796,"parent_record_offset":116772,"ancestor_record_offset":115148,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3252,"record_offset":123220,"parent_record_offset":123196,"ancestor_record_offset":115148,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3252,"record_offset":123196,"parent_record_offset":123172,"ancestor_record_offset":115148,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3252,"record_offset":116748,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3252,"record_offset":117000,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3252,"record_offset":120780,"parent_record_offset":120744,"ancestor_record_offset":115148,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3252,"record_offset":139392,"parent_record_offset":138816,"ancestor_record_offset":115148,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3252,"record_offset":129548,"parent_record_offset":129524,"ancestor_record_offset":115148,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3252,"record_offset":120744,"parent_record_offset":120716,"ancestor_record_offset":115148,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3252,"record_offset":119300,"parent_record_offset":119276,"ancestor_record_offset":115148,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3252,"record_offset":132024,"parent_record_offset":131996,"ancestor_record_offset":115148,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3252,"record_offset":133928,"parent_record_offset":133904,"ancestor_record_offset":115148,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3252,"record_offset":123536,"parent_record_offset":123504,"ancestor_record_offset":115148,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3252,"record_offset":129712,"parent_record_offset":129656,"ancestor_record_offset":115148,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3252,"record_offset":134616,"parent_record_offset":134588,"ancestor_record_offset":115148,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":3252,"record_offset":131288,"parent_record_offset":131264,"ancestor_record_offset":115148,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3252,"record_offset":129292,"parent_record_offset":129256,"ancestor_record_offset":115148,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3252,"record_offset":133868,"parent_record_offset":133844,"ancestor_record_offset":115148,"inlinee":5089,"source_line":49,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3252,"record_offset":133904,"parent_record_offset":133784,"ancestor_record_offset":115148,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3252,"record_offset":133784,"parent_record_offset":133760,"ancestor_record_offset":115148,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":3252,"record_offset":133844,"parent_record_offset":133820,"ancestor_record_offset":115148,"inlinee":8703,"source_line":220,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3252,"record_offset":116520,"parent_record_offset":116496,"ancestor_record_offset":115148,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3252,"record_offset":118940,"parent_record_offset":118916,"ancestor_record_offset":115148,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3252,"record_offset":129412,"parent_record_offset":129380,"ancestor_record_offset":115148,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3252,"record_offset":137776,"parent_record_offset":137496,"ancestor_record_offset":115148,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":3252,"record_offset":117112,"parent_record_offset":117084,"ancestor_record_offset":115148,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":3252,"record_offset":117136,"parent_record_offset":117112,"ancestor_record_offset":115148,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3252,"record_offset":131100,"parent_record_offset":131016,"ancestor_record_offset":115148,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3252,"record_offset":133088,"parent_record_offset":132676,"ancestor_record_offset":115148,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3252,"record_offset":123316,"parent_record_offset":123292,"ancestor_record_offset":115148,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3252,"record_offset":134044,"parent_record_offset":133984,"ancestor_record_offset":115148,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":133112,"parent_record_offset":133088,"ancestor_record_offset":115148,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=209
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":116256,"parent_record_offset":115988,"ancestor_record_offset":115148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3252,"record_offset":116712,"parent_record_offset":116688,"ancestor_record_offset":115148,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3252,"record_offset":131556,"parent_record_offset":131532,"ancestor_record_offset":115148,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3252,"record_offset":123292,"parent_record_offset":123268,"ancestor_record_offset":115148,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3252,"record_offset":129500,"parent_record_offset":129380,"ancestor_record_offset":115148,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3252,"record_offset":131408,"parent_record_offset":131384,"ancestor_record_offset":115148,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3252,"record_offset":118764,"parent_record_offset":118692,"ancestor_record_offset":115148,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3252,"record_offset":122532,"parent_record_offset":122500,"ancestor_record_offset":115148,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3252,"record_offset":123268,"parent_record_offset":123244,"ancestor_record_offset":115148,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3252,"record_offset":118792,"parent_record_offset":118692,"ancestor_record_offset":115148,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3252,"record_offset":118732,"parent_record_offset":118692,"ancestor_record_offset":115148,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":118816,"parent_record_offset":118792,"ancestor_record_offset":115148,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":129524,"parent_record_offset":129500,"ancestor_record_offset":115148,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3252,"record_offset":118656,"parent_record_offset":118484,"ancestor_record_offset":115148,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3252,"record_offset":121440,"parent_record_offset":121276,"ancestor_record_offset":115148,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3252,"record_offset":121212,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3252,"record_offset":124680,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3252,"record_offset":133984,"parent_record_offset":133960,"ancestor_record_offset":115148,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3252,"record_offset":116664,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3252,"record_offset":134016,"parent_record_offset":133984,"ancestor_record_offset":115148,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3252,"record_offset":116688,"parent_record_offset":116664,"ancestor_record_offset":115148,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3252,"record_offset":129944,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3252,"record_offset":129832,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":115928,"parent_record_offset":115904,"ancestor_record_offset":115148,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":116316,"parent_record_offset":116280,"ancestor_record_offset":115148,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":116436,"parent_record_offset":116412,"ancestor_record_offset":115148,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":115952,"parent_record_offset":115928,"ancestor_record_offset":115148,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":116460,"parent_record_offset":116436,"ancestor_record_offset":115148,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":116340,"parent_record_offset":116316,"ancestor_record_offset":115148,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3252,"record_offset":116372,"parent_record_offset":116280,"ancestor_record_offset":115148,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":3252,"record_offset":134840,"parent_record_offset":134816,"ancestor_record_offset":115148,"inlinee":14084,"source_line":30,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3252,"record_offset":131232,"parent_record_offset":131204,"ancestor_record_offset":115148,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3252,"record_offset":131264,"parent_record_offset":131232,"ancestor_record_offset":115148,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":3252,"record_offset":131204,"parent_record_offset":131164,"ancestor_record_offset":115148,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3252,"record_offset":134360,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3252,"record_offset":131044,"parent_record_offset":131016,"ancestor_record_offset":115148,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3252,"record_offset":134940,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":3252,"record_offset":134444,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":3252,"record_offset":134304,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":3252,"record_offset":134792,"parent_record_offset":134768,"ancestor_record_offset":115148,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3252,"record_offset":134144,"parent_record_offset":134120,"ancestor_record_offset":115148,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3252,"record_offset":134768,"parent_record_offset":134660,"ancestor_record_offset":115148,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":3252,"record_offset":134172,"parent_record_offset":134144,"ancestor_record_offset":115148,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131532,"parent_record_offset":130928,"ancestor_record_offset":115148,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134708,"parent_record_offset":134684,"ancestor_record_offset":115148,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131384,"parent_record_offset":131164,"ancestor_record_offset":115148,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":135032,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131016,"parent_record_offset":130960,"ancestor_record_offset":115148,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131164,"parent_record_offset":131136,"ancestor_record_offset":115148,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134684,"parent_record_offset":134660,"ancestor_record_offset":115148,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134224,"parent_record_offset":134200,"ancestor_record_offset":115148,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134200,"parent_record_offset":134144,"ancestor_record_offset":115148,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131936,"parent_record_offset":131904,"ancestor_record_offset":115148,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130988,"parent_record_offset":130960,"ancestor_record_offset":115148,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130928,"parent_record_offset":130864,"ancestor_record_offset":115148,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130864,"parent_record_offset":130836,"ancestor_record_offset":115148,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134248,"parent_record_offset":134224,"ancestor_record_offset":115148,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131904,"parent_record_offset":130864,"ancestor_record_offset":115148,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130900,"parent_record_offset":130864,"ancestor_record_offset":115148,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130960,"parent_record_offset":130928,"ancestor_record_offset":115148,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134332,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131968,"parent_record_offset":131936,"ancestor_record_offset":115148,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":130836,"parent_record_offset":130808,"ancestor_record_offset":115148,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":131136,"parent_record_offset":130928,"ancestor_record_offset":115148,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":3252,"record_offset":134660,"parent_record_offset":134248,"ancestor_record_offset":115148,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3252,"record_offset":116600,"parent_record_offset":116576,"ancestor_record_offset":115148,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3252,"record_offset":116552,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3252,"record_offset":116576,"parent_record_offset":116552,"ancestor_record_offset":115148,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3252,"record_offset":116624,"parent_record_offset":116600,"ancestor_record_offset":115148,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3252,"record_offset":116412,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::relative_path_buf::impl$19::fmt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":3252,"record_offset":121384,"parent_record_offset":121276,"ancestor_record_offset":115148,"inlinee":70443,"source_line":464,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::relative_path_buf::impl$8::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/relative_path_buf.rs
//     evidence={"module":3252,"record_offset":132536,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":42273,"source_line":371,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3252,"record_offset":134120,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3252,"record_offset":133396,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3252,"record_offset":129100,"parent_record_offset":128852,"ancestor_record_offset":115148,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":3252,"record_offset":129128,"parent_record_offset":129100,"ancestor_record_offset":115148,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":3252,"record_offset":129200,"parent_record_offset":129156,"ancestor_record_offset":115148,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3252,"record_offset":129460,"parent_record_offset":129436,"ancestor_record_offset":115148,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3252,"record_offset":129320,"parent_record_offset":129292,"ancestor_record_offset":115148,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":3252,"record_offset":129656,"parent_record_offset":129256,"ancestor_record_offset":115148,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":3252,"record_offset":129156,"parent_record_offset":129128,"ancestor_record_offset":115148,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3252,"record_offset":129348,"parent_record_offset":129320,"ancestor_record_offset":115148,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3252,"record_offset":129380,"parent_record_offset":129348,"ancestor_record_offset":115148,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3252,"record_offset":129256,"parent_record_offset":129228,"ancestor_record_offset":115148,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3252,"record_offset":129228,"parent_record_offset":129200,"ancestor_record_offset":115148,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::BuildContext::with_spawner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":3252,"record_offset":115988,"parent_record_offset":115260,"ancestor_record_offset":115148,"inlinee":81575,"source_line":668,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::BuiltAsset::asset_ref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":3252,"record_offset":121796,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":81565,"source_line":555,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":3252,"record_offset":139364,"parent_record_offset":138816,"ancestor_record_offset":115148,"inlinee":28224,"source_line":273,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::asset::impl$33::deref
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":3252,"record_offset":121824,"parent_record_offset":117348,"ancestor_record_offset":115148,"inlinee":81567,"source_line":574,"archive":"_pdb/C/modules/03252.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::Fn::call<void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >),tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1651,"record_offset":312676,"rva":24355568,"va":5393064688}
// 编译实例 RVAs: 0x173a2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8498395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8d5221dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e892a10000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::AsyncComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e825957eff","size":5,"soft_fail":false,"target_va":5384583232,"va":5393064726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_publish::spawn_async_task
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_publish.rs
//     evidence={"module":1651,"record_offset":313088,"parent_record_offset":312676,"ancestor_record_offset":312676,"inlinee":35077,"source_line":602,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":313124,"parent_record_offset":313088,"ancestor_record_offset":312676,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnMut::call_mut<void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >),tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1651,"record_offset":313160,"rva":24355568,"va":5393064688}
// 编译实例 RVAs: 0x173a2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8498395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8d5221dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e892a10000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::AsyncComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e825957eff","size":5,"soft_fail":false,"target_va":5384583232,"va":5393064726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_publish::spawn_async_task
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_publish.rs
//     evidence={"module":1651,"record_offset":313580,"parent_record_offset":313160,"ancestor_record_offset":313160,"inlinee":35077,"source_line":602,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":313616,"parent_record_offset":313580,"ancestor_record_offset":313160,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1651,"record_offset":314672,"rva":24355568,"va":5393064688}
// 编译实例 RVAs: 0x173a2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8498395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8d5221dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e892a10000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::AsyncComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e825957eff","size":5,"soft_fail":false,"target_va":5384583232,"va":5393064726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1651,"record_offset":315032,"parent_record_offset":314672,"ancestor_record_offset":314672,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":315088,"parent_record_offset":315056,"ancestor_record_offset":314672,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1651,"record_offset":315056,"parent_record_offset":315032,"ancestor_record_offset":314672,"inlinee":81753,"source_line":398,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F59
// 首条位置: {"module":1651,"inlinee":81753}
// 编译实例 RVAs: 0x173a2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=1
//     target_aliases=66 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23755959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8498395fe","size":5,"soft_fail":false,"target_va":5369308864,"va":5393064818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090c610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14867755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e8d5221dff","size":5,"soft_fail":false,"target_va":5378197008,"va":5393064758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417444c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e892a10000","size":5,"soft_fail":false,"target_va":5393106112,"va":5393064745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::AsyncComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8481499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393064688,"raw_hex":"e825957eff","size":5,"soft_fail":false,"target_va":5384583232,"va":5393064726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":311772,"parent_record_offset":311604,"ancestor_record_offset":311604,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":515,"record_offset":239680,"rva":9914864,"va":5378623984}
// 编译实例 RVAs: 0x9749f0, 0x1746410, 0x25925d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=50
//     evidence={"assembly":"\tcallq\t-2945489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e82f0ed3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378624188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9394720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8e0a570ff","size":5,"soft_fail":false,"target_va":5369231648,"va":5378626363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9111610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8c6f774ff","size":5,"soft_fail":false,"target_va":5369512480,"va":5378624085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e860dfffff","size":5,"soft_fail":false,"target_va":5378616384,"va":5378624731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140979570
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14472709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393114128,"raw_hex":"e8fb2923ff","size":5,"soft_fail":false,"target_va":5378643312,"va":5393116016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098d100
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t99406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e84e840100","size":5,"soft_fail":false,"target_va":5378724096,"va":5378624685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t111258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e89ab20100","size":5,"soft_fail":false,"target_va":5378736160,"va":5378624897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140994800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14360666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393114128,"raw_hex":"e8a6df24ff","size":5,"soft_fail":false,"target_va":5378754560,"va":5393115221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a2540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t184800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8e0d10200","size":5,"soft_fail":false,"target_va":5378811200,"va":5378626395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90660
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3258808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8b8b93100","size":5,"soft_fail":false,"target_va":5381883488,"va":5378624675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3265124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e864d23100","size":5,"soft_fail":false,"target_va":5381889776,"va":5378624647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91fc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3265366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e856d33100","size":5,"soft_fail":false,"target_va":5381889984,"va":5378624613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425928e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5408105936,"raw_hex":"e8d6020000","size":5,"soft_fail":false,"target_va":5408106720,"va":5408105989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77180
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37756359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8c71d4002","size":5,"soft_fail":false,"target_va":5416382848,"va":5378626484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37772111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e84f5b4002","size":5,"soft_fail":false,"target_va":5416398672,"va":5378626556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t5013509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e805804c00","size":5,"soft_fail":false,"target_va":5383637632,"va":5378624118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<asset_pipe::texture::build_texture_assets::async_fn_env$0> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t203739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e8db1b0300","size":5,"soft_fail":false,"target_va":5378828448,"va":5378624704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<clutter_pack_pipe::build_object_textures::async_fn_env$0> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t201576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378623984,"raw_hex":"e868130300","size":5,"soft_fail":false,"target_va":5378826160,"va":5378624579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t37773630","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378623984,"raw_hex":"e93e614002","size":5,"soft_fail":false,"target_va":5416398672,"va":5378625037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tjmp\t5013340","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378623984,"raw_hex":"e95c7f4c00","size":5,"soft_fail":false,"target_va":5383637632,"va":5378624287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378623984,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378624347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":239876,"parent_record_offset":239680,"ancestor_record_offset":239680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":240376,"parent_record_offset":240352,"ancestor_record_offset":239680,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":240400,"parent_record_offset":240376,"ancestor_record_offset":239680,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":240424,"parent_record_offset":240400,"ancestor_record_offset":239680,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":515,"record_offset":240352,"parent_record_offset":240220,"ancestor_record_offset":239680,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":515,"record_offset":240220,"parent_record_offset":240192,"ancestor_record_offset":239680,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":242952,"parent_record_offset":242920,"ancestor_record_offset":239680,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":242920,"parent_record_offset":242892,"ancestor_record_offset":239680,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":515,"record_offset":242892,"parent_record_offset":242864,"ancestor_record_offset":239680,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":515,"record_offset":240036,"parent_record_offset":240000,"ancestor_record_offset":239680,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":515,"record_offset":240000,"parent_record_offset":239976,"ancestor_record_offset":239680,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":515,"record_offset":242724,"parent_record_offset":242696,"ancestor_record_offset":239680,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":515,"record_offset":249700,"parent_record_offset":249676,"ancestor_record_offset":239680,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":515,"record_offset":249676,"parent_record_offset":249652,"ancestor_record_offset":239680,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":515,"record_offset":240252,"parent_record_offset":240220,"ancestor_record_offset":239680,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1654,"record_offset":55952,"parent_record_offset":55928,"ancestor_record_offset":51548,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":515,"record_offset":240316,"parent_record_offset":240280,"ancestor_record_offset":239680,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":515,"record_offset":240280,"parent_record_offset":240252,"ancestor_record_offset":239680,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1654,"record_offset":56880,"parent_record_offset":56852,"ancestor_record_offset":51548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1654,"record_offset":56908,"parent_record_offset":56880,"ancestor_record_offset":51548,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":515,"record_offset":243516,"parent_record_offset":243488,"ancestor_record_offset":239680,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1654,"record_offset":56172,"parent_record_offset":56148,"ancestor_record_offset":51548,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1654,"record_offset":55984,"parent_record_offset":55900,"ancestor_record_offset":51548,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=190
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":515,"record_offset":239976,"parent_record_offset":239876,"ancestor_record_offset":239680,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1654,"record_offset":56440,"parent_record_offset":56416,"ancestor_record_offset":51548,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1654,"record_offset":56292,"parent_record_offset":56268,"ancestor_record_offset":51548,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":515,"record_offset":240068,"parent_record_offset":240036,"ancestor_record_offset":239680,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":515,"record_offset":249728,"parent_record_offset":249700,"ancestor_record_offset":239680,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":515,"record_offset":240096,"parent_record_offset":240036,"ancestor_record_offset":239680,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":515,"record_offset":240128,"parent_record_offset":240000,"ancestor_record_offset":239680,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1654,"record_offset":56116,"parent_record_offset":56088,"ancestor_record_offset":51548,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1654,"record_offset":56148,"parent_record_offset":56116,"ancestor_record_offset":51548,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1654,"record_offset":56088,"parent_record_offset":56048,"ancestor_record_offset":51548,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1654,"record_offset":55928,"parent_record_offset":55900,"ancestor_record_offset":51548,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56416,"parent_record_offset":55808,"ancestor_record_offset":51548,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56268,"parent_record_offset":56048,"ancestor_record_offset":51548,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55900,"parent_record_offset":55844,"ancestor_record_offset":51548,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56048,"parent_record_offset":56020,"ancestor_record_offset":51548,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56820,"parent_record_offset":56788,"ancestor_record_offset":51548,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55872,"parent_record_offset":55844,"ancestor_record_offset":51548,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55808,"parent_record_offset":55740,"ancestor_record_offset":51548,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55740,"parent_record_offset":55708,"ancestor_record_offset":51548,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56788,"parent_record_offset":55740,"ancestor_record_offset":51548,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55780,"parent_record_offset":55740,"ancestor_record_offset":51548,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55844,"parent_record_offset":55808,"ancestor_record_offset":51548,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56852,"parent_record_offset":56820,"ancestor_record_offset":51548,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":55708,"parent_record_offset":55680,"ancestor_record_offset":51548,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1654,"record_offset":56020,"parent_record_offset":55808,"ancestor_record_offset":51548,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01654.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::InFlightPackBuild::objects
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9231
// 首条位置: {"module":586,"inlinee":37425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::mark_workshop_clutter_failed
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FE0
// 首条位置: {"module":1770,"inlinee":85984}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":82284,"parent_record_offset":82248,"ancestor_record_offset":82248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":82308,"parent_record_offset":82284,"ancestor_record_offset":82248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::published_item_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 466
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FDF
// 首条位置: {"module":1770,"inlinee":85983}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::build_requests
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8AD0
// 首条位置: {"module":565,"inlinee":35536}
// 编译实例 RVAs: 0x1adc140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21194907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873536,"raw_hex":"e86597bcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396873590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28151390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873536,"raw_hex":"e8a27152fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396873577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27361260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873536,"raw_hex":"e814805efe","size":5,"soft_fail":false,"target_va":5369512480,"va":5396873735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180dca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2942253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873536,"raw_hex":"e8d31ad3ff","size":5,"soft_fail":false,"target_va":5393931424,"va":5396873672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19747181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396873536,"raw_hex":"e86d512d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396873608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":32664,"parent_record_offset":32620,"ancestor_record_offset":32620,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":565,"record_offset":33364,"parent_record_offset":32620,"ancestor_record_offset":32620,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33128,"parent_record_offset":33104,"ancestor_record_offset":32620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33152,"parent_record_offset":33128,"ancestor_record_offset":32620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33176,"parent_record_offset":33152,"ancestor_record_offset":32620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":565,"record_offset":33104,"parent_record_offset":32960,"ancestor_record_offset":32620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32900,"parent_record_offset":32868,"ancestor_record_offset":32620,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32960,"parent_record_offset":32928,"ancestor_record_offset":32620,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32928,"parent_record_offset":32900,"ancestor_record_offset":32620,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":565,"record_offset":32992,"parent_record_offset":32960,"ancestor_record_offset":32620,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":33260,"parent_record_offset":33236,"ancestor_record_offset":32620,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32836,"parent_record_offset":32784,"ancestor_record_offset":32620,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32868,"parent_record_offset":32836,"ancestor_record_offset":32620,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":565,"record_offset":32704,"parent_record_offset":32664,"ancestor_record_offset":32620,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":565,"record_offset":33236,"parent_record_offset":32784,"ancestor_record_offset":32620,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":565,"record_offset":32744,"parent_record_offset":32704,"ancestor_record_offset":32620,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":565,"record_offset":32784,"parent_record_offset":32744,"ancestor_record_offset":32620,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":565,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":32620,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":33284,"parent_record_offset":33260,"ancestor_record_offset":32620,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":565,"record_offset":33308,"parent_record_offset":33284,"ancestor_record_offset":32620,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":33040,"parent_record_offset":33016,"ancestor_record_offset":32620,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":565,"record_offset":33064,"parent_record_offset":33040,"ancestor_record_offset":32620,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":565,"record_offset":33412,"parent_record_offset":33388,"ancestor_record_offset":32620,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":565,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":32620,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00565.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::build_requests::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14364
// 首条位置: {"module":1682,"inlinee":82788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1682,"record_offset":20192,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":21564,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::as_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1682,"record_offset":20108,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":31752,"source_line":48,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1682,"record_offset":20820,"parent_record_offset":20064,"ancestor_record_offset":20064,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":20064,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":20664,"parent_record_offset":20640,"ancestor_record_offset":20064,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":21740,"parent_record_offset":21716,"ancestor_record_offset":20064,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":21764,"parent_record_offset":21740,"ancestor_record_offset":20064,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":20688,"parent_record_offset":20664,"ancestor_record_offset":20064,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":21788,"parent_record_offset":21764,"ancestor_record_offset":20064,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":20616,"parent_record_offset":20460,"ancestor_record_offset":20064,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":21716,"parent_record_offset":21660,"ancestor_record_offset":20064,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":20064,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":21688,"parent_record_offset":21660,"ancestor_record_offset":20064,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":21660,"parent_record_offset":21636,"ancestor_record_offset":20064,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":20460,"parent_record_offset":20424,"ancestor_record_offset":20064,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":20424,"parent_record_offset":20392,"ancestor_record_offset":20064,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":20064,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":20504,"parent_record_offset":20460,"ancestor_record_offset":20064,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":20256,"parent_record_offset":20224,"ancestor_record_offset":20064,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":20288,"parent_record_offset":20256,"ancestor_record_offset":20064,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":20320,"parent_record_offset":20288,"ancestor_record_offset":20064,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":20224,"parent_record_offset":20192,"ancestor_record_offset":20064,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":21012,"parent_record_offset":20960,"ancestor_record_offset":20064,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":20360,"parent_record_offset":20320,"ancestor_record_offset":20064,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":20932,"parent_record_offset":20904,"ancestor_record_offset":20064,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":20960,"parent_record_offset":20932,"ancestor_record_offset":20064,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1682,"record_offset":20904,"parent_record_offset":20876,"ancestor_record_offset":20064,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1682,"record_offset":20528,"parent_record_offset":20504,"ancestor_record_offset":20064,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":20576,"parent_record_offset":20552,"ancestor_record_offset":20064,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":20552,"parent_record_offset":20528,"ancestor_record_offset":20064,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":20744,"parent_record_offset":20320,"ancestor_record_offset":20064,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":20768,"parent_record_offset":20744,"ancestor_record_offset":20064,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":20064,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1682,"record_offset":20848,"parent_record_offset":20820,"ancestor_record_offset":20064,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1682,"record_offset":20876,"parent_record_offset":20848,"ancestor_record_offset":20064,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1682,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":20064,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1682,"record_offset":20156,"parent_record_offset":20132,"ancestor_record_offset":20064,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs
// 记录形式: compiled_procedure; instances=1
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1770,"record_offset":35364,"rva":28170912,"va":5396880032}
// 编译实例 RVAs: 0x1addaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12086040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e89447ff","size":5,"soft_fail":false,"target_va":5384795984,"va":5396882019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12236561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8ef4845ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5396881468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12274184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8f8b544ff","size":5,"soft_fail":false,"target_va":5384609408,"va":5396883587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-21206978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e83e68bcfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5396885661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=45
//     evidence={"assembly":"\tcallq\t-21202239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8c17abcfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396880938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_realloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21205593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8a76dbcfe","size":5,"soft_fail":false,"target_va":5375678720,"va":5396884308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28163455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8814252fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5396885642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27669469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e823cc59fe","size":5,"soft_fail":false,"target_va":5369224048,"va":5396893512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087a30
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27634636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e834545afe","size":5,"soft_fail":false,"target_va":5369264688,"va":5396899319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=2
//     target_aliases=34 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27220961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e81fa460fe","size":5,"soft_fail":false,"target_va":5369660368,"va":5396881324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23400866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e85eee9afe","size":5,"soft_fail":false,"target_va":5373480896,"va":5396881757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22790669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8f33da4fe","size":5,"soft_fail":false,"target_va":5374098704,"va":5396889368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ca10
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18301661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e823bde8fe","size":5,"soft_fail":false,"target_va":5378591248,"va":5396892904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18300061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e863c3e8fe","size":5,"soft_fail":false,"target_va":5378593056,"va":5396893112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971550
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18282810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8c606e9fe","size":5,"soft_fail":false,"target_va":5378610512,"va":5396893317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140977fb0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18261096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8985be9fe","size":5,"soft_fail":false,"target_va":5378637744,"va":5396898835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ce50
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18240285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e3ace9fe","size":5,"soft_fail":false,"target_va":5378657872,"va":5396898152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18161692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e4dfeafe","size":5,"soft_fail":false,"target_va":5378734528,"va":5396896215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18150883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e81d0aebfe","size":5,"soft_fail":false,"target_va":5378736416,"va":5396887294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409911c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18157293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e813f1eafe","size":5,"soft_fail":false,"target_va":5378740672,"va":5396897960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140994800
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18126339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8fd69ebfe","size":5,"soft_fail":false,"target_va":5378754560,"va":5396880894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099e810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18098186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8f6d7ebfe","size":5,"soft_fail":false,"target_va":5378795536,"va":5396893717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a9370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18055656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8187eecfe","size":5,"soft_fail":false,"target_va":5378839408,"va":5396895059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac74f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16873797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8bb86fefe","size":5,"soft_fail":false,"target_va":5380011248,"va":5396885040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac7970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16872546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e89e8bfefe","size":5,"soft_fail":false,"target_va":5380012400,"va":5396884941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16869959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8b995fefe","size":5,"soft_fail":false,"target_va":5380014704,"va":5396884658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c05040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15573701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e83b5d12ff","size":5,"soft_fail":false,"target_va":5381312576,"va":5396886272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c06460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15568623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8117112ff","size":5,"soft_fail":false,"target_va":5381317728,"va":5396886346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9fdf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14938708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8ac0d1cff","size":5,"soft_fail":false,"target_va":5381946864,"va":5396885567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca6340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14914206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8626d1cff","size":5,"soft_fail":false,"target_va":5381972800,"va":5396887001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13795274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e836802dff","size":5,"soft_fail":false,"target_va":5383091344,"va":5396886613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13558487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8291d31ff","size":5,"soft_fail":false,"target_va":5383324784,"va":5396883266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f57190
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12094337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e87f7447ff","size":5,"soft_fail":false,"target_va":5384794512,"va":5396888844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f2370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5167213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e89327b1ff","size":5,"soft_fail":false,"target_va":5391721328,"va":5396888536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5075620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e85c8db2ff","size":5,"soft_fail":false,"target_va":5391812880,"va":5396888495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4111706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8a642c1ff","size":5,"soft_fail":false,"target_va":5392780768,"va":5396892469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ab2c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3356128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e820caccff","size":5,"soft_fail":false,"target_va":5393527488,"va":5396883611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417af680
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3337248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e013cdff","size":5,"soft_fail":false,"target_va":5393544832,"va":5396882075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3323995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8a547cdff","size":5,"soft_fail":false,"target_va":5393558928,"va":5396882918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b4650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3317271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e961cdff","size":5,"soft_fail":false,"target_va":5393565264,"va":5396882530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141810bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2937883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e52bd3ff","size":5,"soft_fail":false,"target_va":5393943536,"va":5396881414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419c2910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1160857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e86749eeff","size":5,"soft_fail":false,"target_va":5395720464,"va":5396881316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0e380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e80a060300","size":5,"soft_fail":false,"target_va":5397078912,"va":5396880753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t198798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e88e080300","size":5,"soft_fail":false,"target_va":5397079424,"va":5396880621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0e9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t196576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e0ff0200","size":5,"soft_fail":false,"target_va":5397080528,"va":5396883947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c12b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1258345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e869331300","size":5,"soft_fail":false,"target_va":5398145904,"va":5396887554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19493469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e85d722901","size":5,"soft_fail":false,"target_va":5416385024,"va":5396891550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d92af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19604174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8ce222b01","size":5,"soft_fail":false,"target_va":5416495856,"va":5396891677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da02d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19663685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8450b2c01","size":5,"soft_fail":false,"target_va":5416551120,"va":5396887430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19888543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e89f792f01","size":5,"soft_fail":false,"target_va":5416769200,"va":5396880652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20175154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e832d93301","size":5,"soft_fail":false,"target_va":5417057040,"va":5396881881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: load_clutter_packs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20171305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e829ca3301","size":5,"soft_fail":false,"target_va":5417057056,"va":5396885746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19728892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8fc092d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5396891872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12087896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8a88d47ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5396882755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t19730695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e807112d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5396890094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19886050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8e26f2f01","size":5,"soft_fail":false,"target_va":5416776864,"va":5396890809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12265476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8fcd744ff","size":5,"soft_fail":false,"target_va":5384624080,"va":5396889551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] clutter_pack_pipe::ClutterMaterialEntry::emits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14652772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e89c6a20ff","size":5,"soft_fail":false,"target_va":5382230080,"va":5396882847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] clutter_pack_pipe::clutter_pack_package_dir
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14650384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8f07320ff","size":5,"soft_fail":false,"target_va":5382230208,"va":5396880587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19729813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8950d2d01","size":5,"soft_fail":false,"target_va":5416621408,"va":5396891590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19729934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e80e0e2d01","size":5,"soft_fail":false,"target_va":5416621762,"va":5396891823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19732161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8c1162d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5396890074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e885600000","size":5,"soft_fail":false,"target_va":5396904896,"va":5396880182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19729458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8320c2d01","size":5,"soft_fail":false,"target_va":5416621376,"va":5396891913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17619784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8b824f3fe","size":5,"soft_fail":false,"target_va":5379266432,"va":5396886211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17611604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8ac44f3fe","size":5,"soft_fail":false,"target_va":5379276736,"va":5396888335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::load_packaged_texture_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16051607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e869120bff","size":5,"soft_fail":false,"target_va":5380836512,"va":5396888114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::aabb_bounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15422522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8c6ab14ff","size":5,"soft_fail":false,"target_va":5381462752,"va":5396885269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15418497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e87fbb14ff","size":5,"soft_fail":false,"target_va":5381466592,"va":5396885084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::ClutterPackObject::id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14979754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8566d1bff","size":5,"soft_fail":false,"target_va":5381902240,"va":5396881989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::register_external
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17477084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e82452f5fe","size":5,"soft_fail":false,"target_va":5379412080,"va":5396889159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12916751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8f1e73aff","size":5,"soft_fail":false,"target_va":5383963792,"va":5396880538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12908243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e82d093bff","size":5,"soft_fail":false,"target_va":5383972224,"va":5396880462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12916127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e861ea3aff","size":5,"soft_fail":false,"target_va":5383964912,"va":5396881034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12917346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e89ee53aff","size":5,"soft_fail":false,"target_va":5383962816,"va":5396880157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-12163195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8856746ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5396889878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t19727783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8a7052d01","size":5,"soft_fail":false,"target_va":5416619360,"va":5396891572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13582142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8c2c030ff","size":5,"soft_fail":false,"target_va":5383303280,"va":5396885417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::extent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13581878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8cac130ff","size":5,"soft_fail":false,"target_va":5383303440,"va":5396885313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13571117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e8d3eb30ff","size":5,"soft_fail":false,"target_va":5383314384,"va":5396885496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13558403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396880032,"raw_hex":"e87d1d31ff","size":5,"soft_fail":false,"target_va":5383324736,"va":5396883134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1770,"record_offset":62484,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1770,"record_offset":47424,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::into_boxed_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1770,"record_offset":50936,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":11361,"source_line":2174,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":43380,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1770,"record_offset":56188,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":49916,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1770,"record_offset":47508,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":41292,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":47312,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":52040,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":68736,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":38044,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1770,"record_offset":37916,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1770,"record_offset":37952,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":41208,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":47620,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":43492,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1770,"record_offset":54444,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1770,"record_offset":54416,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::collision_bbx
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":52592,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":37981,"source_line":172,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::collision_radius_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":52096,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":37984,"source_line":267,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":51664,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":52068,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":52644,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":32361,"source_line":10,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::mesh_atlas::MeshAtlas::remaining_slots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":1770,"record_offset":48808,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":33225,"source_line":222,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::as_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":47340,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":31752,"source_line":48,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterRegistry::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":46288,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":32530,"source_line":395,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":47776,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":48836,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":33758,"source_line":43,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":50056,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":49832,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":37080,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":35680,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":36068,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":48864,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":43584,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":44672,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":56488,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":50168,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":41948,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":59168,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":59700,"parent_record_offset":35364,"ancestor_record_offset":35364,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":53212,"parent_record_offset":53188,"ancestor_record_offset":35364,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":53236,"parent_record_offset":53212,"ancestor_record_offset":35364,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":38328,"parent_record_offset":38304,"ancestor_record_offset":35364,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":38352,"parent_record_offset":38328,"ancestor_record_offset":35364,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":51184,"parent_record_offset":51160,"ancestor_record_offset":35364,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":51208,"parent_record_offset":51184,"ancestor_record_offset":35364,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":53260,"parent_record_offset":53236,"ancestor_record_offset":35364,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":38376,"parent_record_offset":38352,"ancestor_record_offset":35364,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":53188,"parent_record_offset":53008,"ancestor_record_offset":35364,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":38304,"parent_record_offset":38216,"ancestor_record_offset":35364,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":51160,"parent_record_offset":51124,"ancestor_record_offset":35364,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1770,"record_offset":51232,"parent_record_offset":51208,"ancestor_record_offset":35364,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1770,"record_offset":62520,"parent_record_offset":62484,"ancestor_record_offset":35364,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1770,"record_offset":47472,"parent_record_offset":47448,"ancestor_record_offset":35364,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":51040,"parent_record_offset":51008,"ancestor_record_offset":35364,"inlinee":6565,"source_line":398,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":52948,"parent_record_offset":52920,"ancestor_record_offset":35364,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":38244,"parent_record_offset":38216,"ancestor_record_offset":35364,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":38216,"parent_record_offset":38192,"ancestor_record_offset":35364,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":51100,"parent_record_offset":51068,"ancestor_record_offset":35364,"inlinee":6567,"source_line":811,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":51068,"parent_record_offset":51040,"ancestor_record_offset":35364,"inlinee":6566,"source_line":743,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":51124,"parent_record_offset":51100,"ancestor_record_offset":35364,"inlinee":6568,"source_line":827,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":53008,"parent_record_offset":52976,"ancestor_record_offset":35364,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":52976,"parent_record_offset":52948,"ancestor_record_offset":35364,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":38192,"parent_record_offset":38168,"ancestor_record_offset":35364,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1770,"record_offset":53044,"parent_record_offset":53008,"ancestor_record_offset":35364,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1770,"record_offset":54584,"parent_record_offset":54552,"ancestor_record_offset":35364,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1770,"record_offset":54616,"parent_record_offset":54584,"ancestor_record_offset":35364,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1770,"record_offset":52880,"parent_record_offset":52812,"ancestor_record_offset":35364,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1770,"record_offset":54552,"parent_record_offset":54520,"ancestor_record_offset":35364,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1770,"record_offset":54520,"parent_record_offset":54488,"ancestor_record_offset":35364,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1770,"record_offset":54488,"parent_record_offset":54444,"ancestor_record_offset":35364,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":52852,"parent_record_offset":52812,"ancestor_record_offset":35364,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":50972,"parent_record_offset":50936,"ancestor_record_offset":35364,"inlinee":6562,"source_line":1708,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":51008,"parent_record_offset":50972,"ancestor_record_offset":35364,"inlinee":6563,"source_line":1579,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":52920,"parent_record_offset":52880,"ancestor_record_offset":35364,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":52780,"parent_record_offset":52644,"ancestor_record_offset":35364,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":62424,"parent_record_offset":62400,"ancestor_record_offset":35364,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":38100,"parent_record_offset":38072,"ancestor_record_offset":35364,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1770,"record_offset":52812,"parent_record_offset":52780,"ancestor_record_offset":35364,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1770,"record_offset":53516,"parent_record_offset":53484,"ancestor_record_offset":35364,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1770,"record_offset":53068,"parent_record_offset":53044,"ancestor_record_offset":35364,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1770,"record_offset":56212,"parent_record_offset":56188,"ancestor_record_offset":35364,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1770,"record_offset":56328,"parent_record_offset":56236,"ancestor_record_offset":35364,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37436,"parent_record_offset":37408,"ancestor_record_offset":35364,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":41588,"parent_record_offset":41560,"ancestor_record_offset":35364,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37464,"parent_record_offset":37436,"ancestor_record_offset":35364,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":41616,"parent_record_offset":41560,"ancestor_record_offset":35364,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37376,"parent_record_offset":37344,"ancestor_record_offset":35364,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37408,"parent_record_offset":37376,"ancestor_record_offset":35364,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37524,"parent_record_offset":37408,"ancestor_record_offset":35364,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1770,"record_offset":37636,"parent_record_offset":37612,"ancestor_record_offset":35364,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1770,"record_offset":44328,"parent_record_offset":44304,"ancestor_record_offset":35364,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":60040,"parent_record_offset":60016,"ancestor_record_offset":35364,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":43892,"parent_record_offset":43868,"ancestor_record_offset":35364,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1770,"record_offset":43976,"parent_record_offset":43952,"ancestor_record_offset":35364,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1770,"record_offset":52560,"parent_record_offset":52096,"ancestor_record_offset":35364,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1770,"record_offset":52520,"parent_record_offset":52496,"ancestor_record_offset":35364,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":44496,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1770,"record_offset":41868,"parent_record_offset":41836,"ancestor_record_offset":35364,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1770,"record_offset":52164,"parent_record_offset":52132,"ancestor_record_offset":35364,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1770,"record_offset":59116,"parent_record_offset":59092,"ancestor_record_offset":35364,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1770,"record_offset":52132,"parent_record_offset":52096,"ancestor_record_offset":35364,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1770,"record_offset":62448,"parent_record_offset":62424,"ancestor_record_offset":35364,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1770,"record_offset":41904,"parent_record_offset":41776,"ancestor_record_offset":35364,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1770,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":35364,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":59504,"parent_record_offset":59420,"ancestor_record_offset":35364,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":53092,"parent_record_offset":53068,"ancestor_record_offset":35364,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":53120,"parent_record_offset":53092,"ancestor_record_offset":35364,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":56296,"parent_record_offset":56272,"ancestor_record_offset":35364,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":38272,"parent_record_offset":38244,"ancestor_record_offset":35364,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1770,"record_offset":42544,"parent_record_offset":42488,"ancestor_record_offset":35364,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":44128,"parent_record_offset":44100,"ancestor_record_offset":35364,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1770,"record_offset":44100,"parent_record_offset":44072,"ancestor_record_offset":35364,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":47744,"parent_record_offset":47620,"ancestor_record_offset":35364,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":37244,"parent_record_offset":37196,"ancestor_record_offset":35364,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1770,"record_offset":52360,"parent_record_offset":52328,"ancestor_record_offset":35364,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1770,"record_offset":56272,"parent_record_offset":56236,"ancestor_record_offset":35364,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1770,"record_offset":56236,"parent_record_offset":56212,"ancestor_record_offset":35364,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":52296,"parent_record_offset":52164,"ancestor_record_offset":35364,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":49940,"parent_record_offset":49916,"ancestor_record_offset":35364,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":52328,"parent_record_offset":52132,"ancestor_record_offset":35364,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":47448,"parent_record_offset":47424,"ancestor_record_offset":35364,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":42244,"parent_record_offset":42212,"ancestor_record_offset":35364,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1770,"record_offset":53384,"parent_record_offset":52644,"ancestor_record_offset":35364,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":36016,"parent_record_offset":35988,"ancestor_record_offset":35364,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":37872,"parent_record_offset":37196,"ancestor_record_offset":35364,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":53316,"parent_record_offset":52880,"ancestor_record_offset":35364,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1770,"record_offset":56412,"parent_record_offset":56352,"ancestor_record_offset":35364,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":43916,"parent_record_offset":43892,"ancestor_record_offset":35364,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":37588,"parent_record_offset":37344,"ancestor_record_offset":35364,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":41256,"parent_record_offset":41232,"ancestor_record_offset":35364,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":65056,"parent_record_offset":65032,"ancestor_record_offset":35364,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":60216,"parent_record_offset":59812,"ancestor_record_offset":35364,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":42332,"parent_record_offset":42212,"ancestor_record_offset":35364,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1770,"record_offset":44244,"parent_record_offset":44220,"ancestor_record_offset":35364,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":43552,"parent_record_offset":43492,"ancestor_record_offset":35364,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":35364,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":35364,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1770,"record_offset":36672,"parent_record_offset":36644,"ancestor_record_offset":35364,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":56436,"parent_record_offset":56412,"ancestor_record_offset":35364,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1770,"record_offset":42356,"parent_record_offset":42332,"ancestor_record_offset":35364,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1770,"record_offset":51272,"parent_record_offset":51124,"ancestor_record_offset":35364,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1770,"record_offset":44412,"parent_record_offset":44384,"ancestor_record_offset":35364,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1770,"record_offset":36772,"parent_record_offset":36744,"ancestor_record_offset":35364,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1770,"record_offset":44440,"parent_record_offset":44412,"ancestor_record_offset":35364,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":36744,"parent_record_offset":36716,"ancestor_record_offset":35364,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":56352,"parent_record_offset":56328,"ancestor_record_offset":35364,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1770,"record_offset":43404,"parent_record_offset":43380,"ancestor_record_offset":35364,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1770,"record_offset":56384,"parent_record_offset":56352,"ancestor_record_offset":35364,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1770,"record_offset":41232,"parent_record_offset":41208,"ancestor_record_offset":35364,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":36416,"parent_record_offset":36392,"ancestor_record_offset":35364,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":35364,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":36800,"parent_record_offset":36772,"ancestor_record_offset":35364,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":47652,"parent_record_offset":47620,"ancestor_record_offset":35364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1770,"record_offset":36392,"parent_record_offset":36276,"ancestor_record_offset":35364,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1770,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":35364,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1770,"record_offset":36104,"parent_record_offset":36068,"ancestor_record_offset":35364,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1770,"record_offset":36244,"parent_record_offset":36212,"ancestor_record_offset":35364,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1770,"record_offset":36212,"parent_record_offset":36176,"ancestor_record_offset":35364,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1770,"record_offset":36152,"parent_record_offset":36128,"ancestor_record_offset":35364,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1770,"record_offset":36176,"parent_record_offset":36152,"ancestor_record_offset":35364,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1770,"record_offset":36128,"parent_record_offset":36104,"ancestor_record_offset":35364,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1770,"record_offset":36716,"parent_record_offset":36176,"ancestor_record_offset":35364,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35784,"parent_record_offset":35752,"ancestor_record_offset":35364,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":35364,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35908,"parent_record_offset":35876,"ancestor_record_offset":35364,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35964,"parent_record_offset":35908,"ancestor_record_offset":35364,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35876,"parent_record_offset":35848,"ancestor_record_offset":35364,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":44356,"parent_record_offset":44328,"ancestor_record_offset":35364,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35724,"parent_record_offset":35680,"ancestor_record_offset":35364,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1770,"record_offset":35988,"parent_record_offset":35964,"ancestor_record_offset":35364,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1770,"record_offset":36276,"parent_record_offset":36244,"ancestor_record_offset":35364,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1770,"record_offset":36560,"parent_record_offset":36276,"ancestor_record_offset":35364,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1770,"record_offset":36532,"parent_record_offset":36276,"ancestor_record_offset":35364,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1770,"record_offset":36336,"parent_record_offset":36276,"ancestor_record_offset":35364,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":49856,"parent_record_offset":49832,"ancestor_record_offset":35364,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1770,"record_offset":49880,"parent_record_offset":49856,"ancestor_record_offset":35364,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$1::positions::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1770,"record_offset":52392,"parent_record_offset":52360,"ancestor_record_offset":35364,"inlinee":36789,"source_line":196,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1770,"record_offset":48532,"parent_record_offset":48508,"ancestor_record_offset":35364,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1770,"record_offset":47036,"parent_record_offset":47012,"ancestor_record_offset":35364,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":49616,"parent_record_offset":49592,"ancestor_record_offset":35364,"inlinee":37534,"source_line":20,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":1770,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":35364,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1770,"record_offset":68760,"parent_record_offset":68736,"ancestor_record_offset":35364,"inlinee":85981,"source_line":684,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1770,"record_offset":52464,"parent_record_offset":52440,"ancestor_record_offset":35364,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1770,"record_offset":52440,"parent_record_offset":52096,"ancestor_record_offset":35364,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":44036,"parent_record_offset":44008,"ancestor_record_offset":35364,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":44072,"parent_record_offset":44036,"ancestor_record_offset":35364,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1770,"record_offset":44008,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":43868,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":60016,"parent_record_offset":59852,"ancestor_record_offset":35364,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":44524,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1770,"record_offset":43952,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1770,"record_offset":43812,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1770,"record_offset":44304,"parent_record_offset":44280,"ancestor_record_offset":35364,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":48888,"parent_record_offset":48864,"ancestor_record_offset":35364,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":43620,"parent_record_offset":43584,"ancestor_record_offset":35364,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":56512,"parent_record_offset":56488,"ancestor_record_offset":35364,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":56544,"parent_record_offset":56512,"ancestor_record_offset":35364,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":41808,"parent_record_offset":41776,"ancestor_record_offset":35364,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":44280,"parent_record_offset":44172,"ancestor_record_offset":35364,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":52740,"parent_record_offset":52716,"ancestor_record_offset":35364,"inlinee":13277,"source_line":191,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":59248,"parent_record_offset":59208,"ancestor_record_offset":35364,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":59756,"parent_record_offset":59728,"ancestor_record_offset":35364,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":41776,"parent_record_offset":41748,"ancestor_record_offset":35364,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1770,"record_offset":43660,"parent_record_offset":43620,"ancestor_record_offset":35364,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":49484,"parent_record_offset":49400,"ancestor_record_offset":35364,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":65032,"parent_record_offset":64428,"ancestor_record_offset":35364,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":44220,"parent_record_offset":44196,"ancestor_record_offset":35364,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":60072,"parent_record_offset":59852,"ancestor_record_offset":35364,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":44620,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59620,"parent_record_offset":59596,"ancestor_record_offset":35364,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59852,"parent_record_offset":59812,"ancestor_record_offset":35364,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":44196,"parent_record_offset":44172,"ancestor_record_offset":35364,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":43720,"parent_record_offset":43688,"ancestor_record_offset":35364,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":43688,"parent_record_offset":43620,"ancestor_record_offset":35364,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59372,"parent_record_offset":59332,"ancestor_record_offset":35364,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59332,"parent_record_offset":59288,"ancestor_record_offset":35364,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":46196,"parent_record_offset":45188,"ancestor_record_offset":35364,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59572,"parent_record_offset":59288,"ancestor_record_offset":35364,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":46220,"parent_record_offset":46196,"ancestor_record_offset":35364,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":41836,"parent_record_offset":41808,"ancestor_record_offset":35364,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":65436,"parent_record_offset":65404,"ancestor_record_offset":35364,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":64488,"parent_record_offset":64460,"ancestor_record_offset":35364,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":64428,"parent_record_offset":64368,"ancestor_record_offset":35364,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":64368,"parent_record_offset":64344,"ancestor_record_offset":35364,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":43752,"parent_record_offset":43720,"ancestor_record_offset":35364,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":65404,"parent_record_offset":64368,"ancestor_record_offset":35364,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":64400,"parent_record_offset":64368,"ancestor_record_offset":35364,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59596,"parent_record_offset":59572,"ancestor_record_offset":35364,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":43840,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59420,"parent_record_offset":59372,"ancestor_record_offset":35364,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":64344,"parent_record_offset":64320,"ancestor_record_offset":35364,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59288,"parent_record_offset":59248,"ancestor_record_offset":35364,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59812,"parent_record_offset":59784,"ancestor_record_offset":35364,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":62156,"parent_record_offset":62132,"ancestor_record_offset":35364,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":59784,"parent_record_offset":59756,"ancestor_record_offset":35364,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1770,"record_offset":44172,"parent_record_offset":43752,"ancestor_record_offset":35364,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":50192,"parent_record_offset":50168,"ancestor_record_offset":35364,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":58984,"parent_record_offset":58960,"ancestor_record_offset":35364,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":59208,"parent_record_offset":59168,"ancestor_record_offset":35364,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1770,"record_offset":59728,"parent_record_offset":59700,"ancestor_record_offset":35364,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":50104,"parent_record_offset":50080,"ancestor_record_offset":35364,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1770,"record_offset":50080,"parent_record_offset":50056,"ancestor_record_offset":35364,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1770,"record_offset":50128,"parent_record_offset":50104,"ancestor_record_offset":35364,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":37116,"parent_record_offset":37080,"ancestor_record_offset":35364,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":39160,"parent_record_offset":39128,"ancestor_record_offset":35364,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":39640,"parent_record_offset":39424,"ancestor_record_offset":35364,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1770,"record_offset":37272,"parent_record_offset":37244,"ancestor_record_offset":35364,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":37344,"parent_record_offset":37196,"ancestor_record_offset":35364,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1770,"record_offset":37560,"parent_record_offset":37344,"ancestor_record_offset":35364,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":35364,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":41720,"parent_record_offset":41360,"ancestor_record_offset":35364,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":52716,"parent_record_offset":52644,"ancestor_record_offset":35364,"inlinee":13788,"source_line":1397,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":41360,"parent_record_offset":41320,"ancestor_record_offset":35364,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":41748,"parent_record_offset":41360,"ancestor_record_offset":35364,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1770,"record_offset":51700,"parent_record_offset":51664,"ancestor_record_offset":35364,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":58932,"parent_record_offset":58232,"ancestor_record_offset":35364,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":58960,"parent_record_offset":58232,"ancestor_record_offset":35364,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":41976,"parent_record_offset":41948,"ancestor_record_offset":35364,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1770,"record_offset":58232,"parent_record_offset":58196,"ancestor_record_offset":35364,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1770,"record_offset":52496,"parent_record_offset":52464,"ancestor_record_offset":35364,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1770,"record_offset":53452,"parent_record_offset":53420,"ancestor_record_offset":35364,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":41444,"parent_record_offset":41416,"ancestor_record_offset":35364,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1770,"record_offset":42292,"parent_record_offset":42268,"ancestor_record_offset":35364,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1770,"record_offset":42152,"parent_record_offset":42124,"ancestor_record_offset":35364,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":41560,"parent_record_offset":41504,"ancestor_record_offset":35364,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1770,"record_offset":41416,"parent_record_offset":41360,"ancestor_record_offset":35364,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1770,"record_offset":53420,"parent_record_offset":53384,"ancestor_record_offset":35364,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1770,"record_offset":53484,"parent_record_offset":53452,"ancestor_record_offset":35364,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1770,"record_offset":37300,"parent_record_offset":37272,"ancestor_record_offset":35364,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1770,"record_offset":42212,"parent_record_offset":42180,"ancestor_record_offset":35364,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1770,"record_offset":37196,"parent_record_offset":37152,"ancestor_record_offset":35364,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1770,"record_offset":37152,"parent_record_offset":37116,"ancestor_record_offset":35364,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1770,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":35364,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1770,"record_offset":47388,"parent_record_offset":47364,"ancestor_record_offset":35364,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1770,"record_offset":47060,"parent_record_offset":47036,"ancestor_record_offset":35364,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01770.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 537
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x140E2
// 首条位置: {"module":1658,"inlinee":82146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 541
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14381
// 首条位置: {"module":1682,"inlinee":82817}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1682,"record_offset":86300,"parent_record_offset":86176,"ancestor_record_offset":86176,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::as_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1682,"record_offset":86204,"parent_record_offset":86176,"ancestor_record_offset":86176,"inlinee":31752,"source_line":48,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":86756,"parent_record_offset":86732,"ancestor_record_offset":86176,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":86780,"parent_record_offset":86756,"ancestor_record_offset":86176,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":86804,"parent_record_offset":86780,"ancestor_record_offset":86176,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":86732,"parent_record_offset":86564,"ancestor_record_offset":86176,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":86496,"parent_record_offset":86464,"ancestor_record_offset":86176,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":86564,"parent_record_offset":86528,"ancestor_record_offset":86176,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":86528,"parent_record_offset":86496,"ancestor_record_offset":86176,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":86604,"parent_record_offset":86564,"ancestor_record_offset":86176,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":86364,"parent_record_offset":86332,"ancestor_record_offset":86176,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":86396,"parent_record_offset":86364,"ancestor_record_offset":86176,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":86428,"parent_record_offset":86396,"ancestor_record_offset":86176,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":86332,"parent_record_offset":86300,"ancestor_record_offset":86176,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":86464,"parent_record_offset":86428,"ancestor_record_offset":86176,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1682,"record_offset":86632,"parent_record_offset":86604,"ancestor_record_offset":86176,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":86688,"parent_record_offset":86660,"ancestor_record_offset":86176,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":86660,"parent_record_offset":86632,"ancestor_record_offset":86176,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":86860,"parent_record_offset":86428,"ancestor_record_offset":86176,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":86884,"parent_record_offset":86860,"ancestor_record_offset":86176,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1682,"record_offset":86232,"parent_record_offset":86204,"ancestor_record_offset":86176,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1682,"record_offset":86260,"parent_record_offset":86232,"ancestor_record_offset":86176,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14FDD
// 首条位置: {"module":1770,"inlinee":85981}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 689
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14386
// 首条位置: {"module":1682,"inlinee":82822}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":94556,"parent_record_offset":93548,"ancestor_record_offset":93548,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_by_name
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1682,"record_offset":93588,"parent_record_offset":93548,"ancestor_record_offset":93548,"inlinee":31917,"source_line":308,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":94340,"parent_record_offset":94316,"ancestor_record_offset":93548,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":93848,"parent_record_offset":93824,"ancestor_record_offset":93548,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":93932,"parent_record_offset":93908,"ancestor_record_offset":93548,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":94380,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":94080,"parent_record_offset":94052,"ancestor_record_offset":93548,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":94052,"parent_record_offset":94024,"ancestor_record_offset":93548,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":93872,"parent_record_offset":93848,"ancestor_record_offset":93548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1682,"record_offset":94200,"parent_record_offset":94176,"ancestor_record_offset":93548,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":93992,"parent_record_offset":93964,"ancestor_record_offset":93548,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":94024,"parent_record_offset":93992,"ancestor_record_offset":93548,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":93964,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":93824,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":94408,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":93908,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1682,"record_offset":93768,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1682,"record_offset":94316,"parent_record_offset":94292,"ancestor_record_offset":93548,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":93636,"parent_record_offset":93612,"ancestor_record_offset":93548,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":94292,"parent_record_offset":94124,"ancestor_record_offset":93548,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94232,"parent_record_offset":94148,"ancestor_record_offset":93548,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94176,"parent_record_offset":94148,"ancestor_record_offset":93548,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94500,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94148,"parent_record_offset":94124,"ancestor_record_offset":93548,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93688,"parent_record_offset":93664,"ancestor_record_offset":93548,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93664,"parent_record_offset":93636,"ancestor_record_offset":93548,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93712,"parent_record_offset":93688,"ancestor_record_offset":93548,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93796,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94124,"parent_record_offset":93712,"ancestor_record_offset":93548,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1682,"record_offset":93612,"parent_record_offset":93588,"ancestor_record_offset":93548,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 701
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14385
// 首条位置: {"module":1682,"inlinee":82821}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$5
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_clutter_packs.rs
//     evidence={"module":1682,"record_offset":93548,"parent_record_offset":93460,"ancestor_record_offset":93460,"inlinee":82822,"source_line":689,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":94340,"parent_record_offset":94316,"ancestor_record_offset":93460,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":93848,"parent_record_offset":93824,"ancestor_record_offset":93460,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":93932,"parent_record_offset":93908,"ancestor_record_offset":93460,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":94380,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":94080,"parent_record_offset":94052,"ancestor_record_offset":93460,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":94052,"parent_record_offset":94024,"ancestor_record_offset":93460,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":94556,"parent_record_offset":93548,"ancestor_record_offset":93460,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":93872,"parent_record_offset":93848,"ancestor_record_offset":93460,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1682,"record_offset":94200,"parent_record_offset":94176,"ancestor_record_offset":93460,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_by_name
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1682,"record_offset":93588,"parent_record_offset":93548,"ancestor_record_offset":93460,"inlinee":31917,"source_line":308,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":93992,"parent_record_offset":93964,"ancestor_record_offset":93460,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":94024,"parent_record_offset":93992,"ancestor_record_offset":93460,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":93964,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":93824,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":94408,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":93908,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1682,"record_offset":93768,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1682,"record_offset":94316,"parent_record_offset":94292,"ancestor_record_offset":93460,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":93636,"parent_record_offset":93612,"ancestor_record_offset":93460,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":94292,"parent_record_offset":94124,"ancestor_record_offset":93460,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94232,"parent_record_offset":94148,"ancestor_record_offset":93460,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94176,"parent_record_offset":94148,"ancestor_record_offset":93460,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94500,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94148,"parent_record_offset":94124,"ancestor_record_offset":93460,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93688,"parent_record_offset":93664,"ancestor_record_offset":93460,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93664,"parent_record_offset":93636,"ancestor_record_offset":93460,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93712,"parent_record_offset":93688,"ancestor_record_offset":93460,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":93796,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":94124,"parent_record_offset":93712,"ancestor_record_offset":93460,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1682,"record_offset":93612,"parent_record_offset":93588,"ancestor_record_offset":93460,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01682.jsonl.gz"}
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
// 完整结构及原始类型记录: load_clutter_packs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_clutter_packs::AppliedClutter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_clutter_packs::AppliedClutter>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_clutter_packs::AppliedClutter"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_clutter_packs::InFlightPackBuild
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_clutter_packs::InFlightPackBuild>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_clutter_packs::InFlightPackBuild"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::startup::load_clutter_packs::InFlightPackBuild>", "constructor": "core::option::Option", "arguments": ["country_core::startup::load_clutter_packs::InFlightPackBuild"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::startup::load_clutter_packs::InFlightPackBuild> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::startup::load_clutter_packs::InFlightPackBuild>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::startup::load_clutter_packs::InFlightPackBuild> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::startup::load_clutter_packs::InFlightPackBuild> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>", "constructor": "core::option::Iter", "arguments": ["country_core::startup::load_clutter_packs::InFlightPackBuild"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>", "country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >", "constructor": "hashbrown::set::impl$10::extend::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >", "hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["country_core::resources::clutter_registry::ClutterId", "tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >", "tuple$<>", "hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > >", "constructor": "core::iter::adapters::flatten::impl$20::fold::flatten::closure$0", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_clutter_packs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_clutter_packs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 115; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 函数: core::ops::function::Fn::call<void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >),tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 信息 ID: b2e46e1d1676641dab968f03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 参数类型拼写: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::Fn::call -> ["void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)", "tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnMut::call_mut<void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >),tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 信息 ID: c56cc9035dbca6d14e477b8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [符号内函数类型候选] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 参数类型拼写: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnMut::call_mut -> ["void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)", "tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >
// 信息 ID: 0843d06814d2e68c7ab72cfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] assoc$ -> ["Output", "tuple$<>"]; 字段布局 = unknown。
// [类型关系] core::future::future::Future -> ["assoc$<Output,tuple$<> >"]; 字段布局 = unknown。
// [类型关系] dyn$ -> ["core::future::future::Future<assoc$<Output,tuple$<> > >", "core::marker::Send"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::pin::Pin -> ["alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0", "tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> >
// 信息 ID: dc0559736bb3327a07a51d17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > > @ 0x989ded -> 0x9749f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > > @ 0x989e33 -> 0x9749f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > @ 0x17471ad -> 0x1746410 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > @ 0x17471bd -> 0x1746410 (direct_call); 同地址别名未裁定。
// [静态直接引用] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > @ 0x258e218 -> 0x25925d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > > @ 0x25934fd -> 0x25925d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> > > > @ 0x2593543 -> 0x25925d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::InFlightPackBuild::objects
// 信息 ID: c0c07066fe5e566eef5eeb2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e6a64c8cb4d0ae4c00079711: country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install; module=586, inline_record=23700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install; module=586, inline_record=24016; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$20::fold::flatten::closure$0<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,tuple$<>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::clutter_registry::ClutterId,tuple$<> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0>,hashbrown::set::impl$10::extend::closure_env$0<country_core::resources::clutter_registry::ClutterId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::option::Iter<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::startup::load_clutter_packs::InFlightPackBuild>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)>,country_core::systems::steam::workshop_clutter::impl$0::building_ids::closure_env$0>,country_core::startup::install_mod_clutter::install_mod_clutter::closure_env$0> > > > > > >; module=1682, inline_record=199568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::InFlightPackBuild::poll
// 信息 ID: 9a2f2a5653aa3487cd00fb12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::install_mod_clutter::install_mod_clutter @ 0xbf3171 -> 0x1adc960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf45b8 -> 0x1adc960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::InFlightPackBuild::spawn
// 信息 ID: cc97c4587814c58c866a34f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::install_mod_clutter::install_mod_clutter @ 0xbf33d6 -> 0x1add090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf5e30 -> 0x1add090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs
// 信息 ID: 8ba59cbb5a67d6e43e4650b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_clutter_packs::install_built_clutter_packs @ 0x1ae26e9 -> 0x1addaa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup @ 0x1ae3176 -> 0x1addaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$0
// 信息 ID: 30c31bdcb61c7f0fd8a6c110
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_clutter_packs::apply_built_clutter_packs", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0 <- std::collections::hash::map::impl$78::from_iter<ref$<str$>,turbosloth::asset::AssetRef<asset::texture::blob::TextureAssetBlob>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$78::from_iter<ref$<str$>,turbosloth::asset::AssetRef<asset::texture::blob::TextureAssetBlob>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$0> >; module=1715, inline_record=28364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$1
// 信息 ID: d9340999894c87de3c283e8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_clutter_packs::apply_built_clutter_packs", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,ref$<country_core::resources::clutter_packs::ClutterPackObject>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$1> > > >; module=1682, inline_record=86176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$4
// 信息 ID: f352f96c0302677f76cc255f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::startup::load_clutter_packs::apply_built_clutter_packs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=68760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$5
// 信息 ID: c20bf8567eda70e34908b01a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::startup::load_clutter_packs::apply_built_clutter_packs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=93548; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=94588; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=95628; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=96668; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=97708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure$6
// 信息 ID: ef8b91825d4c5a1dd1500607
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::startup::load_clutter_packs::apply_built_clutter_packs", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::shader::nani::inc::uber_material::UberMaterial,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6 <- core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6 <- core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::shader::nani::inc::uber_material::UberMaterial,core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::shader::nani::inc::uber_material::UberMaterial,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterialEntry>,country_core::startup::load_clutter_packs::apply_built_clutter_packs::closure_env$6> > > >; module=1682, inline_record=93460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::build_requests
// 信息 ID: 2e6bfee1fdaa6e7afc890080
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::workshop_publish::prepare_mod::async_fn$0 @ 0x2a42332 -> 0x1adc140 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::startup::mod_hot_reload::mod_hot_reload_apply; module=565, inline_record=32620; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::InFlightPackBuild::spawn; module=1770, inline_record=26844; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup; module=1770, inline_record=81404; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::build_requests::closure$0
// 信息 ID: 2464fbb67ad6bcfccbccdf5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_clutter_packs::build_requests", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::build_requests::closure_env$0 <- core::iter::adapters::map::impl$2::fold<clutter_pack_pipe::ClutterPackBuildRequest,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::build_requests::closure_env$0 <- core::iter::adapters::map::impl$2::fold<clutter_pack_pipe::ClutterPackBuildRequest,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<clutter_pack_pipe::ClutterPackBuildRequest,core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::vec::impl$21::extend_trusted::closure_env$0<clutter_pack_pipe::ClutterPackBuildRequest,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::clutter_packs::ClutterPackObject>,country_core::startup::load_clutter_packs::build_requests::closure_env$0> > > >; module=1682, inline_record=20064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::discover_clutter_part
// 信息 ID: dd2b802b37dc8bf318361501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 25036a088b745b915cd05f07: enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2dc5c -> 0x1add2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf5afd -> 0x1add2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$11::next<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<tg_mods::overlay::Mod>,enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)>,alloc::vec::into_iter::IntoIter<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > @ 0x15eb967 -> 0x1add2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup @ 0x1ae2a44 -> 0x1add2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0
// 信息 ID: 911349f11c391d76cb2de88b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] async_executor::impl$5::spawn_inner::async_block$0<enum2$<core::result::Result<clutter_pack_pipe::ClutterPackBuild,anyhow::Error> >,enum2$<country_core::startup::load_clutter_packs::impl$0::spawn::async_block_env$0> >; module=3252, inline_record=115148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure$0
// 信息 ID: dc027d58396d751af1ea7dbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<clutter_pack_pipe::ClutterMaterialEntry,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterMaterial>,clutter_pack_pipe::build_clutter_packs::async_fn$0::closure_env$3<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterGlbImport>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> >,core::iter::adapters::map::Map<core::slice::iter::Iter<clutter_pack_pipe::ClutterPackBuildRequest>,clutter_pack_pipe::build_clutter_packs::async_fn$0::closure_env$0<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0 <- clutter_pack_pipe::build_clutter_packs::async_fn$0::closure$1<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0 <- core::ptr::drop_in_place<enum2$<clutter_pack_pipe::build_clutter_packs::async_fn_env$0<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::startup::load_clutter_packs::impl$0::spawn::async_block$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >; module=1651, inline_record=315056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。
// [文件映射] namespace_unique_source_candidate; 作者定义文件是否精确确认仍按证据状态。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::install_built_clutter_packs
// 信息 ID: 5cb319d7842031794e01fd08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2e6a0 -> 0x1ae2670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::InFlightPackBuild::poll @ 0x1adcb95 -> 0x1ae2670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::load_clutter_packs_at_startup
// 信息 ID: d47eabee5183affc598f170b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8a174bb5e1473ce78968c792: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> @ 0x17ee53f -> 0x1ae28f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)> > @ 0x19348e7 -> 0x1ae28f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::mark_workshop_clutter_failed
// 信息 ID: 52130c32a8378146470bb881
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup; module=1770, inline_record=82248; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup; module=1770, inline_record=83328; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup; module=1770, inline_record=85132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::published_item_id
// 信息 ID: 368daf0e36822505eff6667e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup; module=1770, inline_record=79800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::resync_clutter
// 信息 ID: 675177ea28e03ce0675038bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2e858 -> 0x1adc220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::install_mod_clutter::install_mod_clutter @ 0xbf2f49 -> 0x1adc220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf5219 -> 0x1adc220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::install_built_clutter_packs @ 0x1ae274a -> 0x1adc220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::resync_clutter_atlases
// 信息 ID: 5a136910d60e32193a21ec22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::resync_clutter; module=1770, inline_record=15508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::resync_clutter_materials
// 信息 ID: e712940e1ef7497e7c3ce845
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_clutter_packs::resync_clutter @ 0x1adc72f -> 0x1add6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::load_clutter_packs_at_startup @ 0x1ae3434 -> 0x1add6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::unload_clutter
// 信息 ID: 676badfbe9adfd24f38df778
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2e427 -> 0x1adc780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::install_mod_clutter::install_mod_clutter @ 0xbf2d44 -> 0x1adc780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf4e9e -> 0x1adc780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::unload_clutter::closure$0
// 信息 ID: bb46a7c888d6227aa428a429
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_clutter_packs::unload_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::unload_clutter::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; module=1643, inline_record=603688; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; module=1643, inline_record=605372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_clutter_packs::unload_clutter::closure$1
// 信息 ID: a3f1cbed24fe6d99964c888b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_clutter_packs::unload_clutter", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_clutter_packs::unload_clutter::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; module=1643, inline_record=604816; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$0>,country_core::startup::load_clutter_packs::unload_clutter::closure_env$1> >; module=1643, inline_record=606600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
