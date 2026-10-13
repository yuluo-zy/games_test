// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/clutter/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 879adcd20a06d5eabb472c948841217d85c56471296ebded5027d719e5ee49cd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 318, 1134, 2741, 2744, 2750, 2751, 2753, 4053, 4054, 4056, 4057, 4058, 4061, 4062, 4066
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=12
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2741,"record_offset":25236,"rva":35369728,"va":5404078848}
// 编译实例 RVAs: 0x21bb300, 0x21c3be0, 0x21c4840, 0x21c7dd0, 0x28bb9d0, 0x28c7040, 0x28c8630, 0x28c95a0, 0x28ca2b0, 0x28cc4d0, 0x28cf1e0, 0x28d2800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=48
//     evidence={"assembly":"\tcallq\t12542416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404078848,"raw_hex":"e8d061bf00","size":5,"soft_fail":false,"target_va":5416621762,"va":5404079341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":2741,"record_offset":25512,"parent_record_offset":25236,"ancestor_record_offset":25236,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2741,"record_offset":25640,"parent_record_offset":25236,"ancestor_record_offset":25236,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2741,"record_offset":25420,"parent_record_offset":25236,"ancestor_record_offset":25236,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2741,"record_offset":25792,"parent_record_offset":25236,"ancestor_record_offset":25236,"inlinee":9307,"source_line":1740,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2741,"record_offset":25596,"parent_record_offset":25568,"ancestor_record_offset":25236,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2741,"record_offset":25728,"parent_record_offset":25668,"ancestor_record_offset":25236,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2741,"record_offset":25668,"parent_record_offset":25640,"ancestor_record_offset":25236,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2741,"record_offset":25568,"parent_record_offset":25540,"ancestor_record_offset":25236,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":2741,"record_offset":25540,"parent_record_offset":25512,"ancestor_record_offset":25236,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2741,"record_offset":25752,"parent_record_offset":25728,"ancestor_record_offset":25236,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2741,"record_offset":25472,"parent_record_offset":25420,"ancestor_record_offset":25236,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02741.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":1520,"rva":7274896,"va":5375984016}
// 编译实例 RVAs: 0x6f0190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t8659777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e841238400","size":5,"soft_fail":false,"target_va":5384644912,"va":5375985130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8234638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e88ea67d00","size":5,"soft_fail":false,"target_va":5384218864,"va":5375984221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10354338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8a2fe9d00","size":5,"soft_fail":false,"target_va":5386338576,"va":5375984233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-305493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8ab56fbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375984176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7261974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8ea3091ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375984161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=2
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6783238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8fa7e98ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5375988897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6324351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8817f9fff","size":5,"soft_fail":false,"target_va":5369660368,"va":5375984714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5386682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e846ceadff","size":5,"soft_fail":false,"target_va":5370597904,"va":5375984581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096dc90
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2606871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e817c72700","size":5,"soft_fail":false,"target_va":5378595984,"va":5375989108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a381d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3438838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8f6783400","size":5,"soft_fail":false,"target_va":5379424720,"va":5375985877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad3fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4077246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8be363e00","size":5,"soft_fail":false,"target_va":5380063184,"va":5375985933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410cab00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10332452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e824a99d00","size":5,"soft_fail":false,"target_va":5386316544,"va":5375984087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410ce420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10347046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e826e29d00","size":5,"soft_fail":false,"target_va":5386331168,"va":5375984117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410cf430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10351130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e81af29d00","size":5,"soft_fail":false,"target_va":5386335280,"va":5375984145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d1f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10362085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8e51c9e00","size":5,"soft_fail":false,"target_va":5386346336,"va":5375984246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d29e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10363656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e808239e00","size":5,"soft_fail":false,"target_va":5386349024,"va":5375985363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d2b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10364868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8c4279e00","size":5,"soft_fail":false,"target_va":5386349408,"va":5375984535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d5610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10375512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e858519e00","size":5,"soft_fail":false,"target_va":5386360336,"va":5375984819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40632063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8fffe6b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375988701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10383193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8596f9e00","size":5,"soft_fail":false,"target_va":5386368176,"va":5375984978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10326474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8ca919d00","size":5,"soft_fail":false,"target_va":5386310544,"va":5375984065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10369417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e889399e00","size":5,"soft_fail":false,"target_va":5386353984,"va":5375984562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10410134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e896d89e00","size":5,"soft_fail":false,"target_va":5386396400,"va":5375986261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10354706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e812009e00","size":5,"soft_fail":false,"target_va":5386339072,"va":5375984361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10319252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e894759d00","size":5,"soft_fail":false,"target_va":5386308304,"va":5375989047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t8737974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8b6548500","size":5,"soft_fail":false,"target_va":5384726688,"va":5375988709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40630500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375984016,"raw_hex":"e8e4f86b02","size":5,"soft_fail":false,"target_va":5416619360,"va":5375988855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":318,"record_offset":6740,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":318,"record_offset":1704,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":318,"record_offset":1816,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":318,"record_offset":2080,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_meshes::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":318,"record_offset":6020,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20499,"source_line":45,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::impl$31::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":318,"record_offset":4540,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20495,"source_line":239,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::shape_intersections::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shape_intersections.rs
//     evidence={"module":318,"record_offset":2868,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20477,"source_line":8,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":318,"record_offset":3736,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20487,"source_line":66,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision.rs
//     evidence={"module":318,"record_offset":3764,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20489,"source_line":79,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_clutter::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_clutter/mod.rs
//     evidence={"module":318,"record_offset":3820,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20493,"source_line":20,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_clutter::ui_focus_decoclutter::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_clutter/ui_focus_decoclutter.rs
//     evidence={"module":318,"record_offset":5988,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20497,"source_line":52,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":318,"record_offset":2208,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20467,"source_line":139,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::populate_autoclutter_regions::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs
//     evidence={"module":318,"record_offset":3792,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20491,"source_line":77,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":318,"record_offset":3616,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20481,"source_line":114,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_roof_merlons/detect_inter_roof_merlons.rs
//     evidence={"module":318,"record_offset":3588,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20479,"source_line":42,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::detect_intra_shape_corners::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/detect_intra_shape_corners.rs
//     evidence={"module":318,"record_offset":2840,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20475,"source_line":32,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::stitch_bricks::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/stitch_bricks.rs
//     evidence={"module":318,"record_offset":2784,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20471,"source_line":243,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::inter_shape_stitches::stitch_bricks::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/inter_shape_stitches/stitch_bricks.rs
//     evidence={"module":318,"record_offset":2812,"parent_record_offset":1520,"ancestor_record_offset":1520,"inlinee":20473,"source_line":249,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1520,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":1520,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1520,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":1948,"parent_record_offset":1916,"ancestor_record_offset":1520,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":318,"record_offset":1916,"parent_record_offset":1880,"ancestor_record_offset":1520,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":318,"record_offset":1880,"parent_record_offset":1844,"ancestor_record_offset":1520,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":318,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":1520,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":318,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":1520,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":318,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":1520,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":1520,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":8088,"parent_record_offset":8048,"ancestor_record_offset":1520,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":2640,"parent_record_offset":2208,"ancestor_record_offset":1520,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":1520,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":318,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":1520,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":318,"record_offset":6824,"parent_record_offset":6792,"ancestor_record_offset":1520,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":318,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":1520,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":318,"record_offset":2120,"parent_record_offset":2080,"ancestor_record_offset":1520,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":318,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":1520,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":318,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":1520,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":318,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":1520,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":318,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":1520,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":318,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":1520,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":318,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":1520,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":318,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":1520,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":318,"record_offset":7772,"parent_record_offset":7748,"ancestor_record_offset":1520,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":318,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":1520,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":318,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":1520,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":318,"record_offset":7524,"parent_record_offset":7496,"ancestor_record_offset":1520,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":318,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":1520,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":318,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1520,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":318,"record_offset":2476,"parent_record_offset":2420,"ancestor_record_offset":1520,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":1520,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":1520,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":318,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1520,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":318,"record_offset":3468,"parent_record_offset":3412,"ancestor_record_offset":1520,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":1520,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":7268,"parent_record_offset":7240,"ancestor_record_offset":1520,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":318,"record_offset":3048,"parent_record_offset":3016,"ancestor_record_offset":1520,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":318,"record_offset":7748,"parent_record_offset":6792,"ancestor_record_offset":1520,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":318,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":1520,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":318,"record_offset":8284,"parent_record_offset":6792,"ancestor_record_offset":1520,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":318,"record_offset":8000,"parent_record_offset":7820,"ancestor_record_offset":1520,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":318,"record_offset":3168,"parent_record_offset":3136,"ancestor_record_offset":1520,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":318,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":1520,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":1520,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":2668,"parent_record_offset":2208,"ancestor_record_offset":1520,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":3256,"parent_record_offset":3136,"ancestor_record_offset":1520,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":7412,"parent_record_offset":7388,"ancestor_record_offset":1520,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":1520,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":318,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":1520,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":318,"record_offset":7908,"parent_record_offset":7872,"ancestor_record_offset":1520,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":318,"record_offset":8228,"parent_record_offset":8048,"ancestor_record_offset":1520,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":318,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":1520,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":318,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":1520,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":318,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":1520,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":318,"record_offset":7180,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":318,"record_offset":7040,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":318,"record_offset":7600,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":318,"record_offset":7124,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":318,"record_offset":6884,"parent_record_offset":6848,"ancestor_record_offset":1520,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":318,"record_offset":7448,"parent_record_offset":7340,"ancestor_record_offset":1520,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":7388,"parent_record_offset":7364,"ancestor_record_offset":1520,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":7692,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":1520,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":1520,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":6912,"parent_record_offset":6884,"ancestor_record_offset":1520,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":1520,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":7012,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":318,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1520,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":318,"record_offset":7340,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":318,"record_offset":7572,"parent_record_offset":6960,"ancestor_record_offset":1520,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":2580,"parent_record_offset":2248,"ancestor_record_offset":1520,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":1520,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":318,"record_offset":3856,"parent_record_offset":3820,"ancestor_record_offset":1520,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":318,"record_offset":2304,"parent_record_offset":2276,"ancestor_record_offset":1520,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":318,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":1520,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":318,"record_offset":3076,"parent_record_offset":3048,"ancestor_record_offset":1520,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":318,"record_offset":2420,"parent_record_offset":2364,"ancestor_record_offset":1520,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":318,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":1520,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":318,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":1520,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":318,"record_offset":3136,"parent_record_offset":3104,"ancestor_record_offset":1520,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":318,"record_offset":2364,"parent_record_offset":2332,"ancestor_record_offset":1520,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":318,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":1520,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":318,"record_offset":2612,"parent_record_offset":2208,"ancestor_record_offset":1520,"inlinee":20469,"source_line":133,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::read_bush_positions_from_gpu::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs
//     evidence={"module":318,"record_offset":3648,"parent_record_offset":3616,"ancestor_record_offset":1520,"inlinee":20483,"source_line":18,"archive":"_pdb/C/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":318,"record_offset":3704,"parent_record_offset":3616,"ancestor_record_offset":1520,"inlinee":20485,"source_line":50,"archive":"_pdb/C/modules/00318.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] system_clutter::ClutterPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<system_clutter::ClutterPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<system_clutter::ClutterPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<system_clutter::ClutterPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,system_clutter::ClutterPlugin>", "constructor": "bevy_app::app::App::add_plugins", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "system_clutter::ClutterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 0538cde337678dd7ea2e895d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21bb1d7 -> 0x21bb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21c3ab7 -> 0x21c3be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21c4717 -> 0x21c4840 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x21c7ca3 -> 0x21c7dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28bb8a7 -> 0x28bb9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28c6f17 -> 0x28c7040 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28c8507 -> 0x28c8630 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28c9477 -> 0x28c95a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28ca187 -> 0x28ca2b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28cc3a7 -> 0x28cc4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28cf0b7 -> 0x28cf1e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x28d26d7 -> 0x28d2800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::impl$0::build
// 信息 ID: 41f3c362fbb315d7c0c8d661
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
