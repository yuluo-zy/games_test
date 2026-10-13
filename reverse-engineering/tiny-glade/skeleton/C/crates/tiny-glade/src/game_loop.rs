// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/game_loop.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 924d5021d056ccb6562b40f78794a45dd4e57a94df9362211f41e57b6cc31a8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 205
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arboard; // namespace candidate
//   package identities: [{"name":"arboard","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.6.0"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use emath; // namespace candidate
//   package identities: [{"name":"emath","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":6524,"rva":5739152,"va":5374448272}
// 编译实例 RVAs: 0x579290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10196312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e858959b00","size":5,"soft_fail":false,"target_va":5384644912,"va":5374448595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5113504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e860f9b1ff","size":5,"soft_fail":false,"target_va":5369335744,"va":5374449243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5061111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e809c6b2ff","size":5,"soft_fail":false,"target_va":5369388080,"va":5374449186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a8e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5048280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e828f8b2ff","size":5,"soft_fail":false,"target_va":5369400864,"va":5374449139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b34b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5005699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e87d9eb3ff","size":5,"soft_fail":false,"target_va":5369443504,"va":5374449198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bb6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4972439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e86920b4ff","size":5,"soft_fail":false,"target_va":5369476800,"va":5374449234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4947487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e8e181b4ff","size":5,"soft_fail":false,"target_va":5369501728,"va":5374449210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4933579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e835b8b4ff","size":5,"soft_fail":false,"target_va":5369515648,"va":5374449222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42608687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e82f288a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374448364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7548229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e8452d7300","size":5,"soft_fail":false,"target_va":5381997408,"va":5374449174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10277621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374448272,"raw_hex":"e8f5d29c00","size":5,"soft_fail":false,"target_va":5384726688,"va":5374449062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":7332,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":205,"record_offset":6648,"parent_record_offset":6524,"ancestor_record_offset":6524,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":6524,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":205,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":6524,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":205,"record_offset":7088,"parent_record_offset":6884,"ancestor_record_offset":6524,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":6524,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":7216,"parent_record_offset":7160,"ancestor_record_offset":6524,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":6796,"parent_record_offset":6760,"ancestor_record_offset":6524,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6524,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":6524,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":7004,"parent_record_offset":6884,"ancestor_record_offset":6524,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":6524,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7440,"parent_record_offset":7404,"ancestor_record_offset":6524,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":6524,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7496,"parent_record_offset":7404,"ancestor_record_offset":6524,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":205,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":6524,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":205,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6524,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":205,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":6524,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":205,"record_offset":7160,"parent_record_offset":6760,"ancestor_record_offset":6524,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":205,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":6524,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":205,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":6524,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":205,"record_offset":6884,"parent_record_offset":6852,"ancestor_record_offset":6524,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":205,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":6524,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":205,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":6524,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":7544,"rva":5740160,"va":5374449280}
// 编译实例 RVAs: 0x579680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10159118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e80e049b00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374450285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1229080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e818c11200","size":5,"soft_fail":false,"target_va":5375678688,"va":5374449603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1228967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8a7c01200","size":5,"soft_fail":false,"target_va":5375678704,"va":5374449732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5727401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8579ba8ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374449588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5110907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e88503b2ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374449750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5107277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8b311b2ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5374450776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5077024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8e087b2ff","size":5,"soft_fail":false,"target_va":5369373296,"va":5374450315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019fa50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4038057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e85762c2ff","size":5,"soft_fail":false,"target_va":5370411600,"va":5374449652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5266211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8235b5000","size":5,"soft_fail":false,"target_va":5379716240,"va":5374450024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42170374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e806788302","size":5,"soft_fail":false,"target_va":5416620794,"va":5374450415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9187322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8fa2f8c00","size":5,"soft_fail":false,"target_va":5383637632,"va":5374450305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9962452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8d4039800","size":5,"soft_fail":false,"target_va":5384412464,"va":5374450007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42171255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8777b8302","size":5,"soft_fail":false,"target_va":5416621600,"va":5374450340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42171133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8fd7a8302","size":5,"soft_fail":false,"target_va":5416621632,"va":5374450494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42126449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e871cc8202","size":5,"soft_fail":false,"target_va":5416576832,"va":5374450378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42125745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8b1c98202","size":5,"soft_fail":false,"target_va":5416576144,"va":5374450394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9534784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8407d9100","size":5,"soft_fail":false,"target_va":5383984848,"va":5374450059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9535273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8297f9100","size":5,"soft_fail":false,"target_va":5383985360,"va":5374450082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10302755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e823359d00","size":5,"soft_fail":false,"target_va":5384752464,"va":5374449704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::main_loop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374449280,"raw_hex":"e8cc050000","size":5,"soft_fail":false,"target_va":5374450800,"va":5374449311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::game_loop::shut_down_everything
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":7664,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":16246,"source_line":1127,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":7544,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":7544,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":7544,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":7544,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":7544,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":7544,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":7544,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10132,"parent_record_offset":10076,"ancestor_record_offset":7544,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9176,"parent_record_offset":9152,"ancestor_record_offset":7544,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":7544,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":7544,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":7544,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9224,"parent_record_offset":9200,"ancestor_record_offset":7544,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":7544,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":7544,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":7544,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":7544,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":12864,"parent_record_offset":12828,"ancestor_record_offset":7544,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":12828,"parent_record_offset":12800,"ancestor_record_offset":7544,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":7544,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":7544,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12432,"parent_record_offset":12392,"ancestor_record_offset":7544,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":7544,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9128,"parent_record_offset":9088,"ancestor_record_offset":7544,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9612,"parent_record_offset":9552,"ancestor_record_offset":7544,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9640,"parent_record_offset":9612,"ancestor_record_offset":7544,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":7544,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":7544,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":7884,"parent_record_offset":7856,"ancestor_record_offset":7544,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":7544,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":7544,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9372,"parent_record_offset":9340,"ancestor_record_offset":7544,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9316,"parent_record_offset":8808,"ancestor_record_offset":7544,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":7544,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":12620,"parent_record_offset":11184,"ancestor_record_offset":7544,"inlinee":9512,"source_line":249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":8836,"parent_record_offset":8808,"ancestor_record_offset":7544,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":12140,"parent_record_offset":11184,"ancestor_record_offset":7544,"inlinee":9508,"source_line":344,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9400,"parent_record_offset":9340,"ancestor_record_offset":7544,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":7544,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":7544,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":7544,"inlinee":9509,"source_line":415,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":7544,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":7544,"inlinee":9511,"source_line":417,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":7832,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":11184,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":9507,"source_line":1807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":7544,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":8808,"parent_record_offset":7856,"ancestor_record_offset":7544,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7544,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":205,"record_offset":12712,"parent_record_offset":11184,"ancestor_record_offset":7544,"inlinee":9513,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":7544,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":7544,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":13324,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":205,"record_offset":10556,"parent_record_offset":10524,"ancestor_record_offset":7544,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7544,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":7544,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":7544,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":7544,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":8976,"parent_record_offset":8912,"ancestor_record_offset":7544,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":7544,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":7544,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":9040,"parent_record_offset":8860,"ancestor_record_offset":7544,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":13708,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":7544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":10004,"parent_record_offset":9552,"ancestor_record_offset":7544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":7544,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":7544,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":7544,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":7544,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":13764,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":7544,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":8948,"parent_record_offset":8912,"ancestor_record_offset":7544,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":9268,"parent_record_offset":9088,"ancestor_record_offset":7544,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":12572,"parent_record_offset":12392,"ancestor_record_offset":7544,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":7544,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13168,"parent_record_offset":13104,"ancestor_record_offset":7544,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":12956,"parent_record_offset":12920,"ancestor_record_offset":7544,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":7544,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":7544,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":7544,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":7544,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":7544,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13636,"parent_record_offset":13576,"ancestor_record_offset":7544,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13668,"parent_record_offset":13540,"ancestor_record_offset":7544,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":7544,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":7544,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8300,"parent_record_offset":8268,"ancestor_record_offset":7544,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8240,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8100,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8660,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8184,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":7944,"parent_record_offset":7908,"ancestor_record_offset":7544,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":8508,"parent_record_offset":8400,"ancestor_record_offset":7544,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":7544,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8752,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":7544,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":7544,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":7544,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7544,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8072,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8400,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":8632,"parent_record_offset":8020,"ancestor_record_offset":7544,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":7544,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":7544,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":13380,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":7544,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":7544,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13240,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":12920,"parent_record_offset":12828,"ancestor_record_offset":7544,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":13104,"parent_record_offset":13076,"ancestor_record_offset":7544,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":13012,"parent_record_offset":12920,"ancestor_record_offset":7544,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":13140,"parent_record_offset":13104,"ancestor_record_offset":7544,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::with_global_allocator
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":205,"record_offset":12800,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":8375,"source_line":877,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":205,"record_offset":9972,"parent_record_offset":9888,"ancestor_record_offset":7544,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9916,"parent_record_offset":9888,"ancestor_record_offset":7544,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":7544,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9504,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":8444,"source_line":2245,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9552,"parent_record_offset":9504,"ancestor_record_offset":7544,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9888,"parent_record_offset":9552,"ancestor_record_offset":7544,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":205,"record_offset":10524,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::crash_service::mark_shutting_down
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":205,"record_offset":7748,"parent_record_offset":7664,"ancestor_record_offset":7544,"inlinee":15327,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::receive_app_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F7B
// 首条位置: {"module":205,"inlinee":16251}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":35368,"parent_record_offset":35324,"ancestor_record_offset":35324,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":35612,"parent_record_offset":35324,"ancestor_record_offset":35324,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":35488,"parent_record_offset":35456,"ancestor_record_offset":35324,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":35536,"parent_record_offset":35512,"ancestor_record_offset":35324,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":35512,"parent_record_offset":35488,"ancestor_record_offset":35324,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":35456,"parent_record_offset":35404,"ancestor_record_offset":35324,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":35404,"parent_record_offset":35368,"ancestor_record_offset":35324,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":35576,"parent_record_offset":35404,"ancestor_record_offset":35324,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":205,"record_offset":35636,"parent_record_offset":35612,"ancestor_record_offset":35324,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::send_window_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F7C
// 首条位置: {"module":205,"inlinee":16252}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":37516,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37484,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":35784,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":37428,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":37400,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":37372,"parent_record_offset":35684,"ancestor_record_offset":35684,"inlinee":16254,"source_line":1185,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38168,"parent_record_offset":38144,"ancestor_record_offset":35684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38192,"parent_record_offset":38168,"ancestor_record_offset":35684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38216,"parent_record_offset":38192,"ancestor_record_offset":35684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38144,"parent_record_offset":38056,"ancestor_record_offset":35684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":37128,"parent_record_offset":37104,"ancestor_record_offset":35684,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38084,"parent_record_offset":38056,"ancestor_record_offset":35684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38056,"parent_record_offset":38032,"ancestor_record_offset":35684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":37176,"parent_record_offset":37152,"ancestor_record_offset":35684,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":37152,"parent_record_offset":37128,"ancestor_record_offset":35684,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38032,"parent_record_offset":38008,"ancestor_record_offset":35684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":37104,"parent_record_offset":37080,"ancestor_record_offset":35684,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":37080,"parent_record_offset":37040,"ancestor_record_offset":35684,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":37040,"parent_record_offset":37016,"ancestor_record_offset":35684,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37936,"parent_record_offset":37912,"ancestor_record_offset":35684,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":35684,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":35836,"parent_record_offset":35808,"ancestor_record_offset":35684,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":37324,"parent_record_offset":37292,"ancestor_record_offset":35684,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":37268,"parent_record_offset":36760,"ancestor_record_offset":35684,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":37292,"parent_record_offset":37268,"ancestor_record_offset":35684,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":35684,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":36836,"parent_record_offset":36812,"ancestor_record_offset":35684,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":36812,"parent_record_offset":36788,"ancestor_record_offset":35684,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":35684,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":35808,"parent_record_offset":35784,"ancestor_record_offset":35684,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":36760,"parent_record_offset":35808,"ancestor_record_offset":35684,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":35860,"parent_record_offset":35836,"ancestor_record_offset":35684,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":36508,"parent_record_offset":36484,"ancestor_record_offset":35684,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":36536,"parent_record_offset":36508,"ancestor_record_offset":35684,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":36076,"parent_record_offset":36052,"ancestor_record_offset":35684,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":36160,"parent_record_offset":36136,"ancestor_record_offset":35684,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":205,"record_offset":37736,"parent_record_offset":37712,"ancestor_record_offset":35684,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":205,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":35684,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":38112,"parent_record_offset":38084,"ancestor_record_offset":35684,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":36308,"parent_record_offset":36280,"ancestor_record_offset":35684,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":36280,"parent_record_offset":36252,"ancestor_record_offset":35684,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":36928,"parent_record_offset":36864,"ancestor_record_offset":35684,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":36952,"parent_record_offset":36928,"ancestor_record_offset":35684,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":36992,"parent_record_offset":36812,"ancestor_record_offset":35684,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":36100,"parent_record_offset":36076,"ancestor_record_offset":35684,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":37452,"parent_record_offset":37428,"ancestor_record_offset":35684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":36424,"parent_record_offset":36400,"ancestor_record_offset":35684,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37564,"parent_record_offset":37540,"ancestor_record_offset":35684,"inlinee":16256,"source_line":1148,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37540,"parent_record_offset":37516,"ancestor_record_offset":35684,"inlinee":16255,"source_line":1168,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37588,"parent_record_offset":37564,"ancestor_record_offset":35684,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":37852,"parent_record_offset":37796,"ancestor_record_offset":35684,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":37824,"parent_record_offset":37796,"ancestor_record_offset":35684,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":35684,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":36864,"parent_record_offset":36836,"ancestor_record_offset":35684,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":36900,"parent_record_offset":36864,"ancestor_record_offset":35684,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":37220,"parent_record_offset":37040,"ancestor_record_offset":35684,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":36484,"parent_record_offset":36460,"ancestor_record_offset":35684,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":36220,"parent_record_offset":36192,"ancestor_record_offset":35684,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":36252,"parent_record_offset":36220,"ancestor_record_offset":35684,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":36192,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":36052,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":36612,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":36136,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":35896,"parent_record_offset":35860,"ancestor_record_offset":35684,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":36460,"parent_record_offset":36352,"ancestor_record_offset":35684,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":36400,"parent_record_offset":36376,"ancestor_record_offset":35684,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":36704,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":36376,"parent_record_offset":36352,"ancestor_record_offset":35684,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":35948,"parent_record_offset":35924,"ancestor_record_offset":35684,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":35924,"parent_record_offset":35896,"ancestor_record_offset":35684,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":35972,"parent_record_offset":35948,"ancestor_record_offset":35684,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":36024,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":36352,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":36584,"parent_record_offset":35972,"ancestor_record_offset":35684,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::main_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":14020,"rva":5741680,"va":5374450800}
// 编译实例 RVAs: 0x579c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10326952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8a8939d00","size":5,"soft_fail":false,"target_va":5384795984,"va":5374469027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42148556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8cc228302","size":5,"soft_fail":false,"target_va":5416616112,"va":5374467551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t42156429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88d418302","size":5,"soft_fail":false,"target_va":5416616304,"va":5374459870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant as core::ops::arith::Add<core::time::Duration>>::add
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t10297520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8b0209d00","size":5,"soft_fail":false,"target_va":5384768864,"va":5374471339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10173931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8eb3d9b00","size":5,"soft_fail":false,"target_va":5384648448,"va":5374474512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t10176706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c2489b00","size":5,"soft_fail":false,"target_va":5384648032,"va":5374471321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..data..input..RawInput$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8155371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8eb707c00","size":5,"soft_fail":false,"target_va":5382632656,"va":5374477280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10157771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8cbfe9a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374451632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$smol_str..SmolStr$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10144860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e85ccc9a00","size":5,"soft_fail":false,"target_va":5384609536,"va":5374464671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t1222526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87ea71200","size":5,"soft_fail":false,"target_va":5375678704,"va":5374456173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=2
//     target_aliases=122 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5220605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e80357b0ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5374485720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5175709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e86306b1ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5374481288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009b3c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5106711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8e913b2ff","size":5,"soft_fail":false,"target_va":5369344960,"va":5374451666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009c0d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5130505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8f7b6b1ff","size":5,"soft_fail":false,"target_va":5369348304,"va":5374478804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5078317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d382b2ff","size":5,"soft_fail":false,"target_va":5369387888,"va":5374466200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a90f0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5054325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88be0b2ff","size":5,"soft_fail":false,"target_va":5369401584,"va":5374455904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400adf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5069532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e824a5b2ff","size":5,"soft_fail":false,"target_va":5369421680,"va":5374491207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2170
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5042999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c90cb3ff","size":5,"soft_fail":false,"target_va":5369438576,"va":5374481570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6390
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5006323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e80d9cb3ff","size":5,"soft_fail":false,"target_va":5369455504,"va":5374461822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400be890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4990564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e89cd9b3ff","size":5,"soft_fail":false,"target_va":5369489552,"va":5374480111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4440
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4954978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e89e64b4ff","size":5,"soft_fail":false,"target_va":5369513024,"va":5374467997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c6a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4960369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88f4fb4ff","size":5,"soft_fail":false,"target_va":5369522832,"va":5374483196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400faa90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4732937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8f7c7b7ff","size":5,"soft_fail":false,"target_va":5369735824,"va":5374468756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140133c40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4504702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88243bbff","size":5,"soft_fail":false,"target_va":5369969728,"va":5374474425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4482862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d298bbff","size":5,"soft_fail":false,"target_va":5369992480,"va":5374475337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d04c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3841860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8bc60c5ff","size":5,"soft_fail":false,"target_va":5370610880,"va":5374452735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d0a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3842531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e81d5ec5ff","size":5,"soft_fail":false,"target_va":5370612352,"va":5374454878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d1050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3839209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8176bc5ff","size":5,"soft_fail":false,"target_va":5370613840,"va":5374453044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d1590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3838442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8166ec5ff","size":5,"soft_fail":false,"target_va":5370615184,"va":5374453621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d1b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3838514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8ce6dc5ff","size":5,"soft_fail":false,"target_va":5370616656,"va":5374455165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d2110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3835195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c57ac5ff","size":5,"soft_fail":false,"target_va":5370618128,"va":5374453318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d2670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3834443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8b57dc5ff","size":5,"soft_fail":false,"target_va":5370619504,"va":5374453942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d2bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3833418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8b681c5ff","size":5,"soft_fail":false,"target_va":5370620848,"va":5374454261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ced90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1760026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8e624e5ff","size":5,"soft_fail":false,"target_va":5372702096,"va":5374462117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040d320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1505640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e89806e9ff","size":5,"soft_fail":false,"target_va":5372957472,"va":5374463107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ce40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-981545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d705f1ff","size":5,"soft_fail":false,"target_va":5373480512,"va":5374462052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048fc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-985711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e891f5f0ff","size":5,"soft_fail":false,"target_va":5373492320,"va":5374478026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140492110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-968579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87d38f1ff","size":5,"soft_fail":false,"target_va":5373501712,"va":5374470286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bf0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-789612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e894f3f3ff","size":5,"soft_fail":false,"target_va":5373685920,"va":5374475527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-372578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e89e50faff","size":5,"soft_fail":false,"target_va":5374098704,"va":5374471277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140646ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t822211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c38b0c00","size":5,"soft_fail":false,"target_va":5375291072,"va":5374468856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140646ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t822156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88c8b0c00","size":5,"soft_fail":false,"target_va":5375291376,"va":5374469215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140647120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t822388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8748c0c00","size":5,"soft_fail":false,"target_va":5375291680,"va":5374469287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140647250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t835937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e861c10c00","size":5,"soft_fail":false,"target_va":5375291984,"va":5374456042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140647400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t836491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88bc30c00","size":5,"soft_fail":false,"target_va":5375292416,"va":5374455920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406475a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t837060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c4c50c00","size":5,"soft_fail":false,"target_va":5375292832,"va":5374455767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140647750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t836979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e873c50c00","size":5,"soft_fail":false,"target_va":5375293264,"va":5374456280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=6
//     target_aliases=77 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1373909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d5f61400","size":5,"soft_fail":false,"target_va":5375829056,"va":5374455142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb490
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3649972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8b4b13700","size":5,"soft_fail":false,"target_va":5378126992,"va":5374477015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5246783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e83f0f5000","size":5,"soft_fail":false,"target_va":5379716240,"va":5374469452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d080
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7130994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e872cf6c00","size":5,"soft_fail":false,"target_va":5381607552,"va":5374476553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41884632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d81b7f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5374465747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41889267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8f32d7f02","size":5,"soft_fail":false,"target_va":5416350496,"va":5374461224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41890158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e86e317f02","size":5,"soft_fail":false,"target_va":5416350720,"va":5374460557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41890565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e805337f02","size":5,"soft_fail":false,"target_va":5416350944,"va":5374460374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41889634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8622f7f02","size":5,"soft_fail":false,"target_va":5416351056,"va":5374461417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f5c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41895656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8e8467f02","size":5,"soft_fail":false,"target_va":5416351168,"va":5374455507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d713d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41880031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8df097f02","size":5,"soft_fail":false,"target_va":5416358864,"va":5374478828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42605608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8281c8a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374451443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t1483054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e82ea11600","size":5,"soft_fail":false,"target_va":5375949040,"va":5374465981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::get_text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1303890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e852e51300","size":5,"soft_fail":false,"target_va":5375768096,"va":5374464201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8906533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e825e78700","size":5,"soft_fail":false,"target_va":5383381824,"va":5374475286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::messages::Messages$LT$M$GT$::write_default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-871076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e85cb5f2ff","size":5,"soft_fail":false,"target_va":5373583456,"va":5374454527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9959788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e86cf99700","size":5,"soft_fail":false,"target_va":5384412464,"va":5374452671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42140799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87f048302","size":5,"soft_fail":false,"target_va":5416621536,"va":5374480732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=48
//     evidence={"assembly":"\tcallq\t42142638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8ae0b8302","size":5,"soft_fail":false,"target_va":5416621600,"va":5374478957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42143303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8470e8302","size":5,"soft_fail":false,"target_va":5416622240,"va":5374478932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<bevy_input::keyboard::Key> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-433194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d663f9ff","size":5,"soft_fail":false,"target_va":5374049168,"va":5374482357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::game_os_events::OsWindowEvent> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d6160000","size":5,"soft_fail":false,"target_va":5374491344,"va":5374485493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42141117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8bd058302","size":5,"soft_fail":false,"target_va":5416621632,"va":5374480510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42140571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e89b038302","size":5,"soft_fail":false,"target_va":5416621376,"va":5374480800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::is_profiling
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4909295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8efe84a00","size":5,"soft_fail":false,"target_va":5379387136,"va":5374477836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::DisplayConfig::fullscreen_exclusive
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5246259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8330d5000","size":5,"soft_fail":false,"target_va":5379716480,"va":5374470216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::UpscalingPreset::tgsr_resolution_scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5244446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e81e065000","size":5,"soft_fail":false,"target_va":5379716496,"va":5374472045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::impl$42::clone
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t30906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8ba780000","size":5,"soft_fail":false,"target_va":5374491840,"va":5374460929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7131027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e893cf6c00","size":5,"soft_fail":false,"target_va":5381607568,"va":5374476536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::finish_wait
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6333425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8f1a36000","size":5,"soft_fail":false,"target_va":5380785376,"va":5374451946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_finished
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6333213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e81da36000","size":5,"soft_fail":false,"target_va":5380785392,"va":5374452174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_waiting
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6334061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e86da66000","size":5,"soft_fail":false,"target_va":5380785360,"va":5374451294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4073563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e85b283e00","size":5,"soft_fail":false,"target_va":5378529696,"va":5374456128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7293575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8874a6f00","size":5,"soft_fail":false,"target_va":5381756016,"va":5374462436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::egui_input::EguiInputResource::on_keyboard_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5521018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87a3e5400","size":5,"soft_fail":false,"target_va":5379987168,"va":5374466145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::egui_input::EguiInputResource::on_mouse_button
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5524941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8cd4d5400","size":5,"soft_fail":false,"target_va":5379986928,"va":5374461982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::egui_input::EguiInputResource::on_mouse_wheel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5520342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d63b5400","size":5,"soft_fail":false,"target_va":5379986272,"va":5374465925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3957112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e878613c00","size":5,"soft_fail":false,"target_va":5378411968,"va":5374454851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::begin_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8171047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e827ae7c00","size":5,"soft_fail":false,"target_va":5382649776,"va":5374478724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::wants_keyboard_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8212827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e85b517d00","size":5,"soft_fail":false,"target_va":5382676736,"va":5374463904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::wants_pointer_input
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8214923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e88b597d00","size":5,"soft_fail":false,"target_va":5382674976,"va":5374460048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t42097274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87a5a8202","size":5,"soft_fail":false,"target_va":5416576144,"va":5374478865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9510789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8851f9100","size":5,"soft_fail":false,"target_va":5383985360,"va":5374474566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9488345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8d9c79000","size":5,"soft_fail":false,"target_va":5383962880,"va":5374474530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::new_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9488837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c5c99000","size":5,"soft_fail":false,"target_va":5383963392,"va":5374474550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9488682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e82ac99000","size":5,"soft_fail":false,"target_va":5383962816,"va":5374474129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9260022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8f64b8d00","size":5,"soft_fail":false,"target_va":5383729488,"va":5374469461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9256933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8e53f8d00","size":5,"soft_fail":false,"target_va":5383730768,"va":5374473830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::any_present_successful
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9421075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e813c18f00","size":5,"soft_fail":false,"target_va":5383889728,"va":5374468648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::desc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9419920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e890bc8f00","size":5,"soft_fail":false,"target_va":5383890160,"va":5374470235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::mark_stale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9426537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e869d68f00","size":5,"soft_fail":false,"target_va":5383884160,"va":5374457618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::set_allow_triple_buffering
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9420824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e818c08f00","size":5,"soft_fail":false,"target_va":5383889744,"va":5374468915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::FallibleSwapchain::set_vsync
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9424210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e852cd8f00","size":5,"soft_fail":false,"target_va":5383893104,"va":5374468889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::rcache::RenderResourceCache::clear_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9379649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8411f8f00","size":5,"soft_fail":false,"target_va":5383853232,"va":5374473578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42152087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e897308302","size":5,"soft_fail":false,"target_va":5416619696,"va":5374467604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-4756517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8db6bb7ff","size":5,"soft_fail":false,"target_va":5369699360,"va":5374455872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-161244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8248afdff","size":5,"soft_fail":false,"target_va":5374290144,"va":5374451383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4635700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8cc43b9ff","size":5,"soft_fail":false,"target_va":5369815664,"va":5374451359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1826892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8b41fe4ff","size":5,"soft_fail":false,"target_va":5372624512,"va":5374451399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10259532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e84c8c9c00","size":5,"soft_fail":false,"target_va":5384728160,"va":5374468623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::egui_backend::EguiBackend::end_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3912122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8464ec4ff","size":5,"soft_fail":false,"target_va":5370563248,"va":5374475365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fps_limiter::FpsLimiter::frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3923038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8a223c4ff","size":5,"soft_fail":false,"target_va":5370554848,"va":5374477881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::on_cursor_moved_absolute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c8ddffff","size":5,"soft_fail":false,"target_va":5374447056,"va":5374455811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::log_time_to_first_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t790139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e87b0e0c00","size":5,"soft_fail":false,"target_va":5375266240,"va":5374476096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-73923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e83ddffeff","size":5,"soft_fail":false,"target_va":5374401920,"va":5374475838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_max_resolution_scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1003971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e8c3510f00","size":5,"soft_fail":false,"target_va":5375473712,"va":5374469736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_clipboard_paste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1739026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e812891a00","size":5,"soft_fail":false,"target_va":5376206160,"va":5374467129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_key_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1739322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374450800,"raw_hex":"e83a8a1a00","size":5,"soft_fail":false,"target_va":5376205904,"va":5374466577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":38956,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":14236,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":205,"record_offset":39276,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":18216,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":14264,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":30216,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":20312,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":9507,"source_line":1807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":16388,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":9411,"source_line":2014,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":16192,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":205,"record_offset":39248,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":15984,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":20000,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":21940,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":16360,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":16248,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":16108,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":16040,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":159884,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::apply_setting_changes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":159972,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":16284,"source_line":1080,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::process_app_event
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":39480,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":16257,"source_line":330,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::receive_app_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":35324,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":16251,"source_line":130,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::send_window_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":35684,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":16252,"source_line":147,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::simulate_and_render_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":102140,"parent_record_offset":14020,"ancestor_record_offset":14020,"inlinee":16283,"source_line":591,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38168,"parent_record_offset":38144,"ancestor_record_offset":14020,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38192,"parent_record_offset":38168,"ancestor_record_offset":14020,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38216,"parent_record_offset":38192,"ancestor_record_offset":14020,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":38144,"parent_record_offset":38056,"ancestor_record_offset":14020,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":15656,"parent_record_offset":15632,"ancestor_record_offset":14020,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38084,"parent_record_offset":38056,"ancestor_record_offset":14020,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38056,"parent_record_offset":38032,"ancestor_record_offset":14020,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":14020,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":14020,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":38032,"parent_record_offset":38008,"ancestor_record_offset":14020,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":87828,"parent_record_offset":87796,"ancestor_record_offset":14020,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":87796,"parent_record_offset":87772,"ancestor_record_offset":14020,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":88820,"parent_record_offset":88796,"ancestor_record_offset":14020,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":14020,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":21564,"parent_record_offset":21524,"ancestor_record_offset":14020,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":15632,"parent_record_offset":15608,"ancestor_record_offset":14020,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":15608,"parent_record_offset":15568,"ancestor_record_offset":14020,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":39104,"parent_record_offset":38956,"ancestor_record_offset":14020,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":35368,"parent_record_offset":35324,"ancestor_record_offset":14020,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":35404,"parent_record_offset":35368,"ancestor_record_offset":14020,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":39132,"parent_record_offset":38956,"ancestor_record_offset":14020,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":205,"record_offset":39416,"parent_record_offset":39304,"ancestor_record_offset":14020,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":14020,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":37516,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":62420,"parent_record_offset":62392,"ancestor_record_offset":14020,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":14020,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37936,"parent_record_offset":37912,"ancestor_record_offset":14020,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":14020,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":205,"record_offset":37484,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":18240,"parent_record_offset":18216,"ancestor_record_offset":14020,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":62308,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":14336,"parent_record_offset":14296,"ancestor_record_offset":14020,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$43::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":205,"record_offset":88456,"parent_record_offset":87576,"ancestor_record_offset":14020,"inlinee":16280,"source_line":385,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68228,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68572,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68252,"parent_record_offset":68228,"ancestor_record_offset":14020,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=52
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":15928,"parent_record_offset":15904,"ancestor_record_offset":14020,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=52
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":15904,"parent_record_offset":15880,"ancestor_record_offset":14020,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":15852,"parent_record_offset":15820,"ancestor_record_offset":14020,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":15796,"parent_record_offset":15288,"ancestor_record_offset":14020,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":17912,"parent_record_offset":17400,"ancestor_record_offset":14020,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":14020,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":21752,"parent_record_offset":20312,"ancestor_record_offset":14020,"inlinee":9512,"source_line":249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=57
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14020,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":21272,"parent_record_offset":20312,"ancestor_record_offset":14020,"inlinee":9508,"source_line":344,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":15880,"parent_record_offset":15820,"ancestor_record_offset":14020,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":17996,"parent_record_offset":17936,"ancestor_record_offset":14020,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":14020,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=57
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":15340,"parent_record_offset":15316,"ancestor_record_offset":14020,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":14020,"inlinee":9509,"source_line":415,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=57
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":14020,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":21500,"parent_record_offset":21476,"ancestor_record_offset":14020,"inlinee":9511,"source_line":417,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":134360,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":35784,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":16424,"parent_record_offset":16388,"ancestor_record_offset":14020,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":18112,"parent_record_offset":18088,"ancestor_record_offset":14020,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":135448,"parent_record_offset":135420,"ancestor_record_offset":14020,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":18088,"parent_record_offset":17400,"ancestor_record_offset":14020,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":14296,"parent_record_offset":14264,"ancestor_record_offset":14020,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":15288,"parent_record_offset":14296,"ancestor_record_offset":14020,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":16448,"parent_record_offset":16424,"ancestor_record_offset":14020,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":17400,"parent_record_offset":16448,"ancestor_record_offset":14020,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/lib.rs
//     evidence={"module":205,"record_offset":88428,"parent_record_offset":87576,"ancestor_record_offset":14020,"inlinee":16276,"source_line":137,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$2::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87576,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":16272,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$34::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87712,"parent_record_offset":87652,"ancestor_record_offset":14020,"inlinee":16282,"source_line":749,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$46::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":85244,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":16269,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$48::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87652,"parent_record_offset":87576,"ancestor_record_offset":14020,"inlinee":16274,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":14368,"parent_record_offset":14336,"ancestor_record_offset":14020,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":205,"record_offset":21840,"parent_record_offset":20312,"ancestor_record_offset":14020,"inlinee":9513,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":14020,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":15064,"parent_record_offset":15036,"ancestor_record_offset":14020,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113184,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":16288,"source_line":14,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113240,"parent_record_offset":113212,"ancestor_record_offset":14020,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113212,"parent_record_offset":113184,"ancestor_record_offset":14020,"inlinee":16289,"source_line":159,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":205,"record_offset":113124,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":205,"record_offset":134116,"parent_record_offset":134088,"ancestor_record_offset":14020,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":43936,"parent_record_offset":43912,"ancestor_record_offset":14020,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":77276,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":126504,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":62248,"parent_record_offset":62224,"ancestor_record_offset":14020,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":49372,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119876,"parent_record_offset":119848,"ancestor_record_offset":14020,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":124528,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":85272,"parent_record_offset":85244,"ancestor_record_offset":14020,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":205,"record_offset":85132,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":205,"record_offset":89452,"parent_record_offset":89428,"ancestor_record_offset":14020,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14020,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=130
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":14020,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":205,"record_offset":120376,"parent_record_offset":113280,"ancestor_record_offset":14020,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":205,"record_offset":155752,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":119740,"parent_record_offset":119680,"ancestor_record_offset":14020,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":205,"record_offset":37736,"parent_record_offset":37712,"ancestor_record_offset":14020,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":205,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":14020,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":20024,"parent_record_offset":20000,"ancestor_record_offset":14020,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":152100,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":38112,"parent_record_offset":38084,"ancestor_record_offset":14020,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119776,"parent_record_offset":119636,"ancestor_record_offset":14020,"inlinee":5654,"source_line":761,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119680,"parent_record_offset":119636,"ancestor_record_offset":14020,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119708,"parent_record_offset":119680,"ancestor_record_offset":14020,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":14020,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":14808,"parent_record_offset":14780,"ancestor_record_offset":14020,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=68
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":15456,"parent_record_offset":15392,"ancestor_record_offset":14020,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":85332,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=67
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":14020,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":119812,"parent_record_offset":119604,"ancestor_record_offset":14020,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":14020,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":15520,"parent_record_offset":15340,"ancestor_record_offset":14020,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":96836,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":126536,"parent_record_offset":126504,"ancestor_record_offset":14020,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":87948,"parent_record_offset":87652,"ancestor_record_offset":14020,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14020,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=183
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":37428,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":39208,"parent_record_offset":39184,"ancestor_record_offset":14020,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":21868,"parent_record_offset":21840,"ancestor_record_offset":14020,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":14020,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37564,"parent_record_offset":37540,"ancestor_record_offset":14020,"inlinee":16256,"source_line":1148,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37540,"parent_record_offset":37516,"ancestor_record_offset":14020,"inlinee":16255,"source_line":1168,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":37588,"parent_record_offset":37564,"ancestor_record_offset":14020,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":39388,"parent_record_offset":39304,"ancestor_record_offset":14020,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":87864,"parent_record_offset":87828,"ancestor_record_offset":14020,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":133000,"parent_record_offset":132972,"ancestor_record_offset":14020,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":37852,"parent_record_offset":37796,"ancestor_record_offset":14020,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":132972,"parent_record_offset":132944,"ancestor_record_offset":14020,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":37824,"parent_record_offset":37796,"ancestor_record_offset":14020,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":21896,"parent_record_offset":21868,"ancestor_record_offset":14020,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":35576,"parent_record_offset":35404,"ancestor_record_offset":14020,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":85380,"parent_record_offset":85356,"ancestor_record_offset":14020,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":63936,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":205,"record_offset":133312,"parent_record_offset":133284,"ancestor_record_offset":14020,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":133284,"parent_record_offset":133256,"ancestor_record_offset":14020,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":15392,"parent_record_offset":15364,"ancestor_record_offset":14020,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":39160,"parent_record_offset":38956,"ancestor_record_offset":14020,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":15428,"parent_record_offset":15392,"ancestor_record_offset":14020,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":15748,"parent_record_offset":15568,"ancestor_record_offset":14020,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":21704,"parent_record_offset":21524,"ancestor_record_offset":14020,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":205,"record_offset":39184,"parent_record_offset":39160,"ancestor_record_offset":14020,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132916,"parent_record_offset":132888,"ancestor_record_offset":14020,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132944,"parent_record_offset":132916,"ancestor_record_offset":14020,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":133340,"parent_record_offset":133312,"ancestor_record_offset":14020,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":39304,"parent_record_offset":39276,"ancestor_record_offset":14020,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132888,"parent_record_offset":132764,"ancestor_record_offset":14020,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":205,"record_offset":132584,"parent_record_offset":132548,"ancestor_record_offset":14020,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":205,"record_offset":132732,"parent_record_offset":132700,"ancestor_record_offset":14020,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":205,"record_offset":132700,"parent_record_offset":132668,"ancestor_record_offset":14020,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132640,"parent_record_offset":132612,"ancestor_record_offset":14020,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132668,"parent_record_offset":132640,"ancestor_record_offset":14020,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132612,"parent_record_offset":132584,"ancestor_record_offset":14020,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":133256,"parent_record_offset":132668,"ancestor_record_offset":14020,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":205,"record_offset":133432,"parent_record_offset":133400,"ancestor_record_offset":14020,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":205,"record_offset":133400,"parent_record_offset":132548,"ancestor_record_offset":14020,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":132764,"parent_record_offset":132732,"ancestor_record_offset":14020,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":133080,"parent_record_offset":132764,"ancestor_record_offset":14020,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":133048,"parent_record_offset":132764,"ancestor_record_offset":14020,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":132824,"parent_record_offset":132764,"ancestor_record_offset":14020,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63856,"parent_record_offset":63828,"ancestor_record_offset":14020,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64264,"parent_record_offset":64076,"ancestor_record_offset":14020,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":133760,"parent_record_offset":133732,"ancestor_record_offset":14020,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":14020,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63544,"parent_record_offset":63512,"ancestor_record_offset":14020,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64360,"parent_record_offset":64332,"ancestor_record_offset":14020,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":16132,"parent_record_offset":16108,"ancestor_record_offset":14020,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":87896,"parent_record_offset":87828,"ancestor_record_offset":14020,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63568,"parent_record_offset":63544,"ancestor_record_offset":14020,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":14020,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64292,"parent_record_offset":64264,"ancestor_record_offset":14020,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":88916,"parent_record_offset":88856,"ancestor_record_offset":14020,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64384,"parent_record_offset":64360,"ancestor_record_offset":14020,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":88948,"parent_record_offset":88820,"ancestor_record_offset":14020,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119636,"parent_record_offset":119604,"ancestor_record_offset":14020,"inlinee":16293,"source_line":782,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119336,"parent_record_offset":119312,"ancestor_record_offset":14020,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119604,"parent_record_offset":113280,"ancestor_record_offset":14020,"inlinee":16292,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":205,"record_offset":115360,"parent_record_offset":113280,"ancestor_record_offset":14020,"inlinee":13678,"source_line":5,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160804,"parent_record_offset":160776,"ancestor_record_offset":14020,"inlinee":11523,"source_line":10,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160776,"parent_record_offset":160736,"ancestor_record_offset":14020,"inlinee":11521,"source_line":19,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160872,"parent_record_offset":160836,"ancestor_record_offset":14020,"inlinee":11479,"source_line":36,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160736,"parent_record_offset":159972,"ancestor_record_offset":14020,"inlinee":11519,"source_line":54,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":205,"record_offset":155932,"parent_record_offset":155904,"ancestor_record_offset":14020,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::pixels_per_point
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":205,"record_offset":155904,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":9292,"source_line":1595,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] emath::pos2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/pos2.rs
//     evidence={"module":205,"record_offset":155996,"parent_record_offset":155968,"ancestor_record_offset":14020,"inlinee":6034,"source_line":251,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] emath::rect::Rect::from_min_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/rect.rs
//     evidence={"module":205,"record_offset":155968,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":6052,"source_line":78,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":14020,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":14020,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":14780,"parent_record_offset":14748,"ancestor_record_offset":14020,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":14720,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":14580,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":15140,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=130
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":14664,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":14416,"parent_record_offset":14368,"ancestor_record_offset":14020,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73640,"parent_record_offset":73616,"ancestor_record_offset":14020,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73664,"parent_record_offset":73640,"ancestor_record_offset":14020,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":14988,"parent_record_offset":14880,"ancestor_record_offset":14020,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73692,"parent_record_offset":73664,"ancestor_record_offset":14020,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":14020,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":15232,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":14020,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14476,"parent_record_offset":14452,"ancestor_record_offset":14020,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14452,"parent_record_offset":14416,"ancestor_record_offset":14020,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":64656,"parent_record_offset":64624,"ancestor_record_offset":14020,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":64680,"parent_record_offset":64656,"ancestor_record_offset":14020,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":14020,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=55
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14552,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":14880,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":205,"record_offset":79284,"parent_record_offset":79260,"ancestor_record_offset":14020,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":205,"record_offset":73616,"parent_record_offset":73592,"ancestor_record_offset":14020,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=65
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":15112,"parent_record_offset":14500,"ancestor_record_offset":14020,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":134088,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":134180,"parent_record_offset":134152,"ancestor_record_offset":14020,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":14020,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":49428,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":14020,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":16320,"parent_record_offset":16296,"ancestor_record_offset":14020,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":49288,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":205,"record_offset":105948,"parent_record_offset":105920,"ancestor_record_offset":14020,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":205,"record_offset":133732,"parent_record_offset":133704,"ancestor_record_offset":14020,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133608,"parent_record_offset":133572,"ancestor_record_offset":14020,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133704,"parent_record_offset":133672,"ancestor_record_offset":14020,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133640,"parent_record_offset":133608,"ancestor_record_offset":14020,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133672,"parent_record_offset":133640,"ancestor_record_offset":14020,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133572,"parent_record_offset":133540,"ancestor_record_offset":14020,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":134212,"parent_record_offset":134180,"ancestor_record_offset":14020,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":134252,"parent_record_offset":134212,"ancestor_record_offset":14020,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133476,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133504,"parent_record_offset":133476,"ancestor_record_offset":14020,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":138848,"parent_record_offset":138824,"ancestor_record_offset":14020,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133540,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":14396,"source_line":538,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":132548,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::Repr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85204,"parent_record_offset":85180,"ancestor_record_offset":14020,"inlinee":16267,"source_line":424,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85180,"parent_record_offset":85156,"ancestor_record_offset":14020,"inlinee":15309,"source_line":81,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$23::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85156,"parent_record_offset":85132,"ancestor_record_offset":14020,"inlinee":16265,"source_line":304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":87748,"parent_record_offset":87712,"ancestor_record_offset":14020,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$38::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":87772,"parent_record_offset":87748,"ancestor_record_offset":14020,"inlinee":6700,"source_line":410,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":205,"record_offset":79260,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":205,"record_offset":73592,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":63728,"parent_record_offset":63700,"ancestor_record_offset":14020,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":63700,"parent_record_offset":63672,"ancestor_record_offset":14020,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":205,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":14020,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":35612,"parent_record_offset":35324,"ancestor_record_offset":14020,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":37400,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":64076,"parent_record_offset":64040,"ancestor_record_offset":14020,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":63828,"parent_record_offset":63632,"ancestor_record_offset":14020,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":63632,"parent_record_offset":63604,"ancestor_record_offset":14020,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":63472,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":63604,"parent_record_offset":63472,"ancestor_record_offset":14020,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":64040,"parent_record_offset":64012,"ancestor_record_offset":14020,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":63512,"parent_record_offset":63472,"ancestor_record_offset":14020,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":64332,"parent_record_offset":64040,"ancestor_record_offset":14020,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":205,"record_offset":119312,"parent_record_offset":119284,"ancestor_record_offset":14020,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":205,"record_offset":63672,"parent_record_offset":63632,"ancestor_record_offset":14020,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":205,"record_offset":119284,"parent_record_offset":119220,"ancestor_record_offset":14020,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":160672,"parent_record_offset":159972,"ancestor_record_offset":14020,"inlinee":16285,"source_line":603,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":160640,"parent_record_offset":159972,"ancestor_record_offset":14020,"inlinee":11251,"source_line":585,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::impl$33::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":63984,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":16244,"source_line":739,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":62224,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":16259,"source_line":1194,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":37372,"parent_record_offset":35684,"ancestor_record_offset":14020,"inlinee":16254,"source_line":1185,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":43912,"parent_record_offset":39480,"ancestor_record_offset":14020,"inlinee":16258,"source_line":1190,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::GameLoop::maintain_resolution_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":113280,"parent_record_offset":102140,"ancestor_record_offset":14020,"inlinee":16290,"source_line":899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":14020,"inlinee":16248,"source_line":208,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":19968,"parent_record_offset":19944,"ancestor_record_offset":14020,"inlinee":16249,"source_line":228,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":83288,"parent_record_offset":83264,"ancestor_record_offset":14020,"inlinee":16263,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":85356,"parent_record_offset":85332,"ancestor_record_offset":14020,"inlinee":16270,"source_line":454,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":79228,"parent_record_offset":79204,"ancestor_record_offset":14020,"inlinee":16262,"source_line":531,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":64452,"parent_record_offset":64428,"ancestor_record_offset":14020,"inlinee":16261,"source_line":556,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":113152,"parent_record_offset":113124,"ancestor_record_offset":14020,"inlinee":16287,"source_line":650,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":146420,"parent_record_offset":146392,"ancestor_record_offset":14020,"inlinee":16296,"source_line":877,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":155720,"parent_record_offset":155692,"ancestor_record_offset":14020,"inlinee":16297,"source_line":885,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89400,"parent_record_offset":89368,"ancestor_record_offset":14020,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$25::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89368,"parent_record_offset":89336,"ancestor_record_offset":14020,"inlinee":16278,"source_line":387,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89484,"parent_record_offset":89400,"ancestor_record_offset":14020,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F78
// 首条位置: {"module":205,"inlinee":16248}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F79
// 首条位置: {"module":205,"inlinee":16249}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::process_app_event
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F81
// 首条位置: {"module":205,"inlinee":16257}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":62308,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68228,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68572,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":45780,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":40288,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::keyboard::impl$2::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87576,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":16272,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::keyboard::impl$46::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":85244,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":16269,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":77276,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":49372,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":205,"record_offset":85132,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":85332,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":64428,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":96836,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":88488,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":62364,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":63936,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":49428,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":49288,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":205,"record_offset":79260,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":205,"record_offset":73592,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":63472,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$33::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":63984,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":16244,"source_line":739,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":62224,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":16259,"source_line":1194,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":43912,"parent_record_offset":39480,"ancestor_record_offset":39480,"inlinee":16258,"source_line":1190,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":62728,"parent_record_offset":62704,"ancestor_record_offset":39480,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":62752,"parent_record_offset":62728,"ancestor_record_offset":39480,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":62776,"parent_record_offset":62752,"ancestor_record_offset":39480,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":62704,"parent_record_offset":62648,"ancestor_record_offset":39480,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":39480,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":62676,"parent_record_offset":62648,"ancestor_record_offset":39480,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":62648,"parent_record_offset":62624,"ancestor_record_offset":39480,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":41708,"parent_record_offset":41684,"ancestor_record_offset":39480,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":41684,"parent_record_offset":41660,"ancestor_record_offset":39480,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":62624,"parent_record_offset":62600,"ancestor_record_offset":39480,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":87828,"parent_record_offset":87796,"ancestor_record_offset":39480,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":87796,"parent_record_offset":87772,"ancestor_record_offset":39480,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":88820,"parent_record_offset":88796,"ancestor_record_offset":39480,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":68352,"parent_record_offset":68312,"ancestor_record_offset":39480,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":39480,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":41612,"parent_record_offset":41572,"ancestor_record_offset":39480,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":68284,"parent_record_offset":68252,"ancestor_record_offset":39480,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":68312,"parent_record_offset":68284,"ancestor_record_offset":39480,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":62420,"parent_record_offset":62392,"ancestor_record_offset":39480,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":41572,"parent_record_offset":41548,"ancestor_record_offset":39480,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":62332,"parent_record_offset":62308,"ancestor_record_offset":39480,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":40356,"parent_record_offset":40320,"ancestor_record_offset":39480,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$43::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":205,"record_offset":88456,"parent_record_offset":87576,"ancestor_record_offset":39480,"inlinee":16280,"source_line":385,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68596,"parent_record_offset":68572,"ancestor_record_offset":39480,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":68252,"parent_record_offset":68228,"ancestor_record_offset":39480,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":41932,"parent_record_offset":41908,"ancestor_record_offset":39480,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":41908,"parent_record_offset":41884,"ancestor_record_offset":39480,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":41856,"parent_record_offset":41824,"ancestor_record_offset":39480,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":47288,"parent_record_offset":46780,"ancestor_record_offset":39480,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":41800,"parent_record_offset":41288,"ancestor_record_offset":39480,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":41824,"parent_record_offset":41800,"ancestor_record_offset":39480,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":41320,"parent_record_offset":41288,"ancestor_record_offset":39480,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":47372,"parent_record_offset":47312,"ancestor_record_offset":39480,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":41884,"parent_record_offset":41824,"ancestor_record_offset":39480,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":41368,"parent_record_offset":41344,"ancestor_record_offset":39480,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":41344,"parent_record_offset":41320,"ancestor_record_offset":39480,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":39480,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":42000,"parent_record_offset":41976,"ancestor_record_offset":39480,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":41976,"parent_record_offset":41288,"ancestor_record_offset":39480,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":45812,"parent_record_offset":45780,"ancestor_record_offset":39480,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":46780,"parent_record_offset":45812,"ancestor_record_offset":39480,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":40320,"parent_record_offset":40288,"ancestor_record_offset":39480,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":41288,"parent_record_offset":40320,"ancestor_record_offset":39480,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$12::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/lib.rs
//     evidence={"module":205,"record_offset":88428,"parent_record_offset":87576,"ancestor_record_offset":39480,"inlinee":16276,"source_line":137,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$34::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87712,"parent_record_offset":87652,"ancestor_record_offset":39480,"inlinee":16282,"source_line":749,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$48::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/keyboard.rs
//     evidence={"module":205,"record_offset":87652,"parent_record_offset":87576,"ancestor_record_offset":39480,"inlinee":16274,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":40388,"parent_record_offset":40356,"ancestor_record_offset":39480,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":41036,"parent_record_offset":41012,"ancestor_record_offset":39480,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":41064,"parent_record_offset":41036,"ancestor_record_offset":39480,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":43936,"parent_record_offset":43912,"ancestor_record_offset":39480,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":62248,"parent_record_offset":62224,"ancestor_record_offset":39480,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":85272,"parent_record_offset":85244,"ancestor_record_offset":39480,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":205,"record_offset":89428,"parent_record_offset":89400,"ancestor_record_offset":39480,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":205,"record_offset":89452,"parent_record_offset":89428,"ancestor_record_offset":39480,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":40604,"parent_record_offset":40580,"ancestor_record_offset":39480,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":40688,"parent_record_offset":40664,"ancestor_record_offset":39480,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":62976,"parent_record_offset":62944,"ancestor_record_offset":39480,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":40836,"parent_record_offset":40808,"ancestor_record_offset":39480,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":40808,"parent_record_offset":40780,"ancestor_record_offset":39480,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":41460,"parent_record_offset":41396,"ancestor_record_offset":39480,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":41484,"parent_record_offset":41460,"ancestor_record_offset":39480,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":49396,"parent_record_offset":49372,"ancestor_record_offset":39480,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":41524,"parent_record_offset":41344,"ancestor_record_offset":39480,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":87948,"parent_record_offset":87652,"ancestor_record_offset":39480,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":40628,"parent_record_offset":40604,"ancestor_record_offset":39480,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=123
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":62392,"parent_record_offset":62364,"ancestor_record_offset":39480,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":40952,"parent_record_offset":40928,"ancestor_record_offset":39480,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":87864,"parent_record_offset":87828,"ancestor_record_offset":39480,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":68472,"parent_record_offset":68312,"ancestor_record_offset":39480,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":85380,"parent_record_offset":85356,"ancestor_record_offset":39480,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":41396,"parent_record_offset":41368,"ancestor_record_offset":39480,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":41432,"parent_record_offset":41396,"ancestor_record_offset":39480,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":41752,"parent_record_offset":41572,"ancestor_record_offset":39480,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63856,"parent_record_offset":63828,"ancestor_record_offset":39480,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64264,"parent_record_offset":64076,"ancestor_record_offset":39480,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":42024,"parent_record_offset":42000,"ancestor_record_offset":39480,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63544,"parent_record_offset":63512,"ancestor_record_offset":39480,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64360,"parent_record_offset":64332,"ancestor_record_offset":39480,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":49312,"parent_record_offset":49288,"ancestor_record_offset":39480,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":87896,"parent_record_offset":87828,"ancestor_record_offset":39480,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":63568,"parent_record_offset":63544,"ancestor_record_offset":39480,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":39480,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64292,"parent_record_offset":64264,"ancestor_record_offset":39480,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":88916,"parent_record_offset":88856,"ancestor_record_offset":39480,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":64384,"parent_record_offset":64360,"ancestor_record_offset":39480,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":88948,"parent_record_offset":88820,"ancestor_record_offset":39480,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":41012,"parent_record_offset":40988,"ancestor_record_offset":39480,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":40748,"parent_record_offset":40720,"ancestor_record_offset":39480,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":40780,"parent_record_offset":40748,"ancestor_record_offset":39480,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":40720,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":40580,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":41140,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=50
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":40664,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":40424,"parent_record_offset":40388,"ancestor_record_offset":39480,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73640,"parent_record_offset":73616,"ancestor_record_offset":39480,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73664,"parent_record_offset":73640,"ancestor_record_offset":39480,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":40988,"parent_record_offset":40880,"ancestor_record_offset":39480,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":205,"record_offset":73692,"parent_record_offset":73664,"ancestor_record_offset":39480,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40928,"parent_record_offset":40904,"ancestor_record_offset":39480,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":41232,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40904,"parent_record_offset":40880,"ancestor_record_offset":39480,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40476,"parent_record_offset":40452,"ancestor_record_offset":39480,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40452,"parent_record_offset":40424,"ancestor_record_offset":39480,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":64656,"parent_record_offset":64624,"ancestor_record_offset":39480,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":64680,"parent_record_offset":64656,"ancestor_record_offset":39480,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40500,"parent_record_offset":40476,"ancestor_record_offset":39480,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40552,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":40880,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":205,"record_offset":79284,"parent_record_offset":79260,"ancestor_record_offset":39480,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":205,"record_offset":73616,"parent_record_offset":73592,"ancestor_record_offset":39480,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":41112,"parent_record_offset":40500,"ancestor_record_offset":39480,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":49476,"parent_record_offset":49452,"ancestor_record_offset":39480,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":49452,"parent_record_offset":49428,"ancestor_record_offset":39480,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":49500,"parent_record_offset":49476,"ancestor_record_offset":39480,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::Repr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85204,"parent_record_offset":85180,"ancestor_record_offset":39480,"inlinee":16267,"source_line":424,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85180,"parent_record_offset":85156,"ancestor_record_offset":39480,"inlinee":15309,"source_line":81,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$23::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":85156,"parent_record_offset":85132,"ancestor_record_offset":39480,"inlinee":16265,"source_line":304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":87748,"parent_record_offset":87712,"ancestor_record_offset":39480,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$38::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":205,"record_offset":87772,"parent_record_offset":87748,"ancestor_record_offset":39480,"inlinee":6700,"source_line":410,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":63728,"parent_record_offset":63700,"ancestor_record_offset":39480,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":63700,"parent_record_offset":63672,"ancestor_record_offset":39480,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":64076,"parent_record_offset":64040,"ancestor_record_offset":39480,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":63828,"parent_record_offset":63632,"ancestor_record_offset":39480,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":63632,"parent_record_offset":63604,"ancestor_record_offset":39480,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":63604,"parent_record_offset":63472,"ancestor_record_offset":39480,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":64040,"parent_record_offset":64012,"ancestor_record_offset":39480,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":63512,"parent_record_offset":63472,"ancestor_record_offset":39480,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":64332,"parent_record_offset":64040,"ancestor_record_offset":39480,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":205,"record_offset":63672,"parent_record_offset":63632,"ancestor_record_offset":39480,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":83288,"parent_record_offset":83264,"ancestor_record_offset":39480,"inlinee":16263,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":85356,"parent_record_offset":85332,"ancestor_record_offset":39480,"inlinee":16270,"source_line":454,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":79228,"parent_record_offset":79204,"ancestor_record_offset":39480,"inlinee":16262,"source_line":531,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":64452,"parent_record_offset":64428,"ancestor_record_offset":39480,"inlinee":16261,"source_line":556,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89400,"parent_record_offset":89368,"ancestor_record_offset":39480,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$25::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89368,"parent_record_offset":89336,"ancestor_record_offset":39480,"inlinee":16278,"source_line":387,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":205,"record_offset":89484,"parent_record_offset":89400,"ancestor_record_offset":39480,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 434
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F87
// 首条位置: {"module":205,"inlinee":16263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 454
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F8E
// 首条位置: {"module":205,"inlinee":16270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":85380,"parent_record_offset":85356,"ancestor_record_offset":85356,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":85608,"parent_record_offset":85584,"ancestor_record_offset":85356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":85632,"parent_record_offset":85608,"ancestor_record_offset":85356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":85656,"parent_record_offset":85632,"ancestor_record_offset":85356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":85584,"parent_record_offset":85528,"ancestor_record_offset":85356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":85556,"parent_record_offset":85528,"ancestor_record_offset":85356,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":85528,"parent_record_offset":85504,"ancestor_record_offset":85356,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":85504,"parent_record_offset":85480,"ancestor_record_offset":85356,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":85408,"parent_record_offset":85380,"ancestor_record_offset":85356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 531
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F86
// 首条位置: {"module":205,"inlinee":16262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 556
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F85
// 首条位置: {"module":205,"inlinee":16261}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 591
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F9B
// 首条位置: {"module":205,"inlinee":16283}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":126440,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":134360,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":102732,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":124656,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113184,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":16288,"source_line":14,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":205,"record_offset":113124,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":126504,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":124528,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":205,"record_offset":155752,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":152100,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":111532,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":146392,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":159008,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":134152,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] egui::context::Context::pixels_per_point
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":205,"record_offset":155904,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":9292,"source_line":1595,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] emath::rect::Rect::from_min_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/rect.rs
//     evidence={"module":205,"record_offset":155968,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":6052,"source_line":78,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":134088,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133476,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133540,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":14396,"source_line":538,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":132548,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":138920,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::maintain_resolution_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":113280,"parent_record_offset":102140,"ancestor_record_offset":102140,"inlinee":16290,"source_line":899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":104228,"parent_record_offset":104200,"ancestor_record_offset":102140,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":104284,"parent_record_offset":104256,"ancestor_record_offset":102140,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":104256,"parent_record_offset":104228,"ancestor_record_offset":102140,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":104200,"parent_record_offset":104172,"ancestor_record_offset":102140,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":104172,"parent_record_offset":104128,"ancestor_record_offset":102140,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":104128,"parent_record_offset":104100,"ancestor_record_offset":102140,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":119184,"parent_record_offset":119156,"ancestor_record_offset":102140,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":119156,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":102792,"parent_record_offset":102760,"ancestor_record_offset":102140,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":104532,"parent_record_offset":104504,"ancestor_record_offset":102140,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":104504,"parent_record_offset":104476,"ancestor_record_offset":102140,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":104444,"parent_record_offset":104412,"ancestor_record_offset":102140,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":104384,"parent_record_offset":103816,"ancestor_record_offset":102140,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":118660,"parent_record_offset":118088,"ancestor_record_offset":102140,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":104412,"parent_record_offset":104384,"ancestor_record_offset":102140,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":103848,"parent_record_offset":103816,"ancestor_record_offset":102140,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":104476,"parent_record_offset":104412,"ancestor_record_offset":102140,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":118752,"parent_record_offset":118688,"ancestor_record_offset":102140,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":103904,"parent_record_offset":103876,"ancestor_record_offset":102140,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":103876,"parent_record_offset":103848,"ancestor_record_offset":102140,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":104100,"parent_record_offset":104072,"ancestor_record_offset":102140,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":113496,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":117040,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":118884,"parent_record_offset":118856,"ancestor_record_offset":102140,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":135448,"parent_record_offset":135420,"ancestor_record_offset":102140,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":118856,"parent_record_offset":118088,"ancestor_record_offset":102140,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":102760,"parent_record_offset":102732,"ancestor_record_offset":102140,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":103816,"parent_record_offset":102760,"ancestor_record_offset":102140,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":117068,"parent_record_offset":117040,"ancestor_record_offset":102140,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":118088,"parent_record_offset":117068,"ancestor_record_offset":102140,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":102820,"parent_record_offset":102792,"ancestor_record_offset":102140,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":103540,"parent_record_offset":103512,"ancestor_record_offset":102140,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":103572,"parent_record_offset":103540,"ancestor_record_offset":102140,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113240,"parent_record_offset":113212,"ancestor_record_offset":102140,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":205,"record_offset":113212,"parent_record_offset":113184,"ancestor_record_offset":102140,"inlinee":16289,"source_line":159,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":205,"record_offset":134116,"parent_record_offset":134088,"ancestor_record_offset":102140,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119848,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119220,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119092,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119876,"parent_record_offset":119848,"ancestor_record_offset":102140,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":103056,"parent_record_offset":103028,"ancestor_record_offset":102140,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=54
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":103152,"parent_record_offset":103124,"ancestor_record_offset":102140,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":205,"record_offset":120376,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":119740,"parent_record_offset":119680,"ancestor_record_offset":102140,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":152128,"parent_record_offset":152100,"ancestor_record_offset":102140,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119776,"parent_record_offset":119636,"ancestor_record_offset":102140,"inlinee":5654,"source_line":761,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119680,"parent_record_offset":119636,"ancestor_record_offset":102140,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119708,"parent_record_offset":119680,"ancestor_record_offset":102140,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":103320,"parent_record_offset":103288,"ancestor_record_offset":102140,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":103288,"parent_record_offset":103256,"ancestor_record_offset":102140,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":104004,"parent_record_offset":103936,"ancestor_record_offset":102140,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":135420,"parent_record_offset":134360,"ancestor_record_offset":102140,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":104028,"parent_record_offset":104004,"ancestor_record_offset":102140,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":119812,"parent_record_offset":119604,"ancestor_record_offset":102140,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":119120,"parent_record_offset":119092,"ancestor_record_offset":102140,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":104072,"parent_record_offset":103876,"ancestor_record_offset":102140,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":122532,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":126536,"parent_record_offset":126504,"ancestor_record_offset":102140,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":103084,"parent_record_offset":103056,"ancestor_record_offset":102140,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":138824,"parent_record_offset":138800,"ancestor_record_offset":102140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":103448,"parent_record_offset":103420,"ancestor_record_offset":102140,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":133000,"parent_record_offset":132972,"ancestor_record_offset":102140,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":132972,"parent_record_offset":132944,"ancestor_record_offset":102140,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":133208,"parent_record_offset":133176,"ancestor_record_offset":102140,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":205,"record_offset":133312,"parent_record_offset":133284,"ancestor_record_offset":102140,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":133284,"parent_record_offset":133256,"ancestor_record_offset":102140,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":103936,"parent_record_offset":103904,"ancestor_record_offset":102140,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":103972,"parent_record_offset":103936,"ancestor_record_offset":102140,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":104332,"parent_record_offset":104128,"ancestor_record_offset":102140,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132916,"parent_record_offset":132888,"ancestor_record_offset":102140,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132944,"parent_record_offset":132916,"ancestor_record_offset":102140,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":133340,"parent_record_offset":133312,"ancestor_record_offset":102140,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":132888,"parent_record_offset":132764,"ancestor_record_offset":102140,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":205,"record_offset":132584,"parent_record_offset":132548,"ancestor_record_offset":102140,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":205,"record_offset":132732,"parent_record_offset":132700,"ancestor_record_offset":102140,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":205,"record_offset":132700,"parent_record_offset":132668,"ancestor_record_offset":102140,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132640,"parent_record_offset":132612,"ancestor_record_offset":102140,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132668,"parent_record_offset":132640,"ancestor_record_offset":102140,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":132612,"parent_record_offset":132584,"ancestor_record_offset":102140,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":205,"record_offset":133256,"parent_record_offset":132668,"ancestor_record_offset":102140,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":205,"record_offset":133432,"parent_record_offset":133400,"ancestor_record_offset":102140,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":205,"record_offset":133400,"parent_record_offset":132548,"ancestor_record_offset":102140,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":132764,"parent_record_offset":132732,"ancestor_record_offset":102140,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":133080,"parent_record_offset":132764,"ancestor_record_offset":102140,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":133048,"parent_record_offset":132764,"ancestor_record_offset":102140,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":205,"record_offset":132824,"parent_record_offset":132764,"ancestor_record_offset":102140,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":133760,"parent_record_offset":133732,"ancestor_record_offset":102140,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":118912,"parent_record_offset":118884,"ancestor_record_offset":102140,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":134284,"parent_record_offset":134212,"ancestor_record_offset":102140,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":119024,"parent_record_offset":118996,"ancestor_record_offset":102140,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":134312,"parent_record_offset":134284,"ancestor_record_offset":102140,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":118940,"parent_record_offset":118912,"ancestor_record_offset":102140,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119636,"parent_record_offset":119604,"ancestor_record_offset":102140,"inlinee":16293,"source_line":782,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119336,"parent_record_offset":119312,"ancestor_record_offset":102140,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119604,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":16292,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":205,"record_offset":115360,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":13678,"source_line":5,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":205,"record_offset":155932,"parent_record_offset":155904,"ancestor_record_offset":102140,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] emath::pos2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/pos2.rs
//     evidence={"module":205,"record_offset":155996,"parent_record_offset":155968,"ancestor_record_offset":102140,"inlinee":6034,"source_line":251,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":103512,"parent_record_offset":103488,"ancestor_record_offset":102140,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":103220,"parent_record_offset":103188,"ancestor_record_offset":102140,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":103256,"parent_record_offset":103220,"ancestor_record_offset":102140,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":103188,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":103028,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":103656,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=54
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":103124,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":102856,"parent_record_offset":102820,"ancestor_record_offset":102140,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":103488,"parent_record_offset":103368,"ancestor_record_offset":102140,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":103420,"parent_record_offset":103392,"ancestor_record_offset":102140,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":103760,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":103392,"parent_record_offset":103368,"ancestor_record_offset":102140,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":102912,"parent_record_offset":102884,"ancestor_record_offset":102140,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":102884,"parent_record_offset":102856,"ancestor_record_offset":102140,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":102940,"parent_record_offset":102912,"ancestor_record_offset":102140,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":102996,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":103368,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":103624,"parent_record_offset":102940,"ancestor_record_offset":102140,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":134180,"parent_record_offset":134152,"ancestor_record_offset":102140,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":118996,"parent_record_offset":113280,"ancestor_record_offset":102140,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":205,"record_offset":105948,"parent_record_offset":105920,"ancestor_record_offset":102140,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":205,"record_offset":133732,"parent_record_offset":133704,"ancestor_record_offset":102140,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133608,"parent_record_offset":133572,"ancestor_record_offset":102140,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133704,"parent_record_offset":133672,"ancestor_record_offset":102140,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133640,"parent_record_offset":133608,"ancestor_record_offset":102140,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133672,"parent_record_offset":133640,"ancestor_record_offset":102140,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":205,"record_offset":133572,"parent_record_offset":133540,"ancestor_record_offset":102140,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":134212,"parent_record_offset":134180,"ancestor_record_offset":102140,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":134252,"parent_record_offset":134212,"ancestor_record_offset":102140,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":133504,"parent_record_offset":133476,"ancestor_record_offset":102140,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":205,"record_offset":138848,"parent_record_offset":138824,"ancestor_record_offset":102140,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":205,"record_offset":119312,"parent_record_offset":119284,"ancestor_record_offset":102140,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":205,"record_offset":119284,"parent_record_offset":119220,"ancestor_record_offset":102140,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":113152,"parent_record_offset":113124,"ancestor_record_offset":102140,"inlinee":16287,"source_line":650,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":146420,"parent_record_offset":146392,"ancestor_record_offset":102140,"inlinee":16296,"source_line":877,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":205,"record_offset":155720,"parent_record_offset":155692,"ancestor_record_offset":102140,"inlinee":16297,"source_line":885,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 650
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F9F
// 首条位置: {"module":205,"inlinee":16287}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 877
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3FA8
// 首条位置: {"module":205,"inlinee":16296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 885
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3FA9
// 首条位置: {"module":205,"inlinee":16297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::maintain_resolution_scale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 899
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3FA2
// 首条位置: {"module":205,"inlinee":16290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":119156,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":113496,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":117040,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119848,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119220,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119092,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":205,"record_offset":120376,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":122532,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119604,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":16292,"source_line":1220,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":205,"record_offset":115360,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":13678,"source_line":5,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":118996,"parent_record_offset":113280,"ancestor_record_offset":113280,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":114996,"parent_record_offset":114968,"ancestor_record_offset":113280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":115052,"parent_record_offset":115024,"ancestor_record_offset":113280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":115024,"parent_record_offset":114996,"ancestor_record_offset":113280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":114968,"parent_record_offset":114940,"ancestor_record_offset":113280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":114940,"parent_record_offset":114896,"ancestor_record_offset":113280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":114896,"parent_record_offset":114868,"ancestor_record_offset":113280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":119184,"parent_record_offset":119156,"ancestor_record_offset":113280,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":113556,"parent_record_offset":113524,"ancestor_record_offset":113280,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":115300,"parent_record_offset":115272,"ancestor_record_offset":113280,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":115272,"parent_record_offset":115244,"ancestor_record_offset":113280,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":115212,"parent_record_offset":115180,"ancestor_record_offset":113280,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":115152,"parent_record_offset":114584,"ancestor_record_offset":113280,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":118660,"parent_record_offset":118088,"ancestor_record_offset":113280,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":115180,"parent_record_offset":115152,"ancestor_record_offset":113280,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":114616,"parent_record_offset":114584,"ancestor_record_offset":113280,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":115244,"parent_record_offset":115180,"ancestor_record_offset":113280,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":118752,"parent_record_offset":118688,"ancestor_record_offset":113280,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":114672,"parent_record_offset":114644,"ancestor_record_offset":113280,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":114644,"parent_record_offset":114616,"ancestor_record_offset":113280,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":114868,"parent_record_offset":114840,"ancestor_record_offset":113280,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":118884,"parent_record_offset":118856,"ancestor_record_offset":113280,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":118856,"parent_record_offset":118088,"ancestor_record_offset":113280,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":113524,"parent_record_offset":113496,"ancestor_record_offset":113280,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":114584,"parent_record_offset":113524,"ancestor_record_offset":113280,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":117068,"parent_record_offset":117040,"ancestor_record_offset":113280,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":118088,"parent_record_offset":117068,"ancestor_record_offset":113280,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":113584,"parent_record_offset":113556,"ancestor_record_offset":113280,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":114308,"parent_record_offset":114280,"ancestor_record_offset":113280,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":114340,"parent_record_offset":114308,"ancestor_record_offset":113280,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":119876,"parent_record_offset":119848,"ancestor_record_offset":113280,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":113824,"parent_record_offset":113796,"ancestor_record_offset":113280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":113920,"parent_record_offset":113892,"ancestor_record_offset":113280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":119740,"parent_record_offset":119680,"ancestor_record_offset":113280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119776,"parent_record_offset":119636,"ancestor_record_offset":113280,"inlinee":5654,"source_line":761,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119680,"parent_record_offset":119636,"ancestor_record_offset":113280,"inlinee":6867,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":119708,"parent_record_offset":119680,"ancestor_record_offset":113280,"inlinee":6868,"source_line":3050,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":114088,"parent_record_offset":114056,"ancestor_record_offset":113280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":114056,"parent_record_offset":114024,"ancestor_record_offset":113280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":114772,"parent_record_offset":114704,"ancestor_record_offset":113280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":114796,"parent_record_offset":114772,"ancestor_record_offset":113280,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":119812,"parent_record_offset":119604,"ancestor_record_offset":113280,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":119120,"parent_record_offset":119092,"ancestor_record_offset":113280,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":114840,"parent_record_offset":114644,"ancestor_record_offset":113280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":113852,"parent_record_offset":113824,"ancestor_record_offset":113280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":114216,"parent_record_offset":114188,"ancestor_record_offset":113280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":114704,"parent_record_offset":114672,"ancestor_record_offset":113280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":114740,"parent_record_offset":114704,"ancestor_record_offset":113280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":115100,"parent_record_offset":114896,"ancestor_record_offset":113280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":118912,"parent_record_offset":118884,"ancestor_record_offset":113280,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":119024,"parent_record_offset":118996,"ancestor_record_offset":113280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":118940,"parent_record_offset":118912,"ancestor_record_offset":113280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119636,"parent_record_offset":119604,"ancestor_record_offset":113280,"inlinee":16293,"source_line":782,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":205,"record_offset":119336,"parent_record_offset":119312,"ancestor_record_offset":113280,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":114280,"parent_record_offset":114256,"ancestor_record_offset":113280,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":113988,"parent_record_offset":113956,"ancestor_record_offset":113280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":114024,"parent_record_offset":113988,"ancestor_record_offset":113280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":113956,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":113796,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":114424,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":113892,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":113620,"parent_record_offset":113584,"ancestor_record_offset":113280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":114256,"parent_record_offset":114136,"ancestor_record_offset":113280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":114188,"parent_record_offset":114160,"ancestor_record_offset":113280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":114528,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":114160,"parent_record_offset":114136,"ancestor_record_offset":113280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":113680,"parent_record_offset":113652,"ancestor_record_offset":113280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":113652,"parent_record_offset":113620,"ancestor_record_offset":113280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":113708,"parent_record_offset":113680,"ancestor_record_offset":113280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":113764,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":114136,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":114392,"parent_record_offset":113708,"ancestor_record_offset":113280,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":205,"record_offset":116744,"parent_record_offset":116716,"ancestor_record_offset":113280,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::time::windows::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\time\windows.rs
//     evidence={"module":205,"record_offset":119312,"parent_record_offset":119284,"ancestor_record_offset":113280,"inlinee":6431,"source_line":13,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$24::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\time.rs
//     evidence={"module":205,"record_offset":119284,"parent_record_offset":119220,"ancestor_record_offset":113280,"inlinee":6429,"source_line":154,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1080
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F9C
// 首条位置: {"module":205,"inlinee":16284}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":161044,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":160576,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":160120,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":160076,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160736,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":11519,"source_line":54,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":205,"record_offset":160956,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":160672,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":16285,"source_line":603,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":160640,"parent_record_offset":159972,"ancestor_record_offset":159972,"inlinee":11251,"source_line":585,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":161072,"parent_record_offset":161044,"ancestor_record_offset":159972,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":205,"record_offset":160604,"parent_record_offset":160576,"ancestor_record_offset":159972,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":160836,"parent_record_offset":160736,"ancestor_record_offset":159972,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":161012,"parent_record_offset":160988,"ancestor_record_offset":159972,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160400,"parent_record_offset":160192,"ancestor_record_offset":159972,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160500,"parent_record_offset":160468,"ancestor_record_offset":159972,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160324,"parent_record_offset":160292,"ancestor_record_offset":159972,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160352,"parent_record_offset":160324,"ancestor_record_offset":159972,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160428,"parent_record_offset":160400,"ancestor_record_offset":159972,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":160528,"parent_record_offset":160500,"ancestor_record_offset":159972,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160804,"parent_record_offset":160776,"ancestor_record_offset":159972,"inlinee":11523,"source_line":10,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160776,"parent_record_offset":160736,"ancestor_record_offset":159972,"inlinee":11521,"source_line":19,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":205,"record_offset":160872,"parent_record_offset":160836,"ancestor_record_offset":159972,"inlinee":11479,"source_line":36,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":160292,"parent_record_offset":160260,"ancestor_record_offset":159972,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":160260,"parent_record_offset":160228,"ancestor_record_offset":159972,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":160192,"parent_record_offset":160152,"ancestor_record_offset":159972,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":160152,"parent_record_offset":160120,"ancestor_record_offset":159972,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":160468,"parent_record_offset":160152,"ancestor_record_offset":159972,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":205,"record_offset":160228,"parent_record_offset":160192,"ancestor_record_offset":159972,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::on_cursor_moved_absolute
// 记录形式: compiled_procedure; instances=1
// 源码行: 1114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":248,"rva":5737936,"va":5374447056}
// 编译实例 RVAs: 0x578dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41903321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447056,"raw_hex":"e8d9647f02","size":5,"soft_fail":false,"target_va":5416350944,"va":5374447618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42173916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447056,"raw_hex":"e8dc858302","size":5,"soft_fail":false,"target_va":5416621600,"va":5374447679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::egui_input::EguiInputResource::on_cursor_moved
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5539405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447056,"raw_hex":"e84d865400","size":5,"soft_fail":false,"target_va":5379986752,"va":5374447342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":2176,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":4072,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":392,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9394,"source_line":2047,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":4368,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":4204,"parent_record_offset":4164,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":248,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":1688,"parent_record_offset":1648,"ancestor_record_offset":248,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":4132,"parent_record_offset":4096,"ancestor_record_offset":248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":4164,"parent_record_offset":4132,"ancestor_record_offset":248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":248,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":205,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":248,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":444,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":205,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":248,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":248,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":248,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":1932,"parent_record_offset":1900,"ancestor_record_offset":248,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":1876,"parent_record_offset":1364,"ancestor_record_offset":248,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":248,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":1396,"parent_record_offset":1364,"ancestor_record_offset":248,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":1960,"parent_record_offset":1900,"ancestor_record_offset":248,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":248,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":248,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":248,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":248,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":2052,"parent_record_offset":1364,"ancestor_record_offset":248,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":416,"parent_record_offset":392,"ancestor_record_offset":248,"inlinee":9395,"source_line":515,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":1364,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":248,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":248,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":248,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":248,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":916,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":888,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":1536,"parent_record_offset":1472,"ancestor_record_offset":248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":248,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":1600,"parent_record_offset":1420,"ancestor_record_offset":248,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":248,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":4324,"parent_record_offset":4164,"ancestor_record_offset":248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":248,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":1508,"parent_record_offset":1472,"ancestor_record_offset":248,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":1828,"parent_record_offset":1648,"ancestor_record_offset":248,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":248,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":248,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":248,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":248,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":860,"parent_record_offset":828,"ancestor_record_offset":248,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":800,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":660,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":1216,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":744,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":504,"parent_record_offset":468,"ancestor_record_offset":248,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":1064,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":248,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":1308,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":980,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":248,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":248,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":632,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":960,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":1188,"parent_record_offset":580,"ancestor_record_offset":248,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::shut_down_everything
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F76
// 首条位置: {"module":205,"inlinee":16246}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":7832,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":9043,"source_line":2029,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":205,"record_offset":11184,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":9507,"source_line":1807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":13324,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":13708,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":11156,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":13764,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":13380,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13240,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::with_global_allocator
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":205,"record_offset":12800,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":8375,"source_line":877,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9504,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":8444,"source_line":2245,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":205,"record_offset":10524,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::mark_shutting_down
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":205,"record_offset":7748,"parent_record_offset":7664,"ancestor_record_offset":7664,"inlinee":15327,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":7664,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":7664,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":7664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":7664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":7664,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":7664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":7664,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":205,"record_offset":10132,"parent_record_offset":10076,"ancestor_record_offset":7664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9176,"parent_record_offset":9152,"ancestor_record_offset":7664,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9668,"parent_record_offset":9640,"ancestor_record_offset":7664,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":7664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":7664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9224,"parent_record_offset":9200,"ancestor_record_offset":7664,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":7664,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":7664,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":7664,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":205,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":7664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":12864,"parent_record_offset":12828,"ancestor_record_offset":7664,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":12828,"parent_record_offset":12800,"ancestor_record_offset":7664,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":205,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":7664,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":7664,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12432,"parent_record_offset":12392,"ancestor_record_offset":7664,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":7664,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9128,"parent_record_offset":9088,"ancestor_record_offset":7664,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9612,"parent_record_offset":9552,"ancestor_record_offset":7664,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9640,"parent_record_offset":9612,"ancestor_record_offset":7664,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":7664,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":7664,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":205,"record_offset":7884,"parent_record_offset":7856,"ancestor_record_offset":7664,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":7664,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9424,"parent_record_offset":9400,"ancestor_record_offset":7664,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":205,"record_offset":9372,"parent_record_offset":9340,"ancestor_record_offset":7664,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9316,"parent_record_offset":8808,"ancestor_record_offset":7664,"inlinee":9047,"source_line":98,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":7664,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":12620,"parent_record_offset":11184,"ancestor_record_offset":7664,"inlinee":9512,"source_line":249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":8836,"parent_record_offset":8808,"ancestor_record_offset":7664,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":12140,"parent_record_offset":11184,"ancestor_record_offset":7664,"inlinee":9508,"source_line":344,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":205,"record_offset":9400,"parent_record_offset":9340,"ancestor_record_offset":7664,"inlinee":9049,"source_line":99,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":7664,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":7664,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":7664,"inlinee":9509,"source_line":415,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":7664,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":205,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":7664,"inlinee":9511,"source_line":417,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":7664,"inlinee":9044,"source_line":403,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":205,"record_offset":8808,"parent_record_offset":7856,"ancestor_record_offset":7664,"inlinee":9046,"source_line":457,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":205,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7664,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":205,"record_offset":12712,"parent_record_offset":11184,"ancestor_record_offset":7664,"inlinee":9513,"source_line":991,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":205,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":7664,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":205,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":7664,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":205,"record_offset":10556,"parent_record_offset":10524,"ancestor_record_offset":7664,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7664,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":205,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":7664,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":7664,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":205,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":7664,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":205,"record_offset":8976,"parent_record_offset":8912,"ancestor_record_offset":7664,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":7664,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":7664,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":9040,"parent_record_offset":8860,"ancestor_record_offset":7664,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":7664,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":10004,"parent_record_offset":9552,"ancestor_record_offset":7664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":7664,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":7664,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":7664,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":7664,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":7664,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":8948,"parent_record_offset":8912,"ancestor_record_offset":7664,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":9268,"parent_record_offset":9088,"ancestor_record_offset":7664,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":12572,"parent_record_offset":12392,"ancestor_record_offset":7664,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":7664,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13168,"parent_record_offset":13104,"ancestor_record_offset":7664,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":12956,"parent_record_offset":12920,"ancestor_record_offset":7664,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":7664,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":7664,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":7664,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":7664,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":7664,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13636,"parent_record_offset":13576,"ancestor_record_offset":7664,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":13668,"parent_record_offset":13540,"ancestor_record_offset":7664,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":205,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":7664,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":7664,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8300,"parent_record_offset":8268,"ancestor_record_offset":7664,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":205,"record_offset":8240,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8100,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8660,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":205,"record_offset":8184,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":7944,"parent_record_offset":7908,"ancestor_record_offset":7664,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":205,"record_offset":8508,"parent_record_offset":8400,"ancestor_record_offset":7664,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":7664,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8752,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":7664,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":7664,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":7664,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7664,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8072,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":205,"record_offset":8400,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":205,"record_offset":8632,"parent_record_offset":8020,"ancestor_record_offset":7664,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":205,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":7664,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":7664,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":205,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":7664,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":205,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":7664,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":12920,"parent_record_offset":12828,"ancestor_record_offset":7664,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":205,"record_offset":13104,"parent_record_offset":13076,"ancestor_record_offset":7664,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":13012,"parent_record_offset":12920,"ancestor_record_offset":7664,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":205,"record_offset":13140,"parent_record_offset":13104,"ancestor_record_offset":7664,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":205,"record_offset":9972,"parent_record_offset":9888,"ancestor_record_offset":7664,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9916,"parent_record_offset":9888,"ancestor_record_offset":7664,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":7664,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9552,"parent_record_offset":9504,"ancestor_record_offset":7664,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":205,"record_offset":9888,"parent_record_offset":9552,"ancestor_record_offset":7664,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F7E
// 首条位置: {"module":205,"inlinee":16254}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F82
// 首条位置: {"module":205,"inlinee":16258}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":43936,"parent_record_offset":43912,"ancestor_record_offset":43912,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F83
// 首条位置: {"module":205,"inlinee":16259}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":205,"record_offset":62248,"parent_record_offset":62224,"ancestor_record_offset":62224,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":62272,"parent_record_offset":62248,"ancestor_record_offset":62224,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$4::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 1235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":166184,"rva":5782704,"va":5374491824}
// 编译实例 RVAs: 0x583cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x1401cfd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-3882920","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374491824,"raw_hex":"e958c0c4ff","size":5,"soft_fail":false,"target_va":5370608912,"va":5374491827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":205,"record_offset":166304,"parent_record_offset":166184,"ancestor_record_offset":166184,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::gather_pending_video_setting_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 1240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":205,"record_offset":4400,"rva":5738576,"va":5374447696}
// 编译实例 RVAs: 0x579050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42168031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447696,"raw_hex":"e8df6e8302","size":5,"soft_fail":false,"target_va":5416616112,"va":5374448076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42168393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447696,"raw_hex":"e849708302","size":5,"soft_fail":false,"target_va":5416616304,"va":5374447906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5183122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447696,"raw_hex":"e86ee9b0ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5374448237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42173455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447696,"raw_hex":"e80f848302","size":5,"soft_fail":false,"target_va":5416621632,"va":5374448172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42171645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374447696,"raw_hex":"e8fd7c8302","size":5,"soft_fail":false,"target_va":5416619696,"va":5374448046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":205,"record_offset":5260,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":205,"record_offset":4564,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":205,"record_offset":5584,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":205,"record_offset":6448,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":5984,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$33::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":205,"record_offset":6492,"parent_record_offset":4400,"ancestor_record_offset":4400,"inlinee":16244,"source_line":739,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4400,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":205,"record_offset":4880,"parent_record_offset":4652,"ancestor_record_offset":4400,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":205,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":4400,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":205,"record_offset":5292,"parent_record_offset":5260,"ancestor_record_offset":4400,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":205,"record_offset":4628,"parent_record_offset":4588,"ancestor_record_offset":4400,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":205,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":4400,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":205,"record_offset":5360,"parent_record_offset":5328,"ancestor_record_offset":4400,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":205,"record_offset":5328,"parent_record_offset":5292,"ancestor_record_offset":4400,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":205,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":4400,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":4836,"parent_record_offset":4808,"ancestor_record_offset":4400,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":205,"record_offset":4696,"parent_record_offset":4652,"ancestor_record_offset":4400,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":5392,"parent_record_offset":5360,"ancestor_record_offset":4400,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":205,"record_offset":5432,"parent_record_offset":5328,"ancestor_record_offset":4400,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":205,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":4400,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":205,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":4400,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":4752,"parent_record_offset":4652,"ancestor_record_offset":4400,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":205,"record_offset":5104,"parent_record_offset":4652,"ancestor_record_offset":4400,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":4400,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":205,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":4400,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":205,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":4400,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":205,"record_offset":5508,"parent_record_offset":5456,"ancestor_record_offset":4400,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":4400,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5916,"parent_record_offset":5744,"ancestor_record_offset":4400,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":6052,"parent_record_offset":6020,"ancestor_record_offset":4400,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5684,"parent_record_offset":5652,"ancestor_record_offset":4400,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5848,"parent_record_offset":5820,"ancestor_record_offset":4400,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":4400,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":4400,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":4400,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":205,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":4400,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":4400,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":205,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":4400,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":5744,"parent_record_offset":5616,"ancestor_record_offset":4400,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":6340,"parent_record_offset":6140,"ancestor_record_offset":4400,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":205,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":4400,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":6112,"parent_record_offset":5984,"ancestor_record_offset":4400,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":205,"record_offset":5616,"parent_record_offset":5584,"ancestor_record_offset":4400,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":6020,"parent_record_offset":5984,"ancestor_record_offset":4400,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":205,"record_offset":5652,"parent_record_offset":5616,"ancestor_record_offset":4400,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":205,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":4400,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00205.jsonl.gz"}
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
// 完整结构及原始类型记录: game_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::CursorRequestClient
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoop
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::game_loop::GameLoop>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::game_loop::GameLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoopPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::tgsr_capture::TgsrCapturePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::tgsr_capture::TgsrCapturePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 信息 ID: 73dd96008cd37db38f6d25a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=62224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 信息 ID: f42b90501825dbeceb4b3ec9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=37372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 信息 ID: 2ac7729bc5f62ade839ad895
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=43912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 信息 ID: c5a69223ea9e393025d8fc66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=159972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::main_loop
// 信息 ID: ebfbb1d3481eda4b0ccc9cbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x57969f -> 0x579c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::maintain_resolution_scale
// 信息 ID: 36928fa781422a753aee1941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=113280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::new
// 信息 ID: 4e6a9bc528d87bfb98bbc3a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x424d9c -> 0x579290 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x426008 -> 0x579290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::process_app_event
// 信息 ID: bfa5d0aafa25a51d63fea973
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=39480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::receive_app_events
// 信息 ID: f1ad057de5734ae5c9518874
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=35324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::run
// 信息 ID: 53f10928290bd621d5ab9a78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x424dcc -> 0x579680 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x42602a -> 0x579680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::send_window_events
// 信息 ID: 817c10625e049f573eed50cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=35684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 信息 ID: 848262d02e6a1fa66bb52cda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=102140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::gather_pending_video_setting_changes
// 信息 ID: a9471d8d8747faf1eaacb621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3b4dcd21e00537f8abf03bee: void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> > @ 0x2b246a -> 0x579050 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> @ 0x4cd1d2 -> 0x579050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 信息 ID: 19fc1ce39b2852e87e3d5ad1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=16008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 信息 ID: d2923d476263bfd7499f5b82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=19968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 信息 ID: f8244ce6be603c5823f7b86e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=83288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$1
// 信息 ID: 5fdf57397ed4fce9003d0711
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=85356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$2
// 信息 ID: 1f3b58cb27baccc4edb1b55c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=79228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$3
// 信息 ID: 819469aab396075f587bdb1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=64452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
// 信息 ID: 051a1062e4c2e3c425f87590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=113152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$3
// 信息 ID: dcbdc591f203bffbe3ee1035
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=146420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$4
// 信息 ID: d73b9c01e93c3a9455bb8813
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=205, inline_record=155720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$4::build
// 信息 ID: 464587c2bccd2e07719471aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::on_cursor_moved_absolute
// 信息 ID: 3975429def4e4f20d007a5bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x57b003 -> 0x578dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::shut_down_everything
// 信息 ID: cc06b491d3d5d49362625f55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=205, inline_record=7664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
