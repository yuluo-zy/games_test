// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/missing_mods.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f95de946b498414fa2786a6f4c10f64b981882e1de0221c5814425ece39825c2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: missing_mods.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1239, 1267, 1300, 2873, 2961
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2961,"record_offset":2080,"rva":37515984,"va":5406225104}
// 编译实例 RVAs: 0x23c72d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30548094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e882df2dfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5406226793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25161735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8f90f80fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5406225906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19180068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8dc55dbfe","size":5,"soft_fail":false,"target_va":5387045984,"va":5406226047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411909d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19098938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8c692dcfe","size":5,"soft_fail":false,"target_va":5387127248,"va":5406226181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411965b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19076001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e85fecdcfe","size":5,"soft_fail":false,"target_va":5387150768,"va":5406226764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411989c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19066730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e89610ddfe","size":5,"soft_fail":false,"target_va":5387160000,"va":5406226725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ac1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18986205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8234bdefe","size":5,"soft_fail":false,"target_va":5387239872,"va":5406226072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10124509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8dd7c9a00","size":5,"soft_fail":false,"target_va":5416351056,"va":5406226542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10125419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e86b809a00","size":5,"soft_fail":false,"target_va":5416351392,"va":5406225968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10126523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8bb849a00","size":5,"soft_fail":false,"target_va":5416353040,"va":5406226512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10394519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8979b9e00","size":5,"soft_fail":false,"target_va":5416621408,"va":5406226884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10394621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8fd9b9e00","size":5,"soft_fail":false,"target_va":5416621600,"va":5406226974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e88cfbffff","size":5,"soft_fail":false,"target_va":5406224048,"va":5406225183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10394359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8f79a9e00","size":5,"soft_fail":false,"target_va":5416621376,"va":5406227012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::missing_mods::MissingMods::has_issues
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26977777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e80f5a64fe","size":5,"soft_fail":false,"target_va":5379248064,"va":5406225836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22261751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e80950acfe","size":5,"soft_fail":false,"target_va":5383963792,"va":5406225538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22253247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e84171acfe","size":5,"soft_fail":false,"target_va":5383972224,"va":5406225466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22261711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e83150acfe","size":5,"soft_fail":false,"target_va":5383964912,"va":5406226618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22262345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8b74dacfe","size":5,"soft_fail":false,"target_va":5383962816,"va":5406225156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10429661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8dd249f00","size":5,"soft_fail":false,"target_va":5416656560,"va":5406226894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18896150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e8eaaadffe","size":5,"soft_fail":false,"target_va":5387331072,"va":5406227217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10392394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e84a939e00","size":5,"soft_fail":false,"target_va":5416619360,"va":5406226961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30019571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406225104,"raw_hex":"e80df035fe","size":5,"soft_fail":false,"target_va":5376206368,"va":5406225934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":6480,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":6592,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":7156,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2961,"record_offset":4464,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2961,"record_offset":5600,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2961,"record_offset":4524,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2961,"record_offset":6812,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2961,"record_offset":7860,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2961,"record_offset":6508,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":7016,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":3656,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":2260,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":2644,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2961,"record_offset":6284,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":22173,"source_line":250,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2961,"record_offset":6256,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2961,"record_offset":5940,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2961,"record_offset":6416,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2961,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":2080,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2961,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":2080,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2961,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":2080,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2961,"record_offset":8824,"parent_record_offset":8732,"ancestor_record_offset":2080,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":2080,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":2080,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":2080,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":2080,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":2080,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":2080,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":2080,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2961,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":2080,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":2080,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":2080,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":2080,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":2080,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":7680,"parent_record_offset":7212,"ancestor_record_offset":2080,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":6032,"parent_record_offset":5964,"ancestor_record_offset":2080,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":2080,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":7708,"parent_record_offset":7212,"ancestor_record_offset":2080,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":7388,"parent_record_offset":7212,"ancestor_record_offset":2080,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2961,"record_offset":4800,"parent_record_offset":4672,"ancestor_record_offset":2080,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2961,"record_offset":7184,"parent_record_offset":7156,"ancestor_record_offset":2080,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2961,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":2080,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2961,"record_offset":4672,"parent_record_offset":4628,"ancestor_record_offset":2080,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2961,"record_offset":5644,"parent_record_offset":5600,"ancestor_record_offset":2080,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2961,"record_offset":4628,"parent_record_offset":4568,"ancestor_record_offset":2080,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":2961,"record_offset":4568,"parent_record_offset":4524,"ancestor_record_offset":2080,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":2080,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":2080,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":2080,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":2080,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":4072,"parent_record_offset":3956,"ancestor_record_offset":2080,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2961,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":2080,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2961,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":2080,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2961,"record_offset":5692,"parent_record_offset":5644,"ancestor_record_offset":2080,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":2961,"record_offset":7444,"parent_record_offset":7416,"ancestor_record_offset":2080,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2961,"record_offset":7416,"parent_record_offset":7212,"ancestor_record_offset":2080,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2961,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":2080,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2961,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":2080,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2961,"record_offset":4744,"parent_record_offset":4672,"ancestor_record_offset":2080,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2961,"record_offset":8792,"parent_record_offset":8760,"ancestor_record_offset":2080,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2961,"record_offset":3804,"parent_record_offset":3764,"ancestor_record_offset":2080,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2961,"record_offset":7612,"parent_record_offset":7472,"ancestor_record_offset":2080,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2961,"record_offset":5740,"parent_record_offset":5692,"ancestor_record_offset":2080,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2961,"record_offset":5812,"parent_record_offset":5692,"ancestor_record_offset":2080,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2961,"record_offset":6372,"parent_record_offset":6312,"ancestor_record_offset":2080,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2961,"record_offset":5844,"parent_record_offset":5644,"ancestor_record_offset":2080,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2961,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2080,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2961,"record_offset":4420,"parent_record_offset":3764,"ancestor_record_offset":2080,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2961,"record_offset":4136,"parent_record_offset":3904,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2961,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":2080,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2961,"record_offset":6896,"parent_record_offset":6840,"ancestor_record_offset":2080,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2961,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2080,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2961,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2080,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2961,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":2080,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2961,"record_offset":6212,"parent_record_offset":6056,"ancestor_record_offset":2080,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2961,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":2080,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2961,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":2080,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2961,"record_offset":5152,"parent_record_offset":4672,"ancestor_record_offset":2080,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2961,"record_offset":5440,"parent_record_offset":4672,"ancestor_record_offset":2080,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2961,"record_offset":5216,"parent_record_offset":5184,"ancestor_record_offset":2080,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2961,"record_offset":5184,"parent_record_offset":5152,"ancestor_record_offset":2080,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2961,"record_offset":5468,"parent_record_offset":5440,"ancestor_record_offset":2080,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2080,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2080,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":2080,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":7472,"parent_record_offset":7444,"ancestor_record_offset":2080,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":6840,"parent_record_offset":6812,"ancestor_record_offset":2080,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2961,"record_offset":2968,"parent_record_offset":2852,"ancestor_record_offset":2080,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2961,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":2080,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2961,"record_offset":2680,"parent_record_offset":2644,"ancestor_record_offset":2080,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2961,"record_offset":2820,"parent_record_offset":2788,"ancestor_record_offset":2080,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2961,"record_offset":2788,"parent_record_offset":2752,"ancestor_record_offset":2080,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2961,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2080,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2961,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2080,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2961,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2080,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2961,"record_offset":3292,"parent_record_offset":2752,"ancestor_record_offset":2080,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2364,"parent_record_offset":2332,"ancestor_record_offset":2080,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":2080,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2488,"parent_record_offset":2456,"ancestor_record_offset":2080,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2540,"parent_record_offset":2488,"ancestor_record_offset":2080,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":2080,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2304,"parent_record_offset":2260,"ancestor_record_offset":2080,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2961,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2080,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2961,"record_offset":2852,"parent_record_offset":2820,"ancestor_record_offset":2080,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2961,"record_offset":3136,"parent_record_offset":2852,"ancestor_record_offset":2080,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2961,"record_offset":3108,"parent_record_offset":2852,"ancestor_record_offset":2080,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2961,"record_offset":2912,"parent_record_offset":2852,"ancestor_record_offset":2080,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":3692,"parent_record_offset":3656,"ancestor_record_offset":2080,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":7924,"parent_record_offset":7892,"ancestor_record_offset":2080,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":8380,"parent_record_offset":8176,"ancestor_record_offset":2080,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2961,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":2080,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":3904,"parent_record_offset":3764,"ancestor_record_offset":2080,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2961,"record_offset":4108,"parent_record_offset":3904,"ancestor_record_offset":2080,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2961,"record_offset":6344,"parent_record_offset":6312,"ancestor_record_offset":2080,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2961,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":2080,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2961,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":2080,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2961,"record_offset":3728,"parent_record_offset":3692,"ancestor_record_offset":2080,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2961,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":2080,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2961,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":2080,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2961,"record_offset":6004,"parent_record_offset":5964,"ancestor_record_offset":2080,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::missing_mods::ui::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/missing_mods.rs
//     evidence={"module":2961,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":2080,"inlinee":70679,"source_line":161,"archive":"_pdb/C/modules/02961.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":195224,"rva":18470048,"va":5387179168}
// 编译实例 RVAs: 0x119d4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11501057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8ff8150ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387179740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-11501412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e89c8050ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387180111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18457534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8425ce6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387179721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=4
//     target_aliases=6 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13700430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8b2f22eff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387179465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13694893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e853082fff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387179272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13694514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8ce092fff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387179293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6115077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8fbb0a2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387179248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5e620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2355342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8720fdcff","size":5,"soft_fail":false,"target_va":5384824352,"va":5387179689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-155813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e85b9ffdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387179360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-153870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8f2a6fdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387179321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8db620000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387179376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e804670000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387179335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29877289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e829e4c701","size":5,"soft_fail":false,"target_va":5417057056,"va":5387179762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2385227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8b59adbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387180086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29441078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8363cc101","size":5,"soft_fail":false,"target_va":5416620794,"va":5387179711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-62466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8fe0bffff","size":5,"soft_fail":false,"target_va":5387117360,"va":5387179821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29439873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e88137c101","size":5,"soft_fail":false,"target_va":5416621600,"va":5387181722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::missing_mods::MissingMods::mod_mesh_fit_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7931767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e889f886ff","size":5,"soft_fail":false,"target_va":5379248080,"va":5387179842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4044510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e82249c2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387179225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5165378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8be2eb1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387180429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5168239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e89123b1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387180474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_weight
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5167562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e83626b1ff","size":5,"soft_fail":false,"target_va":5382011856,"va":5387179413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10970948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387179168,"raw_hex":"e8bc9858ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387179311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":198636,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":196900,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":201748,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1239,"record_offset":199856,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":198720,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1239,"record_offset":196816,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":196760,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":200208,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":200148,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":200268,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195608,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195668,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195404,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195728,"parent_record_offset":195224,"ancestor_record_offset":195224,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":197568,"parent_record_offset":197544,"ancestor_record_offset":195224,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":197592,"parent_record_offset":197568,"ancestor_record_offset":195224,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":198896,"parent_record_offset":198872,"ancestor_record_offset":195224,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":198920,"parent_record_offset":198896,"ancestor_record_offset":195224,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":197616,"parent_record_offset":197592,"ancestor_record_offset":195224,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":198944,"parent_record_offset":198920,"ancestor_record_offset":195224,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":197544,"parent_record_offset":197380,"ancestor_record_offset":195224,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":198872,"parent_record_offset":198816,"ancestor_record_offset":195224,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":198684,"parent_record_offset":198660,"ancestor_record_offset":195224,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":197300,"parent_record_offset":197264,"ancestor_record_offset":195224,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":198844,"parent_record_offset":198816,"ancestor_record_offset":195224,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":198816,"parent_record_offset":198792,"ancestor_record_offset":195224,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":197380,"parent_record_offset":197336,"ancestor_record_offset":195224,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":197336,"parent_record_offset":197300,"ancestor_record_offset":195224,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":198792,"parent_record_offset":198768,"ancestor_record_offset":195224,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":197432,"parent_record_offset":197380,"ancestor_record_offset":195224,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":197152,"parent_record_offset":197116,"ancestor_record_offset":195224,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":197188,"parent_record_offset":197152,"ancestor_record_offset":195224,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":197224,"parent_record_offset":197188,"ancestor_record_offset":195224,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":197116,"parent_record_offset":197080,"ancestor_record_offset":195224,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1239,"record_offset":197080,"parent_record_offset":197044,"ancestor_record_offset":195224,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":196936,"parent_record_offset":196900,"ancestor_record_offset":195224,"inlinee":15783,"source_line":2973,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":197008,"parent_record_offset":196972,"ancestor_record_offset":195224,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":196972,"parent_record_offset":196936,"ancestor_record_offset":195224,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1239,"record_offset":197044,"parent_record_offset":197008,"ancestor_record_offset":195224,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":197264,"parent_record_offset":197224,"ancestor_record_offset":195224,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1239,"record_offset":197456,"parent_record_offset":197432,"ancestor_record_offset":195224,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":196512,"parent_record_offset":196488,"ancestor_record_offset":195224,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":196080,"parent_record_offset":196056,"ancestor_record_offset":195224,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":196164,"parent_record_offset":196140,"ancestor_record_offset":195224,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":196580,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":197504,"parent_record_offset":197480,"ancestor_record_offset":195224,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1239,"record_offset":197480,"parent_record_offset":197456,"ancestor_record_offset":195224,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":196312,"parent_record_offset":196284,"ancestor_record_offset":195224,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":196284,"parent_record_offset":196256,"ancestor_record_offset":195224,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":198660,"parent_record_offset":198636,"ancestor_record_offset":195224,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":201372,"parent_record_offset":200364,"ancestor_record_offset":195224,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1239,"record_offset":197672,"parent_record_offset":197224,"ancestor_record_offset":195224,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":196104,"parent_record_offset":196080,"ancestor_record_offset":195224,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":198744,"parent_record_offset":198720,"ancestor_record_offset":195224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":196428,"parent_record_offset":196404,"ancestor_record_offset":195224,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":195780,"parent_record_offset":195752,"ancestor_record_offset":195224,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":196536,"parent_record_offset":196512,"ancestor_record_offset":195224,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":196224,"parent_record_offset":196196,"ancestor_record_offset":195224,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":196256,"parent_record_offset":196224,"ancestor_record_offset":195224,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":196196,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":196056,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":196608,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":196140,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":196000,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":196488,"parent_record_offset":196464,"ancestor_record_offset":195224,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":195836,"parent_record_offset":195808,"ancestor_record_offset":195224,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":196864,"parent_record_offset":196840,"ancestor_record_offset":195224,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":196464,"parent_record_offset":196356,"ancestor_record_offset":195224,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":195868,"parent_record_offset":195836,"ancestor_record_offset":195224,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":196404,"parent_record_offset":196380,"ancestor_record_offset":195224,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":196700,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":196380,"parent_record_offset":196356,"ancestor_record_offset":195224,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":195920,"parent_record_offset":195896,"ancestor_record_offset":195224,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":195896,"parent_record_offset":195836,"ancestor_record_offset":195224,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":195944,"parent_record_offset":195920,"ancestor_record_offset":195224,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":196028,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":196356,"parent_record_offset":195944,"ancestor_record_offset":195224,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1239,"record_offset":196840,"parent_record_offset":196816,"ancestor_record_offset":195224,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":195808,"parent_record_offset":195752,"ancestor_record_offset":195224,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195516,"parent_record_offset":195456,"ancestor_record_offset":195224,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195456,"parent_record_offset":195404,"ancestor_record_offset":195224,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195752,"parent_record_offset":195728,"ancestor_record_offset":195224,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":196784,"parent_record_offset":196760,"ancestor_record_offset":195224,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195428,"parent_record_offset":195404,"ancestor_record_offset":195224,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195488,"parent_record_offset":195456,"ancestor_record_offset":195224,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":195540,"parent_record_offset":195516,"ancestor_record_offset":195224,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195636,"parent_record_offset":195608,"ancestor_record_offset":195224,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":195572,"parent_record_offset":195456,"ancestor_record_offset":195224,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11417
// 首条位置: {"module":1267,"inlinee":70679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11231
// 首条位置: {"module":1239,"inlinee":70193}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":139644,"parent_record_offset":139612,"ancestor_record_offset":139612,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":139672,"parent_record_offset":139612,"ancestor_record_offset":139612,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui::closure$3::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B399
// 首条位置: {"module":2873,"inlinee":111513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":61008,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":48360,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2873,"record_offset":60588,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":52708,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":2873,"record_offset":57036,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2873,"record_offset":48580,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":52652,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":60504,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":48468,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":48860,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":51524,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":50128,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":50068,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":58004,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":49864,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":50188,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":58064,"parent_record_offset":48052,"ancestor_record_offset":48052,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":52884,"parent_record_offset":52860,"ancestor_record_offset":48052,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":52908,"parent_record_offset":52884,"ancestor_record_offset":48052,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":52932,"parent_record_offset":52908,"ancestor_record_offset":48052,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":52860,"parent_record_offset":52804,"ancestor_record_offset":48052,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":61056,"parent_record_offset":61032,"ancestor_record_offset":48052,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":60816,"parent_record_offset":60784,"ancestor_record_offset":48052,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":52832,"parent_record_offset":52804,"ancestor_record_offset":48052,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":52804,"parent_record_offset":52780,"ancestor_record_offset":48052,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":60864,"parent_record_offset":60840,"ancestor_record_offset":48052,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":60840,"parent_record_offset":60816,"ancestor_record_offset":48052,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":52780,"parent_record_offset":52756,"ancestor_record_offset":48052,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":60784,"parent_record_offset":60744,"ancestor_record_offset":48052,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":60712,"parent_record_offset":60676,"ancestor_record_offset":48052,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":60744,"parent_record_offset":60712,"ancestor_record_offset":48052,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2873,"record_offset":60648,"parent_record_offset":60612,"ancestor_record_offset":48052,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2873,"record_offset":60612,"parent_record_offset":60588,"ancestor_record_offset":48052,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2873,"record_offset":60676,"parent_record_offset":60612,"ancestor_record_offset":48052,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2873,"record_offset":49620,"parent_record_offset":49596,"ancestor_record_offset":48052,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2873,"record_offset":49188,"parent_record_offset":49164,"ancestor_record_offset":48052,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2873,"record_offset":49272,"parent_record_offset":49248,"ancestor_record_offset":48052,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2873,"record_offset":49688,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2873,"record_offset":49420,"parent_record_offset":49392,"ancestor_record_offset":48052,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2873,"record_offset":49392,"parent_record_offset":49364,"ancestor_record_offset":48052,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":61032,"parent_record_offset":61008,"ancestor_record_offset":48052,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":51224,"parent_record_offset":50220,"ancestor_record_offset":48052,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":49212,"parent_record_offset":49188,"ancestor_record_offset":48052,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":52732,"parent_record_offset":52708,"ancestor_record_offset":48052,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2873,"record_offset":49536,"parent_record_offset":49512,"ancestor_record_offset":48052,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":48892,"parent_record_offset":48860,"ancestor_record_offset":48052,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":58128,"parent_record_offset":58096,"ancestor_record_offset":48052,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":60904,"parent_record_offset":60744,"ancestor_record_offset":48052,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2873,"record_offset":48392,"parent_record_offset":48360,"ancestor_record_offset":48052,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2873,"record_offset":48424,"parent_record_offset":48392,"ancestor_record_offset":48052,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":2873,"record_offset":57784,"parent_record_offset":57760,"ancestor_record_offset":48052,"inlinee":35007,"source_line":25,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2873,"record_offset":49644,"parent_record_offset":49620,"ancestor_record_offset":48052,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":49332,"parent_record_offset":49304,"ancestor_record_offset":48052,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":49364,"parent_record_offset":49332,"ancestor_record_offset":48052,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":49304,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":49164,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":49716,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=22
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":49248,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2873,"record_offset":49108,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2873,"record_offset":49596,"parent_record_offset":49572,"ancestor_record_offset":48052,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":57084,"parent_record_offset":57060,"ancestor_record_offset":48052,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":58224,"parent_record_offset":58188,"ancestor_record_offset":48052,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":48052,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":49572,"parent_record_offset":49464,"ancestor_record_offset":48052,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":48976,"parent_record_offset":48944,"ancestor_record_offset":48052,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49512,"parent_record_offset":49488,"ancestor_record_offset":48052,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49808,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49488,"parent_record_offset":49464,"ancestor_record_offset":48052,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49028,"parent_record_offset":49004,"ancestor_record_offset":48052,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":57144,"parent_record_offset":57084,"ancestor_record_offset":48052,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49004,"parent_record_offset":48944,"ancestor_record_offset":48052,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49052,"parent_record_offset":49028,"ancestor_record_offset":48052,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49136,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":49464,"parent_record_offset":49052,"ancestor_record_offset":48052,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":2873,"record_offset":57060,"parent_record_offset":57036,"ancestor_record_offset":48052,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":58188,"parent_record_offset":58156,"ancestor_record_offset":48052,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":48920,"parent_record_offset":48860,"ancestor_record_offset":48052,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":58156,"parent_record_offset":58096,"ancestor_record_offset":48052,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":49976,"parent_record_offset":49916,"ancestor_record_offset":48052,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":49916,"parent_record_offset":49864,"ancestor_record_offset":48052,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":58096,"parent_record_offset":58064,"ancestor_record_offset":48052,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":50220,"parent_record_offset":50188,"ancestor_record_offset":48052,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":52676,"parent_record_offset":52652,"ancestor_record_offset":48052,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":60528,"parent_record_offset":60504,"ancestor_record_offset":48052,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":60552,"parent_record_offset":60528,"ancestor_record_offset":48052,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":49888,"parent_record_offset":49864,"ancestor_record_offset":48052,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":48516,"parent_record_offset":48492,"ancestor_record_offset":48052,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":48544,"parent_record_offset":48492,"ancestor_record_offset":48052,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":48492,"parent_record_offset":48468,"ancestor_record_offset":48052,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":50096,"parent_record_offset":50068,"ancestor_record_offset":48052,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":58032,"parent_record_offset":58004,"ancestor_record_offset":48052,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":50032,"parent_record_offset":49916,"ancestor_record_offset":48052,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::missing_mods::ui::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1239,"record_offset":190676,"rva":18468752,"va":5387177872}
// 编译实例 RVAs: 0x119cf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11499446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e84a8850ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5387178129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11499992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8288650ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5387178691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18455923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e88d62e6fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5387178110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13699470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e872f62eff","size":5,"soft_fail":false,"target_va":5373479040,"va":5387178505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048dd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13693840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8700c2fff","size":5,"soft_fail":false,"target_va":5373484384,"va":5387178219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048def0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13693462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8ea0d2fff","size":5,"soft_fail":false,"target_va":5373484784,"va":5387178241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10941870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8520a59ff","size":5,"soft_fail":false,"target_va":5376236864,"va":5387178729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6114021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e81bb5a2ff","size":5,"soft_fail":false,"target_va":5381064176,"va":5387178192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-158006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8ca96fdff","size":5,"soft_fail":false,"target_va":5387020736,"va":5387178737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411774c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-154766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e872a3fdff","size":5,"soft_fail":false,"target_va":5387023552,"va":5387178313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e80eabfdff","size":5,"soft_fail":false,"target_va":5387025456,"va":5387178269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8f1660000","size":5,"soft_fail":false,"target_va":5387204688,"va":5387178330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e81e6b0000","size":5,"soft_fail":false,"target_va":5387205712,"va":5387178285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: missing_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29878900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e874eac701","size":5,"soft_fail":false,"target_va":5417057056,"va":5387178151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2383192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8a8a2dbff","size":5,"soft_fail":false,"target_va":5384794864,"va":5387178051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29442689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e88142c101","size":5,"soft_fail":false,"target_va":5416620794,"va":5387178100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29442464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8a041c101","size":5,"soft_fail":false,"target_va":5416621536,"va":5387179067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29442515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8d341c101","size":5,"soft_fail":false,"target_va":5416621600,"va":5387179080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4043252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e80c4ec2ff","size":5,"soft_fail":false,"target_va":5383134720,"va":5387177967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5163356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8a436b1ff","size":5,"soft_fail":false,"target_va":5382015056,"va":5387178407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5166217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8772bb1ff","size":5,"soft_fail":false,"target_va":5382012240,"va":5387178452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10969894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387177872,"raw_hex":"e8da9c58ff","size":5,"soft_fail":false,"target_va":5376208368,"va":5387178257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":191072,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1239,"record_offset":191156,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":194876,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":193304,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1239,"record_offset":190952,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1239,"record_offset":190868,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":193248,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":22042,"source_line":304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":194792,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":192048,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21735,"source_line":285,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":191988,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21814,"source_line":300,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":192108,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21736,"source_line":278,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":191928,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":193588,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21798,"source_line":357,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":193648,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":191724,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$30::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":192172,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":70093,"source_line":222,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$32::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":193708,"parent_record_offset":190676,"ancestor_record_offset":190676,"inlinee":21688,"source_line":216,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":191544,"parent_record_offset":191520,"ancestor_record_offset":190676,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":191568,"parent_record_offset":191544,"ancestor_record_offset":190676,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":193480,"parent_record_offset":193456,"ancestor_record_offset":190676,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":193504,"parent_record_offset":193480,"ancestor_record_offset":190676,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":191592,"parent_record_offset":191568,"ancestor_record_offset":190676,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":193528,"parent_record_offset":193504,"ancestor_record_offset":190676,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":191520,"parent_record_offset":191368,"ancestor_record_offset":190676,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1239,"record_offset":193456,"parent_record_offset":193400,"ancestor_record_offset":190676,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1239,"record_offset":191120,"parent_record_offset":191096,"ancestor_record_offset":190676,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":191312,"parent_record_offset":191288,"ancestor_record_offset":190676,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":193428,"parent_record_offset":193400,"ancestor_record_offset":190676,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":193400,"parent_record_offset":193376,"ancestor_record_offset":190676,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":191368,"parent_record_offset":191336,"ancestor_record_offset":190676,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":191336,"parent_record_offset":191312,"ancestor_record_offset":190676,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":193376,"parent_record_offset":193352,"ancestor_record_offset":190676,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1239,"record_offset":191408,"parent_record_offset":191368,"ancestor_record_offset":190676,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":191212,"parent_record_offset":191180,"ancestor_record_offset":190676,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":191236,"parent_record_offset":191212,"ancestor_record_offset":190676,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":191260,"parent_record_offset":191236,"ancestor_record_offset":190676,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1239,"record_offset":191180,"parent_record_offset":191156,"ancestor_record_offset":190676,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1239,"record_offset":191288,"parent_record_offset":191260,"ancestor_record_offset":190676,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1239,"record_offset":194904,"parent_record_offset":194876,"ancestor_record_offset":190676,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1239,"record_offset":191432,"parent_record_offset":191408,"ancestor_record_offset":190676,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":192972,"parent_record_offset":192948,"ancestor_record_offset":190676,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":192540,"parent_record_offset":192516,"ancestor_record_offset":190676,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":192624,"parent_record_offset":192600,"ancestor_record_offset":190676,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":193040,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":191480,"parent_record_offset":191456,"ancestor_record_offset":190676,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1239,"record_offset":191456,"parent_record_offset":191432,"ancestor_record_offset":190676,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":192772,"parent_record_offset":192744,"ancestor_record_offset":190676,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":192744,"parent_record_offset":192716,"ancestor_record_offset":190676,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":191096,"parent_record_offset":191072,"ancestor_record_offset":190676,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":193212,"parent_record_offset":192204,"ancestor_record_offset":190676,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1239,"record_offset":191648,"parent_record_offset":191260,"ancestor_record_offset":190676,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":191672,"parent_record_offset":191648,"ancestor_record_offset":190676,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":193328,"parent_record_offset":193304,"ancestor_record_offset":190676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":192888,"parent_record_offset":192864,"ancestor_record_offset":190676,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":191036,"parent_record_offset":190980,"ancestor_record_offset":190676,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":192240,"parent_record_offset":192204,"ancestor_record_offset":190676,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":193764,"parent_record_offset":193736,"ancestor_record_offset":190676,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":191008,"parent_record_offset":190980,"ancestor_record_offset":190676,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1239,"record_offset":190980,"parent_record_offset":190952,"ancestor_record_offset":190676,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":192996,"parent_record_offset":192972,"ancestor_record_offset":190676,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":192684,"parent_record_offset":192656,"ancestor_record_offset":190676,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":192716,"parent_record_offset":192684,"ancestor_record_offset":190676,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":192656,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":192516,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":193068,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":192600,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":192460,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":192948,"parent_record_offset":192924,"ancestor_record_offset":190676,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":193864,"parent_record_offset":193824,"ancestor_record_offset":190676,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":192296,"parent_record_offset":192268,"ancestor_record_offset":190676,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":190916,"parent_record_offset":190892,"ancestor_record_offset":190676,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":192924,"parent_record_offset":192816,"ancestor_record_offset":190676,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":192328,"parent_record_offset":192296,"ancestor_record_offset":190676,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192864,"parent_record_offset":192840,"ancestor_record_offset":190676,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":193160,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192840,"parent_record_offset":192816,"ancestor_record_offset":190676,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192380,"parent_record_offset":192356,"ancestor_record_offset":190676,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":193924,"parent_record_offset":193864,"ancestor_record_offset":190676,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192356,"parent_record_offset":192296,"ancestor_record_offset":190676,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192404,"parent_record_offset":192380,"ancestor_record_offset":190676,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192488,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":192816,"parent_record_offset":192404,"ancestor_record_offset":190676,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1239,"record_offset":190892,"parent_record_offset":190868,"ancestor_record_offset":190676,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":193824,"parent_record_offset":193792,"ancestor_record_offset":190676,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":192268,"parent_record_offset":192204,"ancestor_record_offset":190676,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":193792,"parent_record_offset":193736,"ancestor_record_offset":190676,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":191836,"parent_record_offset":191776,"ancestor_record_offset":190676,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":191776,"parent_record_offset":191724,"ancestor_record_offset":190676,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":193736,"parent_record_offset":193708,"ancestor_record_offset":190676,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":192204,"parent_record_offset":192172,"ancestor_record_offset":190676,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":193272,"parent_record_offset":193248,"ancestor_record_offset":190676,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":194816,"parent_record_offset":194792,"ancestor_record_offset":190676,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":194840,"parent_record_offset":194816,"ancestor_record_offset":190676,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":191748,"parent_record_offset":191724,"ancestor_record_offset":190676,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":191808,"parent_record_offset":191776,"ancestor_record_offset":190676,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":191860,"parent_record_offset":191836,"ancestor_record_offset":190676,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":191956,"parent_record_offset":191928,"ancestor_record_offset":190676,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":193616,"parent_record_offset":193588,"ancestor_record_offset":190676,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":191892,"parent_record_offset":191776,"ancestor_record_offset":190676,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
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
// 完整结构及原始类型记录: missing_mods.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::missing_mods::MissingModsPopupState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "enum2$<toolbar::menu::missing_mods::MissingModsPopupState>", "constructor": "enum2$", "arguments": ["toolbar::menu::missing_mods::MissingModsPopupState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["enum2$<toolbar::menu::missing_mods::MissingModsPopupState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["enum2$<toolbar::menu::missing_mods::MissingModsPopupState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$8::init_state<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >", "constructor": "bevy_ecs::system::system_param::impl$8::init_state", "arguments": ["enum2$<toolbar::menu::missing_mods::MissingModsPopupState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$118::init_access<bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >", "constructor": "bevy_ecs::system::system_param::impl$118::init_access", "arguments": ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: missing_mods.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: missing_mods.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui
// 信息 ID: 84f4981dadea22992a5f7d62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea254fe500af8f08e48b29c6: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> @ 0x11e49e9 -> 0x23c72d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<toolbar::menu::missing_mods::MissingModsPopupState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::SubscribeToWorkshopItemCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> > @ 0x2310701 -> 0x23c72d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui::closure$0
// 信息 ID: b0dd7e2e1d1628d4404ec99f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::missing_mods::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::missing_mods::ui::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$0,ref$<str$> > @ 0x1190c58 -> 0x119d4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui::closure$2
// 信息 ID: d320a5b5a32594e1d3489cef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::missing_mods::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::missing_mods::ui; module=2961, inline_record=7636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui::closure$3
// 信息 ID: 73c96ab41d7a21086ed49330
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::missing_mods::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::missing_mods::ui::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$3,ref$<str$> >; module=1239, inline_record=139612; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$3,ref$<str$> >; module=1239, inline_record=139732; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$3,ref$<str$> >; module=1239, inline_record=139824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui::closure$3::closure$0
// 信息 ID: 1e457538870e09b99091ad17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::missing_mods::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::missing_mods::ui::closure$3::closure_env$0 <- toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::missing_mods::ui::closure$3::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::missing_mods::ui::closure$3::closure_env$0>; module=2873, inline_record=48052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::missing_mods::ui::closure$4
// 信息 ID: be0ad7b6224892dddcc576a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::missing_mods::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::missing_mods::ui::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::missing_mods::ui::closure_env$4,ref$<str$> > @ 0x1196848 -> 0x119cf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
