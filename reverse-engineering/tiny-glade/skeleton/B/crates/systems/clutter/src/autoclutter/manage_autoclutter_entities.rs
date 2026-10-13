// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2e12f2611a7d0011a692ce6ac3f674e26ee375f8e34ed08240ee34b9e3d6b188
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: manage_autoclutter_entities.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1082, 2611, 2617, 3796, 3806, 4489
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3806,"record_offset":2040,"rva":36504800,"va":5405213920}
// 编译实例 RVAs: 0x22d04e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=5
//     target_aliases=532 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36266516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8ec9dd6fd","size":5,"soft_fail":false,"target_va":5368953728,"va":5405220239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=1
//     target_aliases=46 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35163986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8ae70e7fd","size":5,"soft_fail":false,"target_va":5370059888,"va":5405223869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30539605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8ab002efe","size":5,"soft_fail":false,"target_va":5374676336,"va":5405215936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30140640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8201734fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405222139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cd70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23286011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e805af9cfe","size":5,"soft_fail":false,"target_va":5381934448,"va":5405220454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ceaad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6187368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e89896a1ff","size":5,"soft_fail":false,"target_va":5399030480,"va":5405217843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142612aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: manage_autoclutter_entities.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3416814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8ee223400","size":5,"soft_fail":false,"target_va":5408631456,"va":5405214637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30863873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8ff0d29fe","size":5,"soft_fail":false,"target_va":5374354992,"va":5405218860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t6552161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e861fa6300","size":5,"soft_fail":false,"target_va":5411768560,"va":5405216394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22795405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8732ba4fe","size":5,"soft_fail":false,"target_va":5382424224,"va":5405219624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8e49cffff","size":5,"soft_fail":false,"target_va":5405194784,"va":5405220151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6319108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8046c6000","size":5,"soft_fail":false,"target_va":5411541584,"va":5405222471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6319288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8b86c6000","size":5,"soft_fail":false,"target_va":5411541776,"va":5405222483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e832fbffff","size":5,"soft_fail":false,"target_va":5405212848,"va":5405214073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6329205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e875936000","size":5,"soft_fail":false,"target_va":5411543360,"va":5405214150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23209675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e835d99dfe","size":5,"soft_fail":false,"target_va":5382009728,"va":5405219398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23829051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8c56594fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405214582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23823862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e80a7a94fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405214497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23835814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e85a4b94fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405222289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23829354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8966494fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405214053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6586738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e872816400","size":5,"soft_fail":false,"target_va":5411809152,"va":5405222409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20848080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e830e2c1fe","size":5,"soft_fail":false,"target_va":5384376048,"va":5405224123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6314640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8905a6000","size":5,"soft_fail":false,"target_va":5411537104,"va":5405222459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8341e0000","size":5,"soft_fail":false,"target_va":5405224224,"va":5405216487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23285969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e82faf9cfe","size":5,"soft_fail":false,"target_va":5381934464,"va":5405220428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23310615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405213920,"raw_hex":"e8e94e9cfe","size":5,"soft_fail":false,"target_va":5381907008,"va":5405217618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":16080,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":16904,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":21296,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3806,"record_offset":21680,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3806,"record_offset":13908,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3806,"record_offset":5320,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13584,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":22364,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13640,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":16108,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":4872,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":18488,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":4844,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":16460,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":14020,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":22856,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":15472,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":23224,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3806,"record_offset":24900,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3806,"record_offset":17768,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3806,"record_offset":4760,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] country_core::components::impl$92::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3806,"record_offset":22712,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":121665,"source_line":396,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3806,"record_offset":21824,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":3776,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":2344,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":2800,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3806,"record_offset":11412,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3806,"record_offset":16572,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3806,"record_offset":17096,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3806,"record_offset":21964,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58563,"source_line":82,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterType::is_plant
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3806,"record_offset":23584,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58562,"source_line":78,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterType::is_window
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3806,"record_offset":21796,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58561,"source_line":74,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterType::is_window_shutter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3806,"record_offset":22884,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58564,"source_line":86,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::manage_autoclutter_entities::update_shutter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs
//     evidence={"module":3806,"record_offset":14048,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":121677,"source_line":249,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3806,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":2040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3806,"record_offset":25132,"parent_record_offset":25044,"ancestor_record_offset":2040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":2040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":2040,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":2040,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":25072,"parent_record_offset":25044,"ancestor_record_offset":2040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":25044,"parent_record_offset":25020,"ancestor_record_offset":2040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":2040,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":2040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":2040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":2040,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":25020,"parent_record_offset":24996,"ancestor_record_offset":2040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3806,"record_offset":22764,"parent_record_offset":22712,"ancestor_record_offset":2040,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":16944,"parent_record_offset":16904,"ancestor_record_offset":2040,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":2040,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":5104,"parent_record_offset":5072,"ancestor_record_offset":2040,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":2040,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":2040,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":13740,"parent_record_offset":13712,"ancestor_record_offset":2040,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":2040,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":2040,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3806,"record_offset":24924,"parent_record_offset":24900,"ancestor_record_offset":2040,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3806,"record_offset":24972,"parent_record_offset":24948,"ancestor_record_offset":2040,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":2040,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":10388,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":7708,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":7792,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":10668,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3806,"record_offset":7624,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3806,"record_offset":13932,"parent_record_offset":13908,"ancestor_record_offset":2040,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3806,"record_offset":15044,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9792,"parent_record_offset":9764,"ancestor_record_offset":2040,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":7916,"parent_record_offset":7884,"ancestor_record_offset":2040,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":5804,"parent_record_offset":5768,"ancestor_record_offset":2040,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9764,"parent_record_offset":9732,"ancestor_record_offset":2040,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9820,"parent_record_offset":9792,"ancestor_record_offset":2040,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":6344,"parent_record_offset":6308,"ancestor_record_offset":2040,"inlinee":10339,"source_line":1577,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":8712,"parent_record_offset":8412,"ancestor_record_offset":2040,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":6632,"parent_record_offset":6308,"ancestor_record_offset":2040,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":2040,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":10020,"parent_record_offset":9996,"ancestor_record_offset":2040,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":8412,"parent_record_offset":7884,"ancestor_record_offset":2040,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":2040,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":6308,"parent_record_offset":5768,"ancestor_record_offset":2040,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":6376,"parent_record_offset":6344,"ancestor_record_offset":2040,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9892,"parent_record_offset":9732,"ancestor_record_offset":2040,"inlinee":26664,"source_line":1867,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":2040,"inlinee":26666,"source_line":1875,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3806,"record_offset":9732,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3806,"record_offset":4984,"parent_record_offset":4896,"ancestor_record_offset":2040,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3806,"record_offset":5044,"parent_record_offset":4984,"ancestor_record_offset":2040,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3806,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":2040,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3806,"record_offset":18828,"parent_record_offset":18512,"ancestor_record_offset":2040,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3806,"record_offset":7884,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3806,"record_offset":5768,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3806,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":2040,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3806,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":2040,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3806,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":2040,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3806,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":2040,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3806,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":2040,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3806,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":2040,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":11208,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":5868,"parent_record_offset":5832,"ancestor_record_offset":2040,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":5832,"parent_record_offset":5804,"ancestor_record_offset":2040,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":2040,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":5684,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":2040,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":5600,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3806,"record_offset":6920,"parent_record_offset":6632,"ancestor_record_offset":2040,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3806,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":2040,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3806,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":2040,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3806,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":2040,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15100,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":2040,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":2040,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":2040,"inlinee":13700,"source_line":266,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15156,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":2040,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":2040,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":2040,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":2040,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3806,"record_offset":16132,"parent_record_offset":16108,"ancestor_record_offset":2040,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":3806,"record_offset":5016,"parent_record_offset":4984,"ancestor_record_offset":2040,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3806,"record_offset":10048,"parent_record_offset":10020,"ancestor_record_offset":2040,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3806,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":2040,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":2040,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":2040,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":2040,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":2040,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":2040,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4256,"parent_record_offset":4084,"ancestor_record_offset":2040,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3806,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":2040,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3806,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":2040,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3806,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":2040,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3806,"record_offset":16760,"parent_record_offset":16732,"ancestor_record_offset":2040,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3806,"record_offset":11904,"parent_record_offset":11880,"ancestor_record_offset":2040,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3806,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":2040,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3806,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":2040,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3806,"record_offset":12404,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3806,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":2040,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":18548,"parent_record_offset":18512,"ancestor_record_offset":2040,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":2040,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3806,"record_offset":17836,"parent_record_offset":17800,"ancestor_record_offset":2040,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3806,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":2040,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3806,"record_offset":25688,"parent_record_offset":25664,"ancestor_record_offset":2040,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3806,"record_offset":25100,"parent_record_offset":25072,"ancestor_record_offset":2040,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3806,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":2040,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3806,"record_offset":12136,"parent_record_offset":12108,"ancestor_record_offset":2040,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3806,"record_offset":12108,"parent_record_offset":12080,"ancestor_record_offset":2040,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":21932,"parent_record_offset":21824,"ancestor_record_offset":2040,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":14308,"parent_record_offset":14176,"ancestor_record_offset":2040,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":15584,"parent_record_offset":15544,"ancestor_record_offset":2040,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3806,"record_offset":3928,"parent_record_offset":3884,"ancestor_record_offset":2040,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":2040,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":2040,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":2040,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3806,"record_offset":6096,"parent_record_offset":5896,"ancestor_record_offset":2040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3806,"record_offset":2748,"parent_record_offset":2720,"ancestor_record_offset":2040,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3806,"record_offset":4716,"parent_record_offset":3884,"ancestor_record_offset":2040,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3806,"record_offset":4320,"parent_record_offset":4032,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3806,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":2040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3806,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":2040,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3806,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":2040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3806,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2040,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3806,"record_offset":21736,"parent_record_offset":21680,"ancestor_record_offset":2040,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3806,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2040,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3806,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":2040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3806,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":2040,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3806,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":2040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$39::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3806,"record_offset":17800,"parent_record_offset":17768,"ancestor_record_offset":2040,"inlinee":32603,"source_line":266,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$39::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3806,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":2040,"inlinee":32605,"source_line":273,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$39::slice_contains::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3806,"record_offset":18076,"parent_record_offset":17952,"ancestor_record_offset":2040,"inlinee":32606,"source_line":278,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3806,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":2040,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3806,"record_offset":10584,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3806,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":2040,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3806,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":2040,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3806,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":2040,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3806,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":2040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2040,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":2040,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":2040,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":2040,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":17892,"parent_record_offset":17800,"ancestor_record_offset":2040,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":5448,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":3116,"parent_record_offset":3000,"ancestor_record_offset":2040,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3806,"record_offset":5512,"parent_record_offset":5448,"ancestor_record_offset":2040,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3806,"record_offset":17860,"parent_record_offset":17836,"ancestor_record_offset":2040,"inlinee":17422,"source_line":1911,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3806,"record_offset":2468,"parent_record_offset":2440,"ancestor_record_offset":2040,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3806,"record_offset":2828,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3806,"record_offset":2968,"parent_record_offset":2936,"ancestor_record_offset":2040,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3806,"record_offset":2936,"parent_record_offset":2900,"ancestor_record_offset":2040,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3806,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2040,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3806,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2040,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3806,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2040,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3806,"record_offset":3440,"parent_record_offset":2900,"ancestor_record_offset":2040,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2440,"parent_record_offset":2408,"ancestor_record_offset":2040,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":2040,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":2040,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2696,"parent_record_offset":2600,"ancestor_record_offset":2040,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":2040,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2380,"parent_record_offset":2344,"ancestor_record_offset":2040,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3806,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":2040,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3806,"record_offset":3000,"parent_record_offset":2968,"ancestor_record_offset":2040,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3806,"record_offset":3284,"parent_record_offset":3000,"ancestor_record_offset":2040,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3806,"record_offset":3256,"parent_record_offset":3000,"ancestor_record_offset":2040,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3806,"record_offset":3060,"parent_record_offset":3000,"ancestor_record_offset":2040,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3806,"record_offset":22792,"parent_record_offset":22764,"ancestor_record_offset":2040,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3806,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":2040,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3806,"record_offset":14804,"parent_record_offset":14716,"ancestor_record_offset":2040,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":14684,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":14716,"parent_record_offset":14684,"ancestor_record_offset":2040,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14116,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":2040,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14240,"parent_record_offset":14208,"ancestor_record_offset":2040,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14208,"parent_record_offset":14176,"ancestor_record_offset":2040,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":2040,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14152,"parent_record_offset":14048,"ancestor_record_offset":2040,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":2040,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3806,"record_offset":12048,"parent_record_offset":12020,"ancestor_record_offset":2040,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3806,"record_offset":12080,"parent_record_offset":12048,"ancestor_record_offset":2040,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3806,"record_offset":17420,"parent_record_offset":17232,"ancestor_record_offset":2040,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3806,"record_offset":12020,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":11880,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":16732,"parent_record_offset":16704,"ancestor_record_offset":2040,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":12432,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":16788,"parent_record_offset":16732,"ancestor_record_offset":2040,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":17512,"parent_record_offset":17180,"ancestor_record_offset":2040,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3806,"record_offset":11964,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3806,"record_offset":11852,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3806,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":2040,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":11444,"parent_record_offset":11412,"ancestor_record_offset":2040,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":11476,"parent_record_offset":11444,"ancestor_record_offset":2040,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":2040,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":12288,"parent_record_offset":12180,"ancestor_record_offset":2040,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":17120,"parent_record_offset":17096,"ancestor_record_offset":2040,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3806,"record_offset":11676,"parent_record_offset":11640,"ancestor_record_offset":2040,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":12228,"parent_record_offset":12204,"ancestor_record_offset":2040,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":2040,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":17612,"parent_record_offset":17180,"ancestor_record_offset":2040,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":12524,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":16704,"parent_record_offset":16644,"ancestor_record_offset":2040,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":17180,"parent_record_offset":17144,"ancestor_record_offset":2040,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11736,"parent_record_offset":11704,"ancestor_record_offset":2040,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11704,"parent_record_offset":11640,"ancestor_record_offset":2040,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11508,"parent_record_offset":11476,"ancestor_record_offset":2040,"inlinee":10143,"source_line":1263,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":16620,"parent_record_offset":16596,"ancestor_record_offset":2040,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":2040,"inlinee":10144,"source_line":1257,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25632,"parent_record_offset":25600,"ancestor_record_offset":2040,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":16676,"parent_record_offset":16644,"ancestor_record_offset":2040,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25544,"parent_record_offset":25520,"ancestor_record_offset":2040,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":2040,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25600,"parent_record_offset":25544,"ancestor_record_offset":2040,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25572,"parent_record_offset":25544,"ancestor_record_offset":2040,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":2040,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":11824,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25664,"parent_record_offset":25632,"ancestor_record_offset":2040,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":2040,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":2040,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3806,"record_offset":12180,"parent_record_offset":11768,"ancestor_record_offset":2040,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3806,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":2040,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":3812,"parent_record_offset":3776,"ancestor_record_offset":2040,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":25900,"parent_record_offset":25868,"ancestor_record_offset":2040,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":26416,"parent_record_offset":26156,"ancestor_record_offset":2040,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3806,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":2040,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":4032,"parent_record_offset":3884,"ancestor_record_offset":2040,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3806,"record_offset":4292,"parent_record_offset":4032,"ancestor_record_offset":2040,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3806,"record_offset":14272,"parent_record_offset":14240,"ancestor_record_offset":2040,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3806,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3806,"record_offset":3884,"parent_record_offset":3848,"ancestor_record_offset":2040,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3806,"record_offset":3848,"parent_record_offset":3812,"ancestor_record_offset":2040,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs
//     evidence={"module":3806,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":2040,"inlinee":121678,"source_line":85,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs
//     evidence={"module":3806,"record_offset":18576,"parent_record_offset":18548,"ancestor_record_offset":2040,"inlinee":121679,"source_line":100,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3806,"record_offset":27824,"rva":36515104,"va":5405224224}
// 编译实例 RVAs: 0x22d2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":28580,"parent_record_offset":27824,"ancestor_record_offset":27824,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":28012,"parent_record_offset":27824,"ancestor_record_offset":27824,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":28048,"parent_record_offset":27824,"ancestor_record_offset":27824,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":28204,"parent_record_offset":28072,"ancestor_record_offset":27824,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3806,"record_offset":28688,"parent_record_offset":28604,"ancestor_record_offset":27824,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":27824,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":28920,"parent_record_offset":28604,"ancestor_record_offset":27824,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":28136,"parent_record_offset":28104,"ancestor_record_offset":27824,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":28104,"parent_record_offset":28072,"ancestor_record_offset":27824,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":28072,"parent_record_offset":28048,"ancestor_record_offset":27824,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3806,"record_offset":28168,"parent_record_offset":28136,"ancestor_record_offset":27824,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE4E6
// 首条位置: {"module":1082,"inlinee":58598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB4E
// 首条位置: {"module":3806,"inlinee":121678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":15496,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":15584,"parent_record_offset":15544,"ancestor_record_offset":15496,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":15612,"parent_record_offset":15544,"ancestor_record_offset":15496,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":15496,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3806,"record_offset":15648,"parent_record_offset":15612,"ancestor_record_offset":15496,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EA4
// 首条位置: {"module":2611,"inlinee":97956}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB4F
// 首条位置: {"module":3806,"inlinee":121679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":18612,"parent_record_offset":18576,"ancestor_record_offset":18576,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":18684,"parent_record_offset":18640,"ancestor_record_offset":18576,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":18744,"parent_record_offset":18640,"ancestor_record_offset":18576,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":18640,"parent_record_offset":18612,"ancestor_record_offset":18576,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3806,"record_offset":18780,"parent_record_offset":18744,"ancestor_record_offset":18576,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::manage_autoclutter_entities::update_shutter
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB4D
// 首条位置: {"module":3806,"inlinee":121677}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3806,"record_offset":15044,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15100,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15156,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":14684,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14116,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14152,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14048,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":14048,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":15376,"parent_record_offset":15344,"ancestor_record_offset":14048,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3806,"record_offset":15344,"parent_record_offset":15316,"ancestor_record_offset":14048,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3806,"record_offset":15288,"parent_record_offset":15252,"ancestor_record_offset":14048,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3806,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":14048,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15220,"parent_record_offset":15188,"ancestor_record_offset":14048,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15252,"parent_record_offset":15220,"ancestor_record_offset":14048,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":14048,"inlinee":13700,"source_line":266,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3806,"record_offset":15188,"parent_record_offset":15156,"ancestor_record_offset":14048,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3806,"record_offset":14308,"parent_record_offset":14176,"ancestor_record_offset":14048,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":3806,"record_offset":14804,"parent_record_offset":14716,"ancestor_record_offset":14048,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3806,"record_offset":14716,"parent_record_offset":14684,"ancestor_record_offset":14048,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14240,"parent_record_offset":14208,"ancestor_record_offset":14048,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14208,"parent_record_offset":14176,"ancestor_record_offset":14048,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3806,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":14048,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03806.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3806,"record_offset":14272,"parent_record_offset":14240,"ancestor_record_offset":14048,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03806.jsonl.gz"}
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
// 完整结构及原始类型记录: manage_autoclutter_entities.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: manage_autoclutter_entities.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: manage_autoclutter_entities.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::AutoClutter>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities
// 信息 ID: 9c727d13dead85e4abf942c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] aed08be5933f0d0d1777a62b: void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >),void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)> > > @ 0x1cdf17b -> 0x22d04e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >),void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)> > @ 0x1cf0c66 -> 0x22d04e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$0
// 信息 ID: c9b9fb413abff0eafa0ef4d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities @ 0x22d0ee7 -> 0x22d2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities @ 0x22d1a14 -> 0x22d2d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$1
// 信息 ID: 8cf8dfbd3f641dfc6b6e5a86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$1 <- std::collections::hash::map::impl$87::from_iter<u64,ref$<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterObject>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$87::from_iter<u64,ref$<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterObject>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$1> >; module=4489, inline_record=1840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$3
// 信息 ID: e20e7080588b4b1956b5fea0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities; module=3806, inline_record=15496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$4
// 信息 ID: 3be3dde75c1df689fd821f02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$4 <- core::iter::adapters::map::impl$2::next<u64,bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<u64,bevy_ecs::query::iter::QueryIter<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref$<system_clutter::autoclutter::WindowShutter> > >,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env$4>; module=2611, inline_record=7672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure$5
// 信息 ID: e59000f425557836194de522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities; module=3806, inline_record=18576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::manage_autoclutter_entities::update_shutter
// 信息 ID: 17f44cc1a130fca5a7cfdc77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities; module=3806, inline_record=14048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
