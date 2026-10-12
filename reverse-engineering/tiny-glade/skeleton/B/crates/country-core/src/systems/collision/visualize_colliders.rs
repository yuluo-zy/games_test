// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/collision/visualize_colliders.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3884d5ae0217ae8deef54c0d23a561285b0bb0487a8aaf41c90f8e2a1a2848f6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visualize_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1585, 1588, 1737
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nalgebra; // namespace candidate
//   package identities: [{"name":"nalgebra","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.33.0"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rapier3d; // namespace candidate
//   package identities: [{"name":"rapier3d","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050","version":"0.22.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1588,"record_offset":17780,"rva":20079472,"va":5388788592}
// 编译实例 RVAs: 0x1326370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22880235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8eb1f5d01","size":5,"soft_fail":false,"target_va":5411669968,"va":5388789728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22751796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8342a5b01","size":5,"soft_fail":false,"target_va":5411541584,"va":5388789783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8d8faffff","size":5,"soft_fail":false,"target_va":5388787344,"va":5388788659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t22754619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e83b355b01","size":5,"soft_fail":false,"target_va":5411543360,"va":5388788736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8db020000","size":5,"soft_fail":false,"target_va":5388790064,"va":5388789328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7403626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e896078fff","size":5,"soft_fail":false,"target_va":5381385536,"va":5388789157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7398440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8d81b8fff","size":5,"soft_fail":false,"target_va":5381390640,"va":5388789075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7403214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e832098fff","size":5,"soft_fail":false,"target_va":5381386480,"va":5388789689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7403937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e85f068fff","size":5,"soft_fail":false,"target_va":5381384704,"va":5388788636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::begin_panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22748913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8f11e5b01","size":5,"soft_fail":false,"target_va":5411538725,"va":5388789807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e8ff890000","size":5,"soft_fail":false,"target_va":5388825296,"va":5388789964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22747278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388788592,"raw_hex":"e88e185b01","size":5,"soft_fail":false,"target_va":5411537104,"va":5388789821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1588,"record_offset":20368,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":21476,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1588,"record_offset":21448,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":27839,"source_line":1569,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1588,"record_offset":21392,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":10926,"source_line":261,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":19408,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":17976,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":18432,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider::Collider::shape
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider.rs
//     evidence={"module":1588,"record_offset":20864,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":35133,"source_line":431,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider_set::ColliderSet::iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider_set.rs
//     evidence={"module":1588,"record_offset":20576,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":72454,"source_line":42,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1588,"record_offset":21184,"parent_record_offset":17780,"ancestor_record_offset":17780,"inlinee":10929,"source_line":863,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":20704,"parent_record_offset":20680,"ancestor_record_offset":17780,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":20752,"parent_record_offset":20728,"ancestor_record_offset":17780,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":20728,"parent_record_offset":20704,"ancestor_record_offset":17780,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1588,"record_offset":20912,"parent_record_offset":20888,"ancestor_record_offset":17780,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1588,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":17780,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1588,"record_offset":20656,"parent_record_offset":20624,"ancestor_record_offset":17780,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1588,"record_offset":20624,"parent_record_offset":20600,"ancestor_record_offset":17780,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19744,"parent_record_offset":19708,"ancestor_record_offset":17780,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19800,"parent_record_offset":19776,"ancestor_record_offset":17780,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19776,"parent_record_offset":19744,"ancestor_record_offset":17780,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19680,"parent_record_offset":19648,"ancestor_record_offset":17780,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19708,"parent_record_offset":19680,"ancestor_record_offset":17780,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":19892,"parent_record_offset":19708,"ancestor_record_offset":17780,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1588,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":17780,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1588,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":17780,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1588,"record_offset":19824,"parent_record_offset":19800,"ancestor_record_offset":17780,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1588,"record_offset":18132,"parent_record_offset":18100,"ancestor_record_offset":17780,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1588,"record_offset":19544,"parent_record_offset":19504,"ancestor_record_offset":17780,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1588,"record_offset":21320,"parent_record_offset":21220,"ancestor_record_offset":17780,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1588,"record_offset":18380,"parent_record_offset":18352,"ancestor_record_offset":17780,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":19956,"parent_record_offset":19648,"ancestor_record_offset":17780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1588,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":17780,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1588,"record_offset":20528,"parent_record_offset":20504,"ancestor_record_offset":17780,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1588,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":17780,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1588,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":17780,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1588,"record_offset":19028,"parent_record_offset":19000,"ancestor_record_offset":17780,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":17780,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1588,"record_offset":21256,"parent_record_offset":21220,"ancestor_record_offset":17780,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1588,"record_offset":21284,"parent_record_offset":21256,"ancestor_record_offset":17780,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1588,"record_offset":20796,"parent_record_offset":20624,"ancestor_record_offset":17780,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":17780,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":20504,"parent_record_offset":20480,"ancestor_record_offset":17780,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":18796,"parent_record_offset":18772,"ancestor_record_offset":17780,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":19100,"parent_record_offset":19072,"ancestor_record_offset":17780,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":17780,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":18748,"parent_record_offset":18632,"ancestor_record_offset":17780,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":20480,"parent_record_offset":20416,"ancestor_record_offset":17780,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1588,"record_offset":18100,"parent_record_offset":18072,"ancestor_record_offset":17780,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1588,"record_offset":18460,"parent_record_offset":18432,"ancestor_record_offset":17780,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1588,"record_offset":18600,"parent_record_offset":18568,"ancestor_record_offset":17780,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1588,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":17780,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1588,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":17780,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1588,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":17780,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1588,"record_offset":18484,"parent_record_offset":18460,"ancestor_record_offset":17780,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1588,"record_offset":19072,"parent_record_offset":18532,"ancestor_record_offset":17780,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18072,"parent_record_offset":18040,"ancestor_record_offset":17780,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18040,"parent_record_offset":18012,"ancestor_record_offset":17780,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18232,"parent_record_offset":18200,"ancestor_record_offset":17780,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18328,"parent_record_offset":18232,"ancestor_record_offset":17780,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":17780,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18012,"parent_record_offset":17976,"ancestor_record_offset":17780,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1588,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":17780,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1588,"record_offset":18632,"parent_record_offset":18600,"ancestor_record_offset":17780,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1588,"record_offset":18916,"parent_record_offset":18632,"ancestor_record_offset":17780,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1588,"record_offset":18888,"parent_record_offset":18632,"ancestor_record_offset":17780,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1588,"record_offset":18692,"parent_record_offset":18632,"ancestor_record_offset":17780,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::shape::shared_shape::impl$1::as_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1588,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":17780,"inlinee":35135,"source_line":32,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":19440,"parent_record_offset":19408,"ancestor_record_offset":17780,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":21540,"parent_record_offset":21508,"ancestor_record_offset":17780,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":22064,"parent_record_offset":21784,"ancestor_record_offset":17780,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1588,"record_offset":19572,"parent_record_offset":19544,"ancestor_record_offset":17780,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":19648,"parent_record_offset":19504,"ancestor_record_offset":17780,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1588,"record_offset":19928,"parent_record_offset":19648,"ancestor_record_offset":17780,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::Arena::iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1588,"record_offset":20600,"parent_record_offset":20576,"ancestor_record_offset":17780,"inlinee":72455,"source_line":701,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::impl$11::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1588,"record_offset":20972,"parent_record_offset":20948,"ancestor_record_offset":17780,"inlinee":29848,"source_line":937,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::KeyData::from_ffi
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":1588,"record_offset":21416,"parent_record_offset":21392,"ancestor_record_offset":17780,"inlinee":10927,"source_line":304,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1588,"record_offset":21220,"parent_record_offset":21184,"ancestor_record_offset":17780,"inlinee":10931,"source_line":538,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1588,"record_offset":21352,"parent_record_offset":21320,"ancestor_record_offset":17780,"inlinee":10933,"source_line":542,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1588,"record_offset":19600,"parent_record_offset":19572,"ancestor_record_offset":17780,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1588,"record_offset":19504,"parent_record_offset":19472,"ancestor_record_offset":17780,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1588,"record_offset":19472,"parent_record_offset":19440,"ancestor_record_offset":17780,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1588,"record_offset":22656,"rva":20080944,"va":5388790064}
// 编译实例 RVAs: 0x1326930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_ball
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3713128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e868a83800","size":5,"soft_fail":false,"target_va":5392504768,"va":5388791635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_compound
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3714703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e88fae3800","size":5,"soft_fail":false,"target_va":5392504896,"va":5388790188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_convex_polyhedron
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3714085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e825ac3800","size":5,"soft_fail":false,"target_va":5392505024,"va":5388790934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3713605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e845aa3800","size":5,"soft_fail":false,"target_va":5392504832,"va":5388791222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_trimesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3714267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e8dbac3800","size":5,"soft_fail":false,"target_va":5392504960,"va":5388790688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13709970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e86ecd2eff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388791453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13709671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e899ce2eff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388791170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412ff120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-162533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e81b85fdff","size":5,"soft_fail":false,"target_va":5388628256,"va":5388790784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130c4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-108605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e8c357feff","size":5,"soft_fail":false,"target_va":5388682464,"va":5388791064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22749277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e85d205b01","size":5,"soft_fail":false,"target_va":5411540992,"va":5388791710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22750091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e88b235b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5388791808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e8a8fdffff","size":5,"soft_fail":false,"target_va":5388790064,"va":5388790659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8703299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e8bd327bff","size":5,"soft_fail":false,"target_va":5380088032,"va":5388791326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8527042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e83ee37dff","size":5,"soft_fail":false,"target_va":5380263792,"va":5388790829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8538735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e891b57dff","size":5,"soft_fail":false,"target_va":5380252960,"va":5388791690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::transformation::to_trimesh::convex_polyhedron_to_trimesh::_$LT$impl$u20$parry3d..shape..convex_polyhedron..ConvexPolyhedron$GT$::to_trimesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3730496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388790064,"raw_hex":"e840ec3800","size":5,"soft_fail":false,"target_va":5392521456,"va":5388790955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*296(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5388790064,"raw_hex":"ff9328010000","size":6,"soft_fail":false,"va":5388791720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25568,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1588,"record_offset":25416,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1588,"record_offset":25504,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1588,"record_offset":26092,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1588,"record_offset":24528,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":24864,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1588,"record_offset":24444,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1588,"record_offset":22864,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$38::to_glam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1588,"record_offset":25332,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":27638,"source_line":2436,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1588,"record_offset":24836,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":28128,"source_line":391,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1588,"record_offset":24612,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":28129,"source_line":369,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1588,"record_offset":26064,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":32885,"source_line":94,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] nalgebra::geometry::isometry_ops::impl$2::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1588,"record_offset":22944,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":13753,"source_line":78,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_direct] parry3d::shape::shared_shape::impl$0::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1588,"record_offset":24380,"parent_record_offset":22656,"ancestor_record_offset":22656,"inlinee":13783,"source_line":26,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":22656,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25648,"parent_record_offset":25624,"ancestor_record_offset":22656,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25048,"parent_record_offset":25024,"ancestor_record_offset":22656,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25600,"parent_record_offset":25568,"ancestor_record_offset":22656,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1588,"record_offset":25024,"parent_record_offset":24936,"ancestor_record_offset":22656,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":24964,"parent_record_offset":24936,"ancestor_record_offset":22656,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":24936,"parent_record_offset":24912,"ancestor_record_offset":22656,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1588,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":22656,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1588,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":22656,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1588,"record_offset":24552,"parent_record_offset":24528,"ancestor_record_offset":22656,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1588,"record_offset":25772,"parent_record_offset":25748,"ancestor_record_offset":22656,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1588,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":22656,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$18::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1588,"record_offset":23548,"parent_record_offset":23476,"ancestor_record_offset":22656,"inlinee":11764,"source_line":336,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1588,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":22656,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1588,"record_offset":25444,"parent_record_offset":25416,"ancestor_record_offset":22656,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1588,"record_offset":24992,"parent_record_offset":24964,"ancestor_record_offset":22656,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1588,"record_offset":23172,"parent_record_offset":23080,"ancestor_record_offset":22656,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1588,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":22656,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1588,"record_offset":24080,"parent_record_offset":24056,"ancestor_record_offset":22656,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1588,"record_offset":23284,"parent_record_offset":23080,"ancestor_record_offset":22656,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":24888,"parent_record_offset":24864,"ancestor_record_offset":22656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1588,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":22656,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1588,"record_offset":22908,"parent_record_offset":22864,"ancestor_record_offset":22656,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1588,"record_offset":25676,"parent_record_offset":25648,"ancestor_record_offset":22656,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1588,"record_offset":24468,"parent_record_offset":24444,"ancestor_record_offset":22656,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1588,"record_offset":25356,"parent_record_offset":25332,"ancestor_record_offset":22656,"inlinee":27639,"source_line":83,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1588,"record_offset":25380,"parent_record_offset":25356,"ancestor_record_offset":22656,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":24056,"parent_record_offset":24032,"ancestor_record_offset":22656,"inlinee":13776,"source_line":183,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":24304,"parent_record_offset":24276,"ancestor_record_offset":22656,"inlinee":13779,"source_line":212,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1588,"record_offset":23476,"parent_record_offset":23356,"ancestor_record_offset":22656,"inlinee":13761,"source_line":2054,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":24032,"parent_record_offset":24008,"ancestor_record_offset":22656,"inlinee":13775,"source_line":272,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$22::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":24276,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":13778,"source_line":290,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":24008,"parent_record_offset":23356,"ancestor_record_offset":22656,"inlinee":13773,"source_line":308,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1588,"record_offset":23404,"parent_record_offset":23356,"ancestor_record_offset":22656,"inlinee":13771,"source_line":467,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::unit::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/unit.rs
//     evidence={"module":1588,"record_offset":24212,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":13763,"source_line":26,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_vector
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1588,"record_offset":23324,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":13757,"source_line":93,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry::Isometry::from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1588,"record_offset":24240,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":13781,"source_line":119,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$3::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1588,"record_offset":22976,"parent_record_offset":22944,"ancestor_record_offset":22656,"inlinee":13755,"source_line":78,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$12::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1588,"record_offset":23080,"parent_record_offset":23052,"ancestor_record_offset":22656,"inlinee":13769,"source_line":93,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1588,"record_offset":23052,"parent_record_offset":23024,"ancestor_record_offset":22656,"inlinee":13767,"source_line":93,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$19::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1588,"record_offset":23024,"parent_record_offset":22976,"ancestor_record_offset":22656,"inlinee":13765,"source_line":93,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1588,"record_offset":23356,"parent_record_offset":23324,"ancestor_record_offset":22656,"inlinee":13759,"source_line":93,"archive":"_pdb/B/modules/01588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x130A4
// 首条位置: {"module":1737,"inlinee":77988}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A60
// 首条位置: {"module":1585,"inlinee":72288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] nalgebra::geometry::isometry::Isometry::transform_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1585,"record_offset":50236,"parent_record_offset":50200,"ancestor_record_offset":50200,"inlinee":72289,"source_line":306,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$18::clone
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1585,"record_offset":50416,"parent_record_offset":50376,"ancestor_record_offset":50200,"inlinee":11764,"source_line":336,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":50528,"parent_record_offset":50376,"ancestor_record_offset":50200,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":51164,"parent_record_offset":51136,"ancestor_record_offset":50200,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":50588,"parent_record_offset":50376,"ancestor_record_offset":50200,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":51136,"parent_record_offset":51108,"ancestor_record_offset":50200,"inlinee":13776,"source_line":183,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1585,"record_offset":50376,"parent_record_offset":50340,"ancestor_record_offset":50200,"inlinee":13761,"source_line":2054,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":51108,"parent_record_offset":51080,"ancestor_record_offset":50200,"inlinee":13775,"source_line":272,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":51080,"parent_record_offset":50340,"ancestor_record_offset":50200,"inlinee":13773,"source_line":308,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":50760,"parent_record_offset":50340,"ancestor_record_offset":50200,"inlinee":13771,"source_line":467,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1585,"record_offset":50288,"parent_record_offset":50260,"ancestor_record_offset":50200,"inlinee":34764,"source_line":98,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$29::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1585,"record_offset":50260,"parent_record_offset":50236,"ancestor_record_offset":50200,"inlinee":34820,"source_line":78,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::point_ops::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1585,"record_offset":51332,"parent_record_offset":51304,"ancestor_record_offset":50200,"inlinee":34822,"source_line":187,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion::Quaternion::scalar
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion.rs
//     evidence={"module":1585,"record_offset":50704,"parent_record_offset":50340,"ancestor_record_offset":50200,"inlinee":34812,"source_line":214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1585,"record_offset":50340,"parent_record_offset":50312,"ancestor_record_offset":50200,"inlinee":13759,"source_line":93,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$44::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1585,"record_offset":50312,"parent_record_offset":50288,"ancestor_record_offset":50200,"inlinee":34766,"source_line":93,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::translation_ops::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1585,"record_offset":51304,"parent_record_offset":50260,"ancestor_record_offset":50200,"inlinee":35147,"source_line":187,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x130A3
// 首条位置: {"module":1737,"inlinee":77987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AE1
// 首条位置: {"module":1585,"inlinee":72417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":447768,"parent_record_offset":447736,"ancestor_record_offset":447736,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$94::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1737,"record_offset":94004,"parent_record_offset":92636,"ancestor_record_offset":92636,"inlinee":26261,"source_line":1873,"archive":"_pdb/B/modules/01737.jsonl.gz"}
//   [pdb_inline_direct] nalgebra::geometry::isometry::Isometry::transform_point
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1585,"record_offset":447876,"parent_record_offset":447736,"ancestor_record_offset":447736,"inlinee":72289,"source_line":306,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$18::clone
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":1585,"record_offset":448060,"parent_record_offset":448020,"ancestor_record_offset":447736,"inlinee":11764,"source_line":336,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":448172,"parent_record_offset":448020,"ancestor_record_offset":447736,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":448672,"parent_record_offset":448648,"ancestor_record_offset":447736,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":448260,"parent_record_offset":448020,"ancestor_record_offset":447736,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1585,"record_offset":447800,"parent_record_offset":447768,"ancestor_record_offset":447736,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1585,"record_offset":447832,"parent_record_offset":447800,"ancestor_record_offset":447736,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1737,"record_offset":94028,"parent_record_offset":94004,"ancestor_record_offset":92636,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/01737.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":448648,"parent_record_offset":448624,"ancestor_record_offset":447736,"inlinee":13776,"source_line":183,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1585,"record_offset":448020,"parent_record_offset":447980,"ancestor_record_offset":447736,"inlinee":13761,"source_line":2054,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":448624,"parent_record_offset":448600,"ancestor_record_offset":447736,"inlinee":13775,"source_line":272,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":448600,"parent_record_offset":447980,"ancestor_record_offset":447736,"inlinee":13773,"source_line":308,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1585,"record_offset":448320,"parent_record_offset":447980,"ancestor_record_offset":447736,"inlinee":13771,"source_line":467,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_point
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1585,"record_offset":447928,"parent_record_offset":447900,"ancestor_record_offset":447736,"inlinee":34764,"source_line":98,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$29::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1585,"record_offset":447900,"parent_record_offset":447876,"ancestor_record_offset":447736,"inlinee":34820,"source_line":78,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::point_ops::impl$14::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1585,"record_offset":448836,"parent_record_offset":448808,"ancestor_record_offset":447736,"inlinee":34822,"source_line":187,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion::Quaternion::scalar
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion.rs
//     evidence={"module":1737,"record_offset":93220,"parent_record_offset":92880,"ancestor_record_offset":92636,"inlinee":34812,"source_line":214,"archive":"_pdb/B/modules/01737.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1585,"record_offset":447980,"parent_record_offset":447952,"ancestor_record_offset":447736,"inlinee":13759,"source_line":93,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$44::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1585,"record_offset":447952,"parent_record_offset":447928,"ancestor_record_offset":447736,"inlinee":34766,"source_line":93,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::translation_ops::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1585,"record_offset":448808,"parent_record_offset":447900,"ancestor_record_offset":447736,"inlinee":35147,"source_line":187,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 完整结构及原始类型记录: visualize_colliders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: visualize_colliders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: visualize_colliders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_colliders
// 信息 ID: 7184eae6b4dfa6bebe14bd87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 56bc6ac7b43a89d4520b70e3: void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x14e4e60 -> 0x1326370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17dc715 -> 0x1326370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape
// 信息 ID: 099f0d1646c46c998a077124
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_colliders @ 0x1326650 -> 0x1326930 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x1326b83 -> 0x1326930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0
// 信息 ID: 4e9793cbdd08a9a13a988521
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 2 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure$0
// 信息 ID: f2ee3ec3f5a8f421d870823f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1585, inline_record=50200; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1585, inline_record=52364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1
// 信息 ID: d784612f38bb0b022cb4393b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 1 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure$0
// 信息 ID: 76595fd9fa1e8632e22732c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<u32>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1585, inline_record=447736; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1585, inline_record=449812; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1585, inline_record=451240; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<u32>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>; module=1737, inline_record=92636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
