// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/autoclutter/add_autoclutter_around_gates.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3366f1ad7d0c59e2a9e3d2655ab82a62db96d6da78be33057f051714f3fef9b6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_autoclutter_around_gates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3801
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: [{"name":"system-clutter","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3801,"record_offset":2040,"rva":36488080,"va":5405197200}
// 编译实例 RVAs: 0x22cc390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: add_autoclutter_around_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23264176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e850049dfe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405198699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf7440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_autoclutter_around_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e89faaa2ff","size":5,"soft_fail":false,"target_va":5399082048,"va":5405198748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6342152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e808c66000","size":5,"soft_fail":false,"target_va":5411541584,"va":5405199427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6342332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e8bcc66000","size":5,"soft_fail":false,"target_va":5411541776,"va":5405199439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e838fbffff","size":5,"soft_fail":false,"target_va":5405196128,"va":5405197347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6345931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e8cbd46000","size":5,"soft_fail":false,"target_va":5411543360,"va":5405197424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23812315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e825a794fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405197846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23807126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e86abb94fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405197761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23812757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e86ba594fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405199232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23812623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e8f1a594fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405197322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6609795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e883db6400","size":5,"soft_fail":false,"target_va":5411809152,"va":5405199352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20823969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e85f40c2fe","size":5,"soft_fail":false,"target_va":5384376048,"va":5405200012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6337684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e894b46000","size":5,"soft_fail":false,"target_va":5411537104,"va":5405199415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::autoclutter::AutoClutterState::add_region
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20876083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405197200,"raw_hex":"e8cd74c1fe","size":5,"soft_fail":false,"target_va":5384323024,"va":5405199102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3801,"record_offset":9296,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3801,"record_offset":5092,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3801,"record_offset":4684,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3801,"record_offset":7648,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3801,"record_offset":7428,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3801,"record_offset":9352,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3801,"record_offset":7400,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8016,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":7988,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3801,"record_offset":8236,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3801,"record_offset":7464,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":3700,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":2268,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":2724,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::AutoClutterRegion::centered
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3801,"record_offset":8432,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58574,"source_line":432,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3801,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":2040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3801,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":2040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3801,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":2040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3801,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":2040,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3801,"record_offset":4876,"parent_record_offset":4844,"ancestor_record_offset":2040,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3801,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":2040,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3801,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":2040,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3801,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":2040,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3801,"record_offset":7000,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3801,"record_offset":6216,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3801,"record_offset":6300,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3801,"record_offset":7196,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3801,"record_offset":6132,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3801,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":2040,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3801,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":2040,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3801,"record_offset":6392,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3801,"record_offset":5512,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3801,"record_offset":6888,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3801,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":2040,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3801,"record_offset":4792,"parent_record_offset":4708,"ancestor_record_offset":2040,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3801,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":2040,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3801,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":2040,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3801,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":2040,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3801,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":2040,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3801,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":2040,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3801,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":2040,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":2040,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":2040,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":5428,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":2040,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":5344,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3801,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":2040,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3801,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":2040,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3801,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":2040,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3801,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":2040,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3801,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":2040,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3801,"record_offset":7324,"parent_record_offset":7300,"ancestor_record_offset":2040,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3801,"record_offset":7788,"parent_record_offset":7680,"ancestor_record_offset":2040,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":2040,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2040,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2040,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":2040,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4180,"parent_record_offset":4008,"ancestor_record_offset":2040,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3801,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":2040,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3801,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":2040,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3801,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":2040,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3801,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":2040,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3801,"record_offset":7496,"parent_record_offset":7464,"ancestor_record_offset":2040,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3801,"record_offset":8172,"parent_record_offset":8040,"ancestor_record_offset":2040,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3801,"record_offset":8772,"parent_record_offset":8740,"ancestor_record_offset":2040,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3801,"record_offset":3852,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3801,"record_offset":7732,"parent_record_offset":7680,"ancestor_record_offset":2040,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3801,"record_offset":7680,"parent_record_offset":7648,"ancestor_record_offset":2040,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3801,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":2040,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3801,"record_offset":5768,"parent_record_offset":5588,"ancestor_record_offset":2040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3801,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2040,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3801,"record_offset":4640,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3801,"record_offset":7908,"parent_record_offset":7816,"ancestor_record_offset":2040,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3801,"record_offset":4244,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3801,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":2040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3801,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":2040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3801,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2040,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3801,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2040,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3801,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":2040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3801,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":2040,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3801,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":2040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3801,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":2040,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3801,"record_offset":7816,"parent_record_offset":7788,"ancestor_record_offset":2040,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3801,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":2040,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3801,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":2040,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3801,"record_offset":7852,"parent_record_offset":7816,"ancestor_record_offset":2040,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3801,"record_offset":5016,"parent_record_offset":4844,"ancestor_record_offset":2040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2040,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":2040,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2040,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":2040,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":5192,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":3040,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3801,"record_offset":5256,"parent_record_offset":5192,"ancestor_record_offset":2040,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3801,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2040,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3801,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2040,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3801,"record_offset":2892,"parent_record_offset":2860,"ancestor_record_offset":2040,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3801,"record_offset":2860,"parent_record_offset":2824,"ancestor_record_offset":2040,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3801,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2040,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3801,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3801,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2040,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3801,"record_offset":3364,"parent_record_offset":2824,"ancestor_record_offset":2040,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2364,"parent_record_offset":2332,"ancestor_record_offset":2040,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":2040,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2524,"parent_record_offset":2492,"ancestor_record_offset":2040,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2620,"parent_record_offset":2524,"ancestor_record_offset":2040,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":2040,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":2040,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3801,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":2040,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3801,"record_offset":2924,"parent_record_offset":2892,"ancestor_record_offset":2040,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3801,"record_offset":3208,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3801,"record_offset":3180,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3801,"record_offset":2984,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8104,"parent_record_offset":8072,"ancestor_record_offset":2040,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8072,"parent_record_offset":8040,"ancestor_record_offset":2040,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":2040,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8472,"parent_record_offset":8432,"ancestor_record_offset":2040,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3801,"record_offset":8560,"parent_record_offset":8432,"ancestor_record_offset":2040,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3801,"record_offset":8648,"parent_record_offset":8432,"ancestor_record_offset":2040,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3801,"record_offset":8740,"parent_record_offset":8432,"ancestor_record_offset":2040,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3801,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":2040,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":3736,"parent_record_offset":3700,"ancestor_record_offset":2040,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":9416,"parent_record_offset":9384,"ancestor_record_offset":2040,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":9932,"parent_record_offset":9672,"ancestor_record_offset":2040,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3801,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":2040,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":3956,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3801,"record_offset":4216,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3801,"record_offset":8136,"parent_record_offset":8104,"ancestor_record_offset":2040,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3801,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":2040,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3801,"record_offset":3808,"parent_record_offset":3772,"ancestor_record_offset":2040,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03801.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3801,"record_offset":3772,"parent_record_offset":3736,"ancestor_record_offset":2040,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03801.jsonl.gz"}
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
// 完整结构及原始类型记录: add_autoclutter_around_gates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_autoclutter_around_gates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_autoclutter_around_gates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates
// 信息 ID: 6703480931b91ec260151f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ccc988e87b54faad588b9e52: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x1cde469 -> 0x22cc390 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x1ceb205 -> 0x22cc390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
