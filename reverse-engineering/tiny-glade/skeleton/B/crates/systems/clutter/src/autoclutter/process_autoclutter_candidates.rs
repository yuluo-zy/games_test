// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/clutter/src/autoclutter/process_autoclutter_candidates.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b5dd0d9ad8fb0a28f4644854d6cf38ea75f5cf195c7a8a40f66c52c098faf3d7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_autoclutter_candidates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2611, 2616, 2630, 3804, 3820, 4493, 4494, 4495
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
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
// 函数符号: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3804,"record_offset":2040,"rva":36495968,"va":5405205088}
// 编译实例 RVAs: 0x22ce260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_autoclutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30124400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8905634fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405205883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: process_autoclutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30125197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8735334fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405206696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_autoclutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24582757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e89be588fe","size":5,"soft_fail":false,"target_va":5380624688,"va":5405207440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cea460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_autoclutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6176963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e83dbfa1ff","size":5,"soft_fail":false,"target_va":5399028832,"va":5405205790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141cf9b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_autoclutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6114865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8cfb1a2ff","size":5,"soft_fail":false,"target_va":5399092048,"va":5405206908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-29626709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8abee3bfe","size":5,"soft_fail":false,"target_va":5375579472,"va":5405206176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6333999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e82fa66000","size":5,"soft_fail":false,"target_va":5411540963,"va":5405206959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6334654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8bea86000","size":5,"soft_fail":false,"target_va":5411541584,"va":5405206925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6335354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e87aab6000","size":5,"soft_fail":false,"target_va":5411542296,"va":5405206937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6137429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8ab59a2ff","size":5,"soft_fail":false,"target_va":5399069552,"va":5405206976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e880fbffff","size":5,"soft_fail":false,"target_va":5405204016,"va":5405205163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6338115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e843b66000","size":5,"soft_fail":false,"target_va":5411543360,"va":5405205240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24951353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8c74583fe","size":5,"soft_fail":false,"target_va":5380255296,"va":5405206644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24951381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8ab4583fe","size":5,"soft_fail":false,"target_va":5380255248,"va":5405206624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24951222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e84a4683fe","size":5,"soft_fail":false,"target_va":5380255440,"va":5405206657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23820141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8938894fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405205672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23814952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8d89c94fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405205587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23820318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8e28794fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405206793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23820439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8698794fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405205138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6602312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e848be6400","size":5,"soft_fail":false,"target_va":5411809152,"va":5405206835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20831078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e89a24c2fe","size":5,"soft_fail":false,"target_va":5384376048,"va":5405207121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6330218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e86a976000","size":5,"soft_fail":false,"target_va":5411537104,"va":5405206881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23290064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405205088,"raw_hex":"e8309f9cfe","size":5,"soft_fail":false,"target_va":5381916032,"va":5405206091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3804,"record_offset":4912,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5768,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5084,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3804,"record_offset":4684,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3804,"record_offset":5444,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3804,"record_offset":4828,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3804,"record_offset":6884,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3804,"record_offset":4744,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":6052,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":3700,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":2268,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":2724,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::ClutterType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3804,"record_offset":5740,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":58563,"source_line":82,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::autoclutter::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/mod.rs
//     evidence={"module":3804,"record_offset":6268,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":121663,"source_line":93,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3804,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":2040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3804,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":2040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3804,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":2040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3804,"record_offset":5256,"parent_record_offset":5216,"ancestor_record_offset":2040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3804,"record_offset":7048,"parent_record_offset":6956,"ancestor_record_offset":2040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5848,"parent_record_offset":5816,"ancestor_record_offset":2040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5148,"parent_record_offset":5116,"ancestor_record_offset":2040,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":6984,"parent_record_offset":6956,"ancestor_record_offset":2040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":2040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":2040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":2040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":2040,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":5180,"parent_record_offset":5148,"ancestor_record_offset":2040,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3804,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":2040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3804,"record_offset":4944,"parent_record_offset":4912,"ancestor_record_offset":2040,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3804,"record_offset":6356,"parent_record_offset":6308,"ancestor_record_offset":2040,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":3804,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":2040,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5816,"parent_record_offset":5768,"ancestor_record_offset":2040,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5116,"parent_record_offset":5084,"ancestor_record_offset":2040,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":2040,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":2040,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3804,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":2040,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":2040,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2040,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2040,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":3984,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":2040,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4180,"parent_record_offset":4008,"ancestor_record_offset":2040,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3804,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":2040,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3804,"record_offset":5016,"parent_record_offset":4976,"ancestor_record_offset":2040,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3804,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":2040,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3804,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":2040,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3804,"record_offset":7016,"parent_record_offset":6984,"ancestor_record_offset":2040,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3804,"record_offset":2424,"parent_record_offset":2392,"ancestor_record_offset":2040,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3804,"record_offset":3852,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3804,"record_offset":5560,"parent_record_offset":5532,"ancestor_record_offset":2040,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3804,"record_offset":5532,"parent_record_offset":5444,"ancestor_record_offset":2040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3804,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2040,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3804,"record_offset":4640,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3804,"record_offset":4244,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3804,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":2040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3804,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":2040,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3804,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2040,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3804,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2040,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3804,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":2040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3804,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":2040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3804,"record_offset":5648,"parent_record_offset":5616,"ancestor_record_offset":2040,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3804,"record_offset":5680,"parent_record_offset":5648,"ancestor_record_offset":2040,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3804,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":2040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2040,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":2040,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2040,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":2040,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":5472,"parent_record_offset":5444,"ancestor_record_offset":2040,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":3040,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3804,"record_offset":6148,"parent_record_offset":6084,"ancestor_record_offset":2040,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":3804,"record_offset":4976,"parent_record_offset":4944,"ancestor_record_offset":2040,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3804,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2040,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3804,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2040,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3804,"record_offset":2892,"parent_record_offset":2860,"ancestor_record_offset":2040,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3804,"record_offset":2860,"parent_record_offset":2824,"ancestor_record_offset":2040,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3804,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2040,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3804,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3804,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2040,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3804,"record_offset":3364,"parent_record_offset":2824,"ancestor_record_offset":2040,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2364,"parent_record_offset":2332,"ancestor_record_offset":2040,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":2040,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2524,"parent_record_offset":2492,"ancestor_record_offset":2040,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2620,"parent_record_offset":2524,"ancestor_record_offset":2040,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":2040,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":2040,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3804,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":2040,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3804,"record_offset":2924,"parent_record_offset":2892,"ancestor_record_offset":2040,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3804,"record_offset":3208,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3804,"record_offset":3180,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3804,"record_offset":2984,"parent_record_offset":2924,"ancestor_record_offset":2040,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3804,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":2040,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3804,"record_offset":6780,"parent_record_offset":6744,"ancestor_record_offset":2040,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3804,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":2040,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3804,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":2040,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3804,"record_offset":6836,"parent_record_offset":6744,"ancestor_record_offset":2040,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$92::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3804,"record_offset":6308,"parent_record_offset":6268,"ancestor_record_offset":2040,"inlinee":121665,"source_line":396,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::impl$94::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":3804,"record_offset":6448,"parent_record_offset":6268,"ancestor_record_offset":2040,"inlinee":121667,"source_line":424,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":3736,"parent_record_offset":3700,"ancestor_record_offset":2040,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":7416,"parent_record_offset":7384,"ancestor_record_offset":2040,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":7916,"parent_record_offset":7656,"ancestor_record_offset":2040,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3804,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":2040,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":3956,"parent_record_offset":3808,"ancestor_record_offset":2040,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3804,"record_offset":4216,"parent_record_offset":3956,"ancestor_record_offset":2040,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3804,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":2040,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3804,"record_offset":3808,"parent_record_offset":3772,"ancestor_record_offset":2040,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3804,"record_offset":3772,"parent_record_offset":3736,"ancestor_record_offset":2040,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/process_autoclutter_candidates.rs
//     evidence={"module":3804,"record_offset":5588,"parent_record_offset":5560,"ancestor_record_offset":2040,"inlinee":97980,"source_line":82,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-clutter","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/autoclutter/process_autoclutter_candidates.rs
//     evidence={"module":3804,"record_offset":6236,"parent_record_offset":6052,"ancestor_record_offset":2040,"inlinee":121661,"source_line":98,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EAF
// 首条位置: {"module":2611,"inlinee":97967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":2611,"record_offset":31476,"parent_record_offset":31444,"ancestor_record_offset":31444,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2611,"record_offset":32116,"parent_record_offset":32092,"ancestor_record_offset":31444,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2611,"record_offset":32092,"parent_record_offset":32068,"ancestor_record_offset":31444,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2611,"record_offset":31744,"parent_record_offset":31720,"ancestor_record_offset":31444,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2611,"record_offset":31828,"parent_record_offset":31804,"ancestor_record_offset":31444,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2611,"record_offset":31768,"parent_record_offset":31744,"ancestor_record_offset":31444,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2611,"record_offset":32160,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2611,"record_offset":31976,"parent_record_offset":31948,"ancestor_record_offset":31444,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2611,"record_offset":31948,"parent_record_offset":31920,"ancestor_record_offset":31444,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2611,"record_offset":31888,"parent_record_offset":31860,"ancestor_record_offset":31444,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2611,"record_offset":31920,"parent_record_offset":31888,"ancestor_record_offset":31444,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2611,"record_offset":31860,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2611,"record_offset":31720,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2611,"record_offset":32188,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2611,"record_offset":31804,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2611,"record_offset":32340,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2611,"record_offset":32068,"parent_record_offset":32044,"ancestor_record_offset":31444,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2611,"record_offset":31540,"parent_record_offset":31508,"ancestor_record_offset":31444,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2611,"record_offset":31572,"parent_record_offset":31540,"ancestor_record_offset":31444,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2611,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":31444,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2611,"record_offset":32380,"parent_record_offset":31572,"ancestor_record_offset":31444,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":32280,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":31636,"parent_record_offset":31608,"ancestor_record_offset":31444,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":31608,"parent_record_offset":31572,"ancestor_record_offset":31444,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":31664,"parent_record_offset":31636,"ancestor_record_offset":31444,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":32312,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2611,"record_offset":32020,"parent_record_offset":31664,"ancestor_record_offset":31444,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":2611,"record_offset":31508,"parent_record_offset":31476,"ancestor_record_offset":31444,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/02611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EE8
// 首条位置: {"module":2630,"inlinee":98024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::reverse
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2630,"record_offset":796,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":10270,"source_line":547,"archive":"_pdb/B/modules/02630.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$65::cmp
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2630,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":744,"inlinee":10268,"source_line":1836,"archive":"_pdb/B/modules/02630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17EBC
// 首条位置: {"module":2616,"inlinee":97980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3804,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":5588,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3804,"record_offset":5648,"parent_record_offset":5616,"ancestor_record_offset":5588,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3804,"record_offset":5680,"parent_record_offset":5648,"ancestor_record_offset":5588,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DB3D
// 首条位置: {"module":3804,"inlinee":121661}
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
// 完整结构及原始类型记录: process_autoclutter_candidates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterCandidate> >,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterCandidate> >,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>,alloc::vec::Vec<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>", "alloc::vec::Vec<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::choose_pivot<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::choose_pivot", "arguments": ["system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: process_autoclutter_candidates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: process_autoclutter_candidates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<array$<u32,2> >,ref$<array$<u32,2> >)
// 候选参数类型: ["ref$<array$<u32,2> >", "ref$<array$<u32,2> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates
// 信息 ID: be4a6a7d3cbf24a167b1f6bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b878bf64e2650ec6b0f179c3: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > > @ 0x1ce1a77 -> 0x22ce260 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x1ceff75 -> 0x22ce260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$0
// 信息 ID: 24bbb03970fa3966e6bea284
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterCandidate> >,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterCandidate> >,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0> >; module=2611, inline_record=31444; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterCandidate> >,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$0> >; module=2611, inline_record=33196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$1
// 信息 ID: cd906b02ba19e4960c811c91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::stable::driftsort_main<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1>,alloc::vec::Vec<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::stable::drift::sort<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1 <- core::slice::sort::stable::merge::merge<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 10 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=744; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=856; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=1140; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=1260; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=1488; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=5140; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=5580; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::autoclutter::process_autoclutter_candidates::OrderedCandidate,system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure_env$1> >; module=2630, inline_record=34560; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$2
// 信息 ID: 854c2cf120967fd24ae2307a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates; module=3804, inline_record=5588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates::closure$3
// 信息 ID: 48079348b663f8fb4213ba75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::autoclutter::process_autoclutter_candidates::process_autoclutter_candidates; module=3804, inline_record=6236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
