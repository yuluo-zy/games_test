// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ivy/ivy_branch_thickness_animation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e1fd9e07f80f76c056567a67b977a0e6a936566ee843cd2625fb88795e773232
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_branch_thickness_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 524
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_branch_thickness_animation::ivy_branch_thickness_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":28568,"rva":10050176,"va":5378759296}
// 编译实例 RVAs: 0x995a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_branch_thickness_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32909408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e86028f601","size":5,"soft_fail":false,"target_va":5411669968,"va":5378760555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ivy_branch_thickness_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33087721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e8e9e0f801","size":5,"soft_fail":false,"target_va":5411847872,"va":5378760146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32780956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e89c32f401","size":5,"soft_fail":false,"target_va":5411541584,"va":5378760623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e815d0ffff","size":5,"soft_fail":false,"target_va":5378747120,"va":5378759382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32783903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e81f3ef401","size":5,"soft_fail":false,"target_va":5411543360,"va":5378759452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2625642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e86a102800","size":5,"soft_fail":false,"target_va":5381385536,"va":5378759889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2630828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e8ac242800","size":5,"soft_fail":false,"target_va":5381390640,"va":5378759807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2625972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e8b4112800","size":5,"soft_fail":false,"target_va":5381386480,"va":5378760503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2625352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e8480f2800","size":5,"soft_fail":false,"target_va":5381384704,"va":5378759347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32776488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378759296,"raw_hex":"e82821f401","size":5,"soft_fail":false,"target_va":5411537104,"va":5378760611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32428,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":24644,"source_line":1237,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":524,"record_offset":31180,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":524,"record_offset":32960,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":33140,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":524,"record_offset":32372,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":26766,"source_line":57,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":524,"record_offset":31240,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":29384,"source_line":176,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":524,"record_offset":32820,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":30212,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":28788,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":29236,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":31648,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":14696,"source_line":2586,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":524,"record_offset":32400,"parent_record_offset":28568,"ancestor_record_offset":28568,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32452,"parent_record_offset":32428,"ancestor_record_offset":28568,"inlinee":14988,"source_line":1304,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32672,"parent_record_offset":32452,"ancestor_record_offset":28568,"inlinee":11310,"source_line":196,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32568,"parent_record_offset":32540,"ancestor_record_offset":28568,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32696,"parent_record_offset":32672,"ancestor_record_offset":28568,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32480,"parent_record_offset":32452,"ancestor_record_offset":28568,"inlinee":11309,"source_line":1358,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32516,"parent_record_offset":32480,"ancestor_record_offset":28568,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":524,"record_offset":32540,"parent_record_offset":32516,"ancestor_record_offset":28568,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::iter_mut::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":524,"record_offset":32984,"parent_record_offset":32960,"ancestor_record_offset":28568,"inlinee":24646,"source_line":157,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":32600,"parent_record_offset":32568,"ancestor_record_offset":28568,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":32720,"parent_record_offset":32696,"ancestor_record_offset":28568,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":28568,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":32768,"parent_record_offset":32744,"ancestor_record_offset":28568,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":524,"record_offset":32744,"parent_record_offset":32720,"ancestor_record_offset":28568,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":524,"record_offset":31208,"parent_record_offset":31180,"ancestor_record_offset":28568,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30532,"parent_record_offset":30504,"ancestor_record_offset":28568,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":28568,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30560,"parent_record_offset":30532,"ancestor_record_offset":28568,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30480,"parent_record_offset":30452,"ancestor_record_offset":28568,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30504,"parent_record_offset":30480,"ancestor_record_offset":28568,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30676,"parent_record_offset":30504,"ancestor_record_offset":28568,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":524,"record_offset":30788,"parent_record_offset":30764,"ancestor_record_offset":28568,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":31524,"parent_record_offset":31496,"ancestor_record_offset":28568,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":31584,"parent_record_offset":31552,"ancestor_record_offset":28568,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":524,"record_offset":30608,"parent_record_offset":30584,"ancestor_record_offset":28568,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":524,"record_offset":28936,"parent_record_offset":28904,"ancestor_record_offset":28568,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":31976,"parent_record_offset":31944,"ancestor_record_offset":28568,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":31944,"parent_record_offset":31912,"ancestor_record_offset":28568,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":32928,"parent_record_offset":32820,"ancestor_record_offset":28568,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":32872,"parent_record_offset":32820,"ancestor_record_offset":28568,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":524,"record_offset":32844,"parent_record_offset":32820,"ancestor_record_offset":28568,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":524,"record_offset":30348,"parent_record_offset":30308,"ancestor_record_offset":28568,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":29184,"parent_record_offset":29156,"ancestor_record_offset":28568,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":31136,"parent_record_offset":30308,"ancestor_record_offset":28568,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":30740,"parent_record_offset":30452,"ancestor_record_offset":28568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":32236,"parent_record_offset":32212,"ancestor_record_offset":28568,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":29648,"parent_record_offset":29624,"ancestor_record_offset":28568,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":29624,"parent_record_offset":29600,"ancestor_record_offset":28568,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":29832,"parent_record_offset":29804,"ancestor_record_offset":28568,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":30632,"parent_record_offset":30608,"ancestor_record_offset":28568,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":29576,"parent_record_offset":29552,"ancestor_record_offset":28568,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":29600,"parent_record_offset":29576,"ancestor_record_offset":28568,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":29904,"parent_record_offset":29876,"ancestor_record_offset":28568,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":29552,"parent_record_offset":29436,"ancestor_record_offset":28568,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":33012,"parent_record_offset":32984,"ancestor_record_offset":28568,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":28904,"parent_record_offset":28876,"ancestor_record_offset":28568,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":29264,"parent_record_offset":29236,"ancestor_record_offset":28568,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":29404,"parent_record_offset":29372,"ancestor_record_offset":28568,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":29372,"parent_record_offset":29336,"ancestor_record_offset":28568,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":29312,"parent_record_offset":29288,"ancestor_record_offset":28568,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":29336,"parent_record_offset":29312,"ancestor_record_offset":28568,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":29288,"parent_record_offset":29264,"ancestor_record_offset":28568,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":29876,"parent_record_offset":29336,"ancestor_record_offset":28568,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":28876,"parent_record_offset":28844,"ancestor_record_offset":28568,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":28844,"parent_record_offset":28820,"ancestor_record_offset":28568,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":29036,"parent_record_offset":29004,"ancestor_record_offset":28568,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":29132,"parent_record_offset":29036,"ancestor_record_offset":28568,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":29004,"parent_record_offset":28980,"ancestor_record_offset":28568,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":28820,"parent_record_offset":28788,"ancestor_record_offset":28568,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":28568,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":29436,"parent_record_offset":29404,"ancestor_record_offset":28568,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":29720,"parent_record_offset":29436,"ancestor_record_offset":28568,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":29692,"parent_record_offset":29436,"ancestor_record_offset":28568,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":29496,"parent_record_offset":29436,"ancestor_record_offset":28568,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":31868,"parent_record_offset":31824,"ancestor_record_offset":28568,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":31912,"parent_record_offset":31868,"ancestor_record_offset":28568,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":32020,"parent_record_offset":31824,"ancestor_record_offset":28568,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":31824,"parent_record_offset":31772,"ancestor_record_offset":28568,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":31496,"parent_record_offset":31440,"ancestor_record_offset":28568,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":31552,"parent_record_offset":31496,"ancestor_record_offset":28568,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":32112,"parent_record_offset":31772,"ancestor_record_offset":28568,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":524,"record_offset":31324,"parent_record_offset":31296,"ancestor_record_offset":28568,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":524,"record_offset":31704,"parent_record_offset":31676,"ancestor_record_offset":28568,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":32212,"parent_record_offset":31772,"ancestor_record_offset":28568,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31440,"parent_record_offset":31380,"ancestor_record_offset":28568,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31772,"parent_record_offset":31732,"ancestor_record_offset":28568,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31352,"parent_record_offset":31324,"ancestor_record_offset":28568,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31412,"parent_record_offset":31380,"ancestor_record_offset":28568,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31380,"parent_record_offset":31352,"ancestor_record_offset":28568,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":524,"record_offset":31732,"parent_record_offset":31704,"ancestor_record_offset":28568,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":30244,"parent_record_offset":30212,"ancestor_record_offset":28568,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":33204,"parent_record_offset":33172,"ancestor_record_offset":28568,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":33704,"parent_record_offset":33444,"ancestor_record_offset":28568,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":30376,"parent_record_offset":30348,"ancestor_record_offset":28568,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":30452,"parent_record_offset":30308,"ancestor_record_offset":28568,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":30712,"parent_record_offset":30452,"ancestor_record_offset":28568,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":31296,"parent_record_offset":31268,"ancestor_record_offset":28568,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":31268,"parent_record_offset":31240,"ancestor_record_offset":28568,"inlinee":13598,"source_line":473,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":31676,"parent_record_offset":31648,"ancestor_record_offset":28568,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":30404,"parent_record_offset":30376,"ancestor_record_offset":28568,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":30308,"parent_record_offset":30276,"ancestor_record_offset":28568,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":30276,"parent_record_offset":30244,"ancestor_record_offset":28568,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00524.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_branch_thickness_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_branch_thickness_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_branch_thickness_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_branch_thickness_animation::ivy_branch_thickness_animation
// 信息 ID: 738d843f4a37b469e0e8eb21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 722bfa9098a4fce6ec596e11: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)> > > @ 0x14ea2be -> 0x995a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)> > @ 0x1661550 -> 0x995a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
