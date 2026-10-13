// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ccf41c7c20669561717580f22c9e344f9da8baa9c9d9c35422d1439e1ba12c75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 330, 331, 1170, 2707, 2709, 2713, 2716, 2728, 2730, 2739, 3876, 3877, 3880, 3884, 3886, 3894
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2707,"record_offset":1456,"rva":30952192,"va":5399661312}
// 编译实例 RVAs: 0x1d84b00, 0x230f800, 0x230fcc0, 0x2317760, 0x231c590, 0x2326780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=24
//     evidence={"assembly":"\tcallq\t11880480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661312,"raw_hex":"e82048b500","size":5,"soft_fail":false,"target_va":5411542296,"va":5399661811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2707,"record_offset":1732,"parent_record_offset":1456,"ancestor_record_offset":1456,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2707,"record_offset":1860,"parent_record_offset":1456,"ancestor_record_offset":1456,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2707,"record_offset":1640,"parent_record_offset":1456,"ancestor_record_offset":1456,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2707,"record_offset":2012,"parent_record_offset":1456,"ancestor_record_offset":1456,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2707,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":1456,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2707,"record_offset":1948,"parent_record_offset":1888,"ancestor_record_offset":1456,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2707,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":1456,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2707,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1456,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2707,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":1456,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2707,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1456,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":1692,"parent_record_offset":1640,"ancestor_record_offset":1456,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::duck::move_ducks_along_trajectories
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3877,"record_offset":22812,"rva":36774400,"va":5405483520}
// 编译实例 RVAs: 0x2312200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a575a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25935570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e82e4174fe","size":5,"soft_fail":false,"target_va":5379552672,"va":5405488237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25935653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8db4074fe","size":5,"soft_fail":false,"target_va":5379552880,"va":5405488528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57da0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25933466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8664974fe","size":5,"soft_fail":false,"target_va":5379554720,"va":5405488181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23604954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e826d197fe","size":5,"soft_fail":false,"target_va":5381883040,"va":5405487989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23552100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e89c9f98fe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405486623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14261ef90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3194810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8babf3000","size":5,"soft_fail":false,"target_va":5408681872,"va":5405487057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14261f0c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3193954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e862bc3000","size":5,"soft_fail":false,"target_va":5408682176,"va":5405488217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924060
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6361250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8a2106100","size":5,"soft_fail":false,"target_va":5411848288,"va":5405487033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31133704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8f8ef24fe","size":5,"soft_fail":false,"target_va":5374354992,"va":5405488691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e807c6ffff","size":5,"soft_fail":false,"target_va":5405469440,"va":5405484276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23060368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e87020a0fe","size":5,"soft_fail":false,"target_va":5382424432,"va":5405484795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6052436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8545a5c00","size":5,"soft_fail":false,"target_va":5411541584,"va":5405489143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22578089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8577ca7fe","size":5,"soft_fail":false,"target_va":5382911104,"va":5405489188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6052569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8d95a5c00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405489202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6052892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e81c5c5c00","size":5,"soft_fail":false,"target_va":5411542064,"va":5405489167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6053049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8b95c5c00","size":5,"soft_fail":false,"target_va":5411542296,"va":5405489242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e83ed8ffff","size":5,"soft_fail":false,"target_va":5405473504,"va":5405483677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6059608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e858765c00","size":5,"soft_fail":false,"target_va":5411543360,"va":5405483747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25936159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8e13e74fe","size":5,"soft_fail":false,"target_va":5379552000,"va":5405488154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::audio_utils::AudioLoop::current
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25928041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8975e74fe","size":5,"soft_fail":false,"target_va":5379560432,"va":5405488468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::audio_utils::AudioLoop::start_if_not_playing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25927329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e85f6174fe","size":5,"soft_fail":false,"target_va":5379561136,"va":5405488460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::audio_utils::AudioLoop::stop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25927934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8025f74fe","size":5,"soft_fail":false,"target_va":5379560496,"va":5405488425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24098667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8954890fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405484198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24093478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8da5c90fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405484113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24102486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8aa3990fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405488961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24098949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e87b4790fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405483648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6322290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e872786000","size":5,"soft_fail":false,"target_va":5411811376,"va":5405489081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8efabffff","size":5,"soft_fail":false,"target_va":5405468144,"va":5405489660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6047968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405483520,"raw_hex":"e8e0485c00","size":5,"soft_fail":false,"target_va":5411537104,"va":5405489131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":38300,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3877,"record_offset":37020,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3877,"record_offset":40456,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3877,"record_offset":25684,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3877,"record_offset":25656,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3877,"record_offset":25480,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3877,"record_offset":36960,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3877,"record_offset":40064,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3877,"record_offset":25628,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3877,"record_offset":40176,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3877,"record_offset":36836,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3877,"record_offset":41016,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3877,"record_offset":36808,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3877,"record_offset":39452,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3877,"record_offset":37076,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38956,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38620,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":37788,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":37700,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":36872,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":39308,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3877,"record_offset":40848,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":24496,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":23064,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":23520,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::fract
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3877,"record_offset":38216,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":12960,"source_line":172,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3877,"record_offset":40092,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3877,"record_offset":38272,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3877,"record_offset":38396,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3877,"record_offset":39944,"parent_record_offset":22812,"ancestor_record_offset":22812,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3877,"record_offset":29448,"parent_record_offset":29424,"ancestor_record_offset":22812,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3877,"record_offset":29496,"parent_record_offset":29472,"ancestor_record_offset":22812,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3877,"record_offset":29472,"parent_record_offset":29448,"ancestor_record_offset":22812,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":40620,"parent_record_offset":40576,"ancestor_record_offset":22812,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":22812,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":22812,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":40576,"parent_record_offset":40540,"ancestor_record_offset":22812,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3877,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":22812,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3877,"record_offset":34992,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3877,"record_offset":31384,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3877,"record_offset":35272,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3877,"record_offset":31300,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3877,"record_offset":40512,"parent_record_offset":40480,"ancestor_record_offset":22812,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3877,"record_offset":40480,"parent_record_offset":40456,"ancestor_record_offset":22812,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3877,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":22812,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3877,"record_offset":40540,"parent_record_offset":40480,"ancestor_record_offset":22812,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":29600,"parent_record_offset":29572,"ancestor_record_offset":22812,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":31500,"parent_record_offset":31476,"ancestor_record_offset":22812,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":26072,"parent_record_offset":26048,"ancestor_record_offset":22812,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":29572,"parent_record_offset":29548,"ancestor_record_offset":22812,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":29628,"parent_record_offset":29600,"ancestor_record_offset":22812,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$73::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3877,"record_offset":29548,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":122455,"source_line":2083,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3877,"record_offset":25588,"parent_record_offset":25504,"ancestor_record_offset":22812,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3877,"record_offset":25708,"parent_record_offset":25684,"ancestor_record_offset":22812,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$8::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3877,"record_offset":31476,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":122456,"source_line":182,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$8::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3877,"record_offset":26048,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":122453,"source_line":195,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3877,"record_offset":26456,"parent_record_offset":26432,"ancestor_record_offset":22812,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3877,"record_offset":26164,"parent_record_offset":26136,"ancestor_record_offset":22812,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3877,"record_offset":26136,"parent_record_offset":26108,"ancestor_record_offset":22812,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3877,"record_offset":26300,"parent_record_offset":26276,"ancestor_record_offset":22812,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3877,"record_offset":29352,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3877,"record_offset":26108,"parent_record_offset":26072,"ancestor_record_offset":22812,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3877,"record_offset":29664,"parent_record_offset":29628,"ancestor_record_offset":22812,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3877,"record_offset":25964,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3877,"record_offset":26488,"parent_record_offset":26072,"ancestor_record_offset":22812,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3877,"record_offset":26432,"parent_record_offset":26072,"ancestor_record_offset":22812,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3877,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":22812,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3877,"record_offset":25556,"parent_record_offset":25532,"ancestor_record_offset":22812,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3877,"record_offset":25504,"parent_record_offset":25480,"ancestor_record_offset":22812,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3877,"record_offset":25532,"parent_record_offset":25504,"ancestor_record_offset":22812,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=27
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3877,"record_offset":29732,"parent_record_offset":29628,"ancestor_record_offset":22812,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24832,"parent_record_offset":24804,"ancestor_record_offset":22812,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24884,"parent_record_offset":24860,"ancestor_record_offset":22812,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24860,"parent_record_offset":24832,"ancestor_record_offset":22812,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24780,"parent_record_offset":24752,"ancestor_record_offset":22812,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24804,"parent_record_offset":24780,"ancestor_record_offset":22812,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":24976,"parent_record_offset":24804,"ancestor_record_offset":22812,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3877,"record_offset":25088,"parent_record_offset":25064,"ancestor_record_offset":22812,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3877,"record_offset":40224,"parent_record_offset":40200,"ancestor_record_offset":22812,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3877,"record_offset":40248,"parent_record_offset":40224,"ancestor_record_offset":22812,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3877,"record_offset":37520,"parent_record_offset":37100,"ancestor_record_offset":22812,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3877,"record_offset":37428,"parent_record_offset":37372,"ancestor_record_offset":22812,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3877,"record_offset":37196,"parent_record_offset":37172,"ancestor_record_offset":22812,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3877,"record_offset":37228,"parent_record_offset":37144,"ancestor_record_offset":22812,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3877,"record_offset":37292,"parent_record_offset":37100,"ancestor_record_offset":22812,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3877,"record_offset":39972,"parent_record_offset":39944,"ancestor_record_offset":22812,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":3877,"record_offset":38180,"parent_record_offset":38096,"ancestor_record_offset":22812,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3877,"record_offset":38152,"parent_record_offset":38096,"ancestor_record_offset":22812,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3877,"record_offset":24908,"parent_record_offset":24884,"ancestor_record_offset":22812,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3877,"record_offset":23220,"parent_record_offset":23188,"ancestor_record_offset":22812,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3877,"record_offset":37728,"parent_record_offset":37700,"ancestor_record_offset":22812,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3877,"record_offset":38892,"parent_record_offset":38760,"ancestor_record_offset":22812,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3877,"record_offset":39336,"parent_record_offset":39308,"ancestor_record_offset":22812,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3877,"record_offset":36900,"parent_record_offset":36872,"ancestor_record_offset":22812,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3877,"record_offset":24648,"parent_record_offset":24604,"ancestor_record_offset":22812,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3877,"record_offset":40200,"parent_record_offset":40176,"ancestor_record_offset":22812,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3877,"record_offset":26188,"parent_record_offset":26164,"ancestor_record_offset":22812,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3877,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":22812,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3877,"record_offset":26276,"parent_record_offset":26136,"ancestor_record_offset":22812,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3877,"record_offset":23468,"parent_record_offset":23440,"ancestor_record_offset":22812,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3877,"record_offset":25436,"parent_record_offset":24604,"ancestor_record_offset":22812,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3877,"record_offset":25040,"parent_record_offset":24752,"ancestor_record_offset":22812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3877,"record_offset":29700,"parent_record_offset":29664,"ancestor_record_offset":22812,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3877,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":22812,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3877,"record_offset":23932,"parent_record_offset":23908,"ancestor_record_offset":22812,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3877,"record_offset":23908,"parent_record_offset":23884,"ancestor_record_offset":22812,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3877,"record_offset":24116,"parent_record_offset":24088,"ancestor_record_offset":22812,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3877,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":22812,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3877,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":22812,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3877,"record_offset":31064,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3877,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":22812,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3877,"record_offset":38328,"parent_record_offset":38300,"ancestor_record_offset":22812,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3877,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":22812,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3877,"record_offset":26356,"parent_record_offset":26324,"ancestor_record_offset":22812,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3877,"record_offset":38356,"parent_record_offset":38328,"ancestor_record_offset":22812,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":23860,"parent_record_offset":23836,"ancestor_record_offset":22812,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":22812,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":23884,"parent_record_offset":23860,"ancestor_record_offset":22812,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":24188,"parent_record_offset":24160,"ancestor_record_offset":22812,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":25812,"parent_record_offset":25708,"ancestor_record_offset":22812,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":23836,"parent_record_offset":23720,"ancestor_record_offset":22812,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3877,"record_offset":25876,"parent_record_offset":25812,"ancestor_record_offset":22812,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3877,"record_offset":23188,"parent_record_offset":23160,"ancestor_record_offset":22812,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3877,"record_offset":23548,"parent_record_offset":23520,"ancestor_record_offset":22812,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3877,"record_offset":23688,"parent_record_offset":23656,"ancestor_record_offset":22812,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3877,"record_offset":23656,"parent_record_offset":23620,"ancestor_record_offset":22812,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3877,"record_offset":23596,"parent_record_offset":23572,"ancestor_record_offset":22812,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3877,"record_offset":23620,"parent_record_offset":23596,"ancestor_record_offset":22812,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3877,"record_offset":23572,"parent_record_offset":23548,"ancestor_record_offset":22812,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3877,"record_offset":24160,"parent_record_offset":23620,"ancestor_record_offset":22812,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23160,"parent_record_offset":23128,"ancestor_record_offset":22812,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23128,"parent_record_offset":23100,"ancestor_record_offset":22812,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23320,"parent_record_offset":23288,"ancestor_record_offset":22812,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23416,"parent_record_offset":23320,"ancestor_record_offset":22812,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23288,"parent_record_offset":23264,"ancestor_record_offset":22812,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23100,"parent_record_offset":23064,"ancestor_record_offset":22812,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3877,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":22812,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3877,"record_offset":23720,"parent_record_offset":23688,"ancestor_record_offset":22812,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3877,"record_offset":24004,"parent_record_offset":23720,"ancestor_record_offset":22812,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3877,"record_offset":23976,"parent_record_offset":23720,"ancestor_record_offset":22812,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3877,"record_offset":23780,"parent_record_offset":23720,"ancestor_record_offset":22812,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3877,"record_offset":39568,"parent_record_offset":39476,"ancestor_record_offset":22812,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3877,"record_offset":39476,"parent_record_offset":39452,"ancestor_record_offset":22812,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3877,"record_offset":37100,"parent_record_offset":37076,"ancestor_record_offset":22812,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3877,"record_offset":37548,"parent_record_offset":37100,"ancestor_record_offset":22812,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38824,"parent_record_offset":38792,"ancestor_record_offset":22812,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38792,"parent_record_offset":38760,"ancestor_record_offset":22812,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38760,"parent_record_offset":38736,"ancestor_record_offset":22812,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":38980,"parent_record_offset":38956,"ancestor_record_offset":22812,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3877,"record_offset":39128,"parent_record_offset":38956,"ancestor_record_offset":22812,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3877,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":22812,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3877,"record_offset":37144,"parent_record_offset":37100,"ancestor_record_offset":22812,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3877,"record_offset":26324,"parent_record_offset":26300,"ancestor_record_offset":22812,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":24532,"parent_record_offset":24496,"ancestor_record_offset":22812,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":41080,"parent_record_offset":41048,"ancestor_record_offset":22812,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":41580,"parent_record_offset":41320,"ancestor_record_offset":22812,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3877,"record_offset":24676,"parent_record_offset":24648,"ancestor_record_offset":22812,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":24752,"parent_record_offset":24604,"ancestor_record_offset":22812,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3877,"record_offset":25012,"parent_record_offset":24752,"ancestor_record_offset":22812,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3877,"record_offset":38856,"parent_record_offset":38824,"ancestor_record_offset":22812,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::trunc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3877,"record_offset":38240,"parent_record_offset":38216,"ancestor_record_offset":22812,"inlinee":12961,"source_line":149,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3877,"record_offset":24704,"parent_record_offset":24676,"ancestor_record_offset":22812,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3877,"record_offset":24604,"parent_record_offset":24568,"ancestor_record_offset":22812,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3877,"record_offset":24568,"parent_record_offset":24532,"ancestor_record_offset":22812,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03877.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":331,"record_offset":264,"rva":6804544,"va":5375513664}
// 编译实例 RVAs: 0x67d440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5897704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e81802a6ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5375513811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-432633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e80766f9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375514116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f7e4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9441387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e86b109000","size":5,"soft_fail":false,"target_va":5384955104,"va":5375513712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f85b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9471481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8f9859000","size":5,"soft_fail":false,"target_va":5384985488,"va":5375514002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36026363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8fbb72502","size":5,"soft_fail":false,"target_va":5411540992,"va":5375514624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9465321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8e96d9000","size":5,"soft_fail":false,"target_va":5384979248,"va":5375513922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9449862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e886319000","size":5,"soft_fail":false,"target_va":5384963616,"va":5375513749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9525618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e872599100","size":5,"soft_fail":false,"target_va":5385039984,"va":5375514361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9437134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8ceff8f00","size":5,"soft_fail":false,"target_va":5384950832,"va":5375513693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9448769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8412d9000","size":5,"soft_fail":false,"target_va":5384962512,"va":5375513738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9456233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8694a9000","size":5,"soft_fail":false,"target_va":5384970144,"va":5375513906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36027290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e89abb2502","size":5,"soft_fail":false,"target_va":5411541904,"va":5375514609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::fish::FishSpawnLocations::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9515875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e863339100","size":5,"soft_fail":false,"target_va":5385030128,"va":5375514248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::spawn_ducks::DuckSpawnLocations::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9509035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375513664,"raw_hex":"e8ab189100","size":5,"soft_fail":false,"target_va":5385023120,"va":5375514080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t9530241","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375513664,"raw_hex":"e9816b9100","size":5,"soft_fail":false,"target_va":5385044736,"va":5375514490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":331,"record_offset":2096,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":331,"record_offset":400,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$36::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":331,"record_offset":1696,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19694,"source_line":309,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_direct] system_critters::duck::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/duck.rs
//     evidence={"module":331,"record_offset":1668,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19692,"source_line":79,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":331,"record_offset":2036,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19696,"source_line":44,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_direct] system_critters::wren_boids::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/wren_boids.rs
//     evidence={"module":331,"record_offset":1608,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":19688,"source_line":36,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":331,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":331,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":331,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":331,"record_offset":1760,"parent_record_offset":1724,"ancestor_record_offset":264,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":331,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":264,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":331,"record_offset":1820,"parent_record_offset":1788,"ancestor_record_offset":264,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":331,"record_offset":1972,"parent_record_offset":1724,"ancestor_record_offset":264,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":331,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":331,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":331,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":331,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":264,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":331,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":331,"record_offset":968,"parent_record_offset":928,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":331,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":331,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":264,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":331,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":9172,"source_line":211,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":331,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":264,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":331,"record_offset":1544,"parent_record_offset":484,"ancestor_record_offset":264,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":331,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":264,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":331,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":264,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":331,"record_offset":1240,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":331,"record_offset":1184,"parent_record_offset":644,"ancestor_record_offset":264,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":331,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":264,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":331,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":264,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":331,"record_offset":1268,"parent_record_offset":1208,"ancestor_record_offset":264,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":331,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":264,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":331,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":264,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":331,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":264,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":331,"record_offset":2148,"parent_record_offset":2120,"ancestor_record_offset":264,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":331,"record_offset":556,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":331,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":264,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":331,"record_offset":520,"parent_record_offset":484,"ancestor_record_offset":264,"inlinee":9173,"source_line":2000,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":331,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":264,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":331,"record_offset":1360,"parent_record_offset":644,"ancestor_record_offset":264,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":331,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":331,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":264,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":331,"record_offset":644,"parent_record_offset":580,"ancestor_record_offset":264,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":331,"record_offset":1484,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":331,"record_offset":1512,"parent_record_offset":520,"ancestor_record_offset":264,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":331,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":264,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":331,"record_offset":816,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":331,"record_offset":2180,"parent_record_offset":2148,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":331,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":331,"record_offset":2684,"parent_record_offset":2148,"ancestor_record_offset":264,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":331,"record_offset":880,"parent_record_offset":700,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":331,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":264,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":331,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":331,"record_offset":1132,"parent_record_offset":1108,"ancestor_record_offset":264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":331,"record_offset":788,"parent_record_offset":752,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":331,"record_offset":1108,"parent_record_offset":928,"ancestor_record_offset":264,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":331,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":264,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":331,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":264,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$6::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":331,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":264,"inlinee":19690,"source_line":78,"archive":"_pdb/B/modules/00331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::critter_control
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CF4
// 首条位置: {"module":1170,"inlinee":19700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1170,"record_offset":4932,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01170.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1170,"record_offset":5044,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01170.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1170,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1170,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":4836,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1170,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4836,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01170.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::CritterPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>", "constructor": "bevy_app::plugin::sealed::impl$12::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_critters::CritterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_critters::CritterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_critters::CritterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_critters::CritterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_critters::CritterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<system_critters::CritterPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_critters::CritterPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_critters::CritterPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_critters::CritterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 44; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Fish> >,bevy_ecs::query::filter::Without<country_core::resources::critters::Duck> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,bevy_ecs::query::filter::Without<country_core::resources::critters::Fish> >,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<system_critters::wren_boids::WrenRng>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::paths::draw_path::PathRastersResources>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Fish> >,bevy_ecs::query::filter::Without<country_core::resources::critters::Duck> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,bevy_ecs::query::filter::Without<country_core::resources::critters::Fish> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<system_critters::wren_boids::WrenRng>", "bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::paths::draw_path::PathRastersResources>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::Duck>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Fish> >,bevy_ecs::query::filter::Without<country_core::resources::critters::Duck> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::Fish>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,bevy_ecs::query::filter::Without<country_core::resources::critters::Fish> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_critters::wren_boids::WrenRng>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>,bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>", "bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::fish::FishBoids>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::fish::FishSpawnLocations>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> > >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> > >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> > >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,country_core::systems::paths::draw_path::PathRastersResources)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "country_core::systems::paths::draw_path::PathRastersResources"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::system::query::Query<ref$<country_core::resources::critters::Duck>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_critters::spawn_ducks::DuckSpawnLocations>,bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::system::query::Query<ref$<country_core::resources::critters::Duck>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_critters::spawn_ducks::DuckSpawnLocations>", "bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::critters::Duck>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::spawn_ducks::DuckSpawnLocations>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::water::WaterDrawEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::reader::EventReader<system_critters::spawn_ducks::DespawnDuckCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >", "country_core::systems::paths::draw_path::PathRastersResources", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::reader::EventReader<system_critters::spawn_ducks::DespawnDuckCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_critters::duck::DuckTrajectoryPlanner>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<system_critters::spawn_ducks::DespawnDuckCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1d84ab7 -> 0x1d84b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x230f7b7 -> 0x230f800 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x230fc77 -> 0x230fcc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x2317717 -> 0x2317760 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x231c547 -> 0x231c590 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x2326737 -> 0x2326780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::critter_control
// 信息 ID: 3bc696bebe609c70f3a9bcbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f427c7059ccd0166ab8f8bc5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> >; module=1170, inline_record=4836; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > >; module=2716, inline_record=434212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::duck::move_ducks_along_trajectories
// 信息 ID: 124beb684a97e34e954d0022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a0d20b4b6c670d1439b9a9bd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::duck_splash_particle::SpawnSplashParticle>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::duck_splash_particle::SpawnSplashParticle>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::duck_splash_particle::SpawnSplashParticle>)> > @ 0x1d8ea8e -> 0x2312200 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::duck_splash_particle::SpawnSplashParticle>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::critters::Duck>,ref_mut$<enum2$<country_core::resources::critters::DuckState> >,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier>,ref_mut$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<system_critters::duck_splash_particle::SpawnSplashParticle>)> > > @ 0x1da2056 -> 0x2312200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::impl$0::build
// 信息 ID: 2dee9bcd610b3fdd7572e532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
