// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/collision/visualize_colliders.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fad8c4604e63694018439419c26dc8610b12763ff26b74d9ce288bf471f93729
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visualize_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1681, 1682
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nalgebra; // namespace candidate
//   package identities: [{"name":"nalgebra","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.33.0"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rapier3d; // namespace candidate
//   package identities: [{"name":"rapier3d","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050","version":"0.22.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::is_shown_by_all
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1434B
// 首条位置: {"module":1681,"inlinee":82763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::category_of
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14348
// 首条位置: {"module":1681,"inlinee":82760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1681,"record_offset":41124,"rva":25215872,"va":5393924992}
// 编译实例 RVAs: 0x180c380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1145538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e83e85eeff","size":5,"soft_fail":false,"target_va":5392780768,"va":5393926301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22458962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e852b25601","size":5,"soft_fail":false,"target_va":5416385024,"va":5393926057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22695356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e8bc4d5a01","size":5,"soft_fail":false,"target_va":5416621408,"va":5393926047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e8c5090000","size":5,"soft_fail":false,"target_va":5393927568,"va":5393925062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t22695212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e82c4d5a01","size":5,"soft_fail":false,"target_va":5416621376,"va":5393926159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e89bf4ffff","size":5,"soft_fail":false,"target_va":5393922704,"va":5393925616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9961639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e859ff67ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5393925426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9953135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e8912068ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5393925354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9961026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e8be0168ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5393925933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9962225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e80ffd67ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5393925036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::begin_panic::<&str>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22688650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e88a335a01","size":5,"soft_fail":false,"target_va":5416614687,"va":5393926032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22693231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393924992,"raw_hex":"e86f455a01","size":5,"soft_fail":false,"target_va":5416619360,"va":5393926124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1681,"record_offset":43564,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":44576,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":44732,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$57::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1681,"record_offset":42724,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":16430,"source_line":417,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::visualize_colliders::category_of
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/visualize_colliders.rs
//     evidence={"module":1681,"record_offset":44548,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":82760,"source_line":28,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1681,"record_offset":44520,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":31415,"source_line":1602,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1681,"record_offset":44464,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":8883,"source_line":262,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":42756,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":41332,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":41712,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider::Collider::shape
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider.rs
//     evidence={"module":1681,"record_offset":43996,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":82757,"source_line":431,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider_set::ColliderSet::iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider_set.rs
//     evidence={"module":1681,"record_offset":43708,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":82755,"source_line":42,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$8::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":1681,"record_offset":44252,"parent_record_offset":41124,"ancestor_record_offset":41124,"inlinee":8887,"source_line":998,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":43836,"parent_record_offset":43812,"ancestor_record_offset":41124,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":43884,"parent_record_offset":43860,"ancestor_record_offset":41124,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":43860,"parent_record_offset":43836,"ancestor_record_offset":41124,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1681,"record_offset":44044,"parent_record_offset":44020,"ancestor_record_offset":41124,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":43812,"parent_record_offset":43788,"ancestor_record_offset":41124,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":43788,"parent_record_offset":43756,"ancestor_record_offset":41124,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":43756,"parent_record_offset":43732,"ancestor_record_offset":41124,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43084,"parent_record_offset":43056,"ancestor_record_offset":41124,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43112,"parent_record_offset":43084,"ancestor_record_offset":41124,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43032,"parent_record_offset":43004,"ancestor_record_offset":41124,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43056,"parent_record_offset":43032,"ancestor_record_offset":41124,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43172,"parent_record_offset":43056,"ancestor_record_offset":41124,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1681,"record_offset":43284,"parent_record_offset":43260,"ancestor_record_offset":41124,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1681,"record_offset":43588,"parent_record_offset":43564,"ancestor_record_offset":41124,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1681,"record_offset":43136,"parent_record_offset":43112,"ancestor_record_offset":41124,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1681,"record_offset":42904,"parent_record_offset":42864,"ancestor_record_offset":41124,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":44388,"parent_record_offset":44288,"ancestor_record_offset":41124,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1681,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":41124,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1681,"record_offset":43520,"parent_record_offset":42864,"ancestor_record_offset":41124,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":43236,"parent_record_offset":43004,"ancestor_record_offset":41124,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1681,"record_offset":43952,"parent_record_offset":43928,"ancestor_record_offset":41124,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":43668,"parent_record_offset":43612,"ancestor_record_offset":41124,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":42132,"parent_record_offset":42108,"ancestor_record_offset":41124,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":42108,"parent_record_offset":42084,"ancestor_record_offset":41124,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":42316,"parent_record_offset":42288,"ancestor_record_offset":41124,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1681,"record_offset":42416,"parent_record_offset":42388,"ancestor_record_offset":41124,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1681,"record_offset":42388,"parent_record_offset":42360,"ancestor_record_offset":41124,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1681,"record_offset":44324,"parent_record_offset":44288,"ancestor_record_offset":41124,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1681,"record_offset":44352,"parent_record_offset":44324,"ancestor_record_offset":41124,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1681,"record_offset":43928,"parent_record_offset":43756,"ancestor_record_offset":41124,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":42060,"parent_record_offset":42036,"ancestor_record_offset":41124,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":42084,"parent_record_offset":42060,"ancestor_record_offset":41124,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":42444,"parent_record_offset":42416,"ancestor_record_offset":41124,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":43612,"parent_record_offset":43588,"ancestor_record_offset":41124,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":42036,"parent_record_offset":41920,"ancestor_record_offset":41124,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1681,"record_offset":41460,"parent_record_offset":41436,"ancestor_record_offset":41124,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1681,"record_offset":41748,"parent_record_offset":41712,"ancestor_record_offset":41124,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1681,"record_offset":41888,"parent_record_offset":41856,"ancestor_record_offset":41124,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1681,"record_offset":41856,"parent_record_offset":41820,"ancestor_record_offset":41124,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1681,"record_offset":41796,"parent_record_offset":41772,"ancestor_record_offset":41124,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1681,"record_offset":41820,"parent_record_offset":41796,"ancestor_record_offset":41124,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1681,"record_offset":41772,"parent_record_offset":41748,"ancestor_record_offset":41124,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1681,"record_offset":42360,"parent_record_offset":41820,"ancestor_record_offset":41124,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41436,"parent_record_offset":41404,"ancestor_record_offset":41124,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41404,"parent_record_offset":41376,"ancestor_record_offset":41124,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41560,"parent_record_offset":41528,"ancestor_record_offset":41124,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41612,"parent_record_offset":41560,"ancestor_record_offset":41124,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41528,"parent_record_offset":41500,"ancestor_record_offset":41124,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41376,"parent_record_offset":41332,"ancestor_record_offset":41124,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1681,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":41124,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1681,"record_offset":41920,"parent_record_offset":41888,"ancestor_record_offset":41124,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1681,"record_offset":42204,"parent_record_offset":41920,"ancestor_record_offset":41124,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1681,"record_offset":42176,"parent_record_offset":41920,"ancestor_record_offset":41124,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1681,"record_offset":41980,"parent_record_offset":41920,"ancestor_record_offset":41124,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$57::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1681,"record_offset":44632,"parent_record_offset":44600,"ancestor_record_offset":41124,"inlinee":16430,"source_line":417,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::visualize_colliders::is_shown_by_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/visualize_colliders.rs
//     evidence={"module":1681,"record_offset":44696,"parent_record_offset":44600,"ancestor_record_offset":41124,"inlinee":82763,"source_line":19,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::visualize_colliders::visualize_colliders::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/visualize_colliders.rs
//     evidence={"module":1681,"record_offset":44600,"parent_record_offset":44576,"ancestor_record_offset":41124,"inlinee":82762,"source_line":73,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::shape::shared_shape::impl$1::as_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1681,"record_offset":44020,"parent_record_offset":43996,"ancestor_record_offset":41124,"inlinee":82759,"source_line":32,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":42792,"parent_record_offset":42756,"ancestor_record_offset":41124,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":44796,"parent_record_offset":44764,"ancestor_record_offset":41124,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":45252,"parent_record_offset":45048,"ancestor_record_offset":41124,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1681,"record_offset":42932,"parent_record_offset":42904,"ancestor_record_offset":41124,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":43004,"parent_record_offset":42864,"ancestor_record_offset":41124,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1681,"record_offset":43208,"parent_record_offset":43004,"ancestor_record_offset":41124,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::Arena::iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1681,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":41124,"inlinee":82756,"source_line":701,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::impl$11::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1681,"record_offset":44104,"parent_record_offset":44080,"ancestor_record_offset":41124,"inlinee":38680,"source_line":937,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::KeyData::from_ffi
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/lib.rs
//     evidence={"module":1681,"record_offset":44488,"parent_record_offset":44464,"ancestor_record_offset":41124,"inlinee":8885,"source_line":289,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":1681,"record_offset":44288,"parent_record_offset":44252,"ancestor_record_offset":41124,"inlinee":8889,"source_line":656,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":1681,"record_offset":44424,"parent_record_offset":44388,"ancestor_record_offset":41124,"inlinee":8891,"source_line":660,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1681,"record_offset":42960,"parent_record_offset":42932,"ancestor_record_offset":41124,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1681,"record_offset":42864,"parent_record_offset":42828,"ancestor_record_offset":41124,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1681,"record_offset":42828,"parent_record_offset":42792,"ancestor_record_offset":41124,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1434A
// 首条位置: {"module":1681,"inlinee":82762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::gui::impl$57::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1681,"record_offset":44632,"parent_record_offset":44600,"ancestor_record_offset":44600,"inlinee":16430,"source_line":417,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::visualize_colliders::is_shown_by_all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/visualize_colliders.rs
//     evidence={"module":1681,"record_offset":44696,"parent_record_offset":44600,"ancestor_record_offset":44600,"inlinee":82763,"source_line":19,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1681,"record_offset":36272,"rva":25213584,"va":5393922704}
// 编译实例 RVAs: 0x180ba90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_ball
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11779226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e866434cff","size":5,"soft_fail":false,"target_va":5382145008,"va":5393924229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_compound
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11777957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e85b484cff","size":5,"soft_fail":false,"target_va":5382144880,"va":5393922832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_convex_polyhedron
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11778571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e8f5454cff","size":5,"soft_fail":false,"target_va":5382144944,"va":5393923510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11778757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e83b454cff","size":5,"soft_fail":false,"target_va":5382145072,"va":5393923824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_trimesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11778475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e855464cff","size":5,"soft_fail":false,"target_va":5382144816,"va":5393923286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18245376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e80099e9fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393924059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-18245070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e8329ae9fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393923769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25201857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e83f737ffe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393924044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140904120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15761326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e852800fff","size":5,"soft_fail":false,"target_va":5378162976,"va":5393924297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140906710
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15750802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e86ea90fff","size":5,"soft_fail":false,"target_va":5378172688,"va":5393923485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416163c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2054577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e84fa6e0ff","size":5,"soft_fail":false,"target_va":5391868864,"va":5393923436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22696354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e8a2515a01","size":5,"soft_fail":false,"target_va":5416620769,"va":5393924410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22697768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e828575a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393924467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e8d2fdffff","size":5,"soft_fail":false,"target_va":5393922704,"va":5393923257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13413533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e8635333ff","size":5,"soft_fail":false,"target_va":5380510400,"va":5393923928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::transformation::to_trimesh::convex_polyhedron_to_trimesh::_$LT$impl$u20$parry3d..shape..convex_polyhedron..ConvexPolyhedron$GT$::to_trimesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11775648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393922704,"raw_hex":"e860514cff","size":5,"soft_fail":false,"target_va":5382147888,"va":5393923531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*296(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5393922704,"raw_hex":"ff9328010000","size":6,"soft_fail":false,"va":5393924421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":39500,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":39320,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":39436,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":38632,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":36476,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::visualize_colliders::visualize_triangles
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/visualize_colliders.rs
//     evidence={"module":1681,"record_offset":37968,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":82754,"source_line":143,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$36::to_glam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1681,"record_offset":39208,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":36767,"source_line":2414,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":39684,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":33494,"source_line":425,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":39712,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":40112,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":33466,"source_line":119,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":40144,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":28664,"source_line":141,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] nalgebra::geometry::isometry_ops::impl$2::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1681,"record_offset":36540,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":8843,"source_line":78,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] parry3d::shape::shared_shape::impl$0::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1681,"record_offset":37904,"parent_record_offset":36272,"ancestor_record_offset":36272,"inlinee":8877,"source_line":26,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":39584,"parent_record_offset":39560,"ancestor_record_offset":36272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":39608,"parent_record_offset":39584,"ancestor_record_offset":36272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":38816,"parent_record_offset":38792,"ancestor_record_offset":36272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":38840,"parent_record_offset":38816,"ancestor_record_offset":36272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":39632,"parent_record_offset":39608,"ancestor_record_offset":36272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":38864,"parent_record_offset":38840,"ancestor_record_offset":36272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":39560,"parent_record_offset":39528,"ancestor_record_offset":36272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1681,"record_offset":38792,"parent_record_offset":38704,"ancestor_record_offset":36272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1681,"record_offset":39528,"parent_record_offset":39500,"ancestor_record_offset":36272,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":38732,"parent_record_offset":38704,"ancestor_record_offset":36272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":38704,"parent_record_offset":38680,"ancestor_record_offset":36272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1681,"record_offset":38680,"parent_record_offset":38656,"ancestor_record_offset":36272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1681,"record_offset":37932,"parent_record_offset":37904,"ancestor_record_offset":36272,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":38172,"parent_record_offset":38144,"ancestor_record_offset":36272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":39764,"parent_record_offset":39740,"ancestor_record_offset":36272,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1681,"record_offset":39812,"parent_record_offset":39788,"ancestor_record_offset":36272,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$35::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":1681,"record_offset":36836,"parent_record_offset":36768,"ancestor_record_offset":36272,"inlinee":8861,"source_line":614,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1681,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":36272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1681,"record_offset":38012,"parent_record_offset":37968,"ancestor_record_offset":36272,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1681,"record_offset":38200,"parent_record_offset":38172,"ancestor_record_offset":36272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1681,"record_offset":38144,"parent_record_offset":37968,"ancestor_record_offset":36272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1681,"record_offset":39348,"parent_record_offset":39320,"ancestor_record_offset":36272,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1681,"record_offset":39372,"parent_record_offset":39348,"ancestor_record_offset":36272,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1681,"record_offset":38760,"parent_record_offset":38732,"ancestor_record_offset":36272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1681,"record_offset":37612,"parent_record_offset":37532,"ancestor_record_offset":36272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1681,"record_offset":36728,"parent_record_offset":36696,"ancestor_record_offset":36272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1681,"record_offset":37312,"parent_record_offset":37288,"ancestor_record_offset":36272,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1681,"record_offset":36920,"parent_record_offset":36768,"ancestor_record_offset":36272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":38108,"parent_record_offset":38012,"ancestor_record_offset":36272,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1681,"record_offset":38656,"parent_record_offset":38632,"ancestor_record_offset":36272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1681,"record_offset":39404,"parent_record_offset":39320,"ancestor_record_offset":36272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":36504,"parent_record_offset":36476,"ancestor_record_offset":36272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":38044,"parent_record_offset":38012,"ancestor_record_offset":36272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":38240,"parent_record_offset":37968,"ancestor_record_offset":36272,"inlinee":33494,"source_line":425,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":38268,"parent_record_offset":37968,"ancestor_record_offset":36272,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1681,"record_offset":39280,"parent_record_offset":39256,"ancestor_record_offset":36272,"inlinee":9837,"source_line":94,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$94::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1681,"record_offset":39256,"parent_record_offset":39232,"ancestor_record_offset":36272,"inlinee":28796,"source_line":2127,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":37288,"parent_record_offset":37264,"ancestor_record_offset":36272,"inlinee":8868,"source_line":183,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":37828,"parent_record_offset":37800,"ancestor_record_offset":36272,"inlinee":8873,"source_line":212,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1681,"record_offset":36768,"parent_record_offset":36648,"ancestor_record_offset":36272,"inlinee":8859,"source_line":2054,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":37264,"parent_record_offset":37240,"ancestor_record_offset":36272,"inlinee":8867,"source_line":272,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$22::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":37800,"parent_record_offset":36572,"ancestor_record_offset":36272,"inlinee":8872,"source_line":290,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":37240,"parent_record_offset":36648,"ancestor_record_offset":36272,"inlinee":8865,"source_line":308,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1681,"record_offset":36696,"parent_record_offset":36648,"ancestor_record_offset":36272,"inlinee":8863,"source_line":467,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::unit::impl$13::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/unit.rs
//     evidence={"module":1681,"record_offset":37456,"parent_record_offset":36572,"ancestor_record_offset":36272,"inlinee":8847,"source_line":26,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_vector
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1681,"record_offset":36616,"parent_record_offset":36572,"ancestor_record_offset":36272,"inlinee":8855,"source_line":93,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry::Isometry::from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1681,"record_offset":37764,"parent_record_offset":36572,"ancestor_record_offset":36272,"inlinee":8875,"source_line":119,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$3::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1681,"record_offset":36572,"parent_record_offset":36540,"ancestor_record_offset":36272,"inlinee":8845,"source_line":78,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$12::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1681,"record_offset":37532,"parent_record_offset":37508,"ancestor_record_offset":36272,"inlinee":8853,"source_line":93,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1681,"record_offset":37508,"parent_record_offset":37484,"ancestor_record_offset":36272,"inlinee":8851,"source_line":93,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$19::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1681,"record_offset":37484,"parent_record_offset":36572,"ancestor_record_offset":36272,"inlinee":8849,"source_line":93,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1681,"record_offset":36648,"parent_record_offset":36616,"ancestor_record_offset":36272,"inlinee":8857,"source_line":93,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_triangles
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14342
// 首条位置: {"module":1681,"inlinee":82754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1681,"record_offset":38012,"parent_record_offset":37968,"ancestor_record_offset":37968,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1681,"record_offset":38144,"parent_record_offset":37968,"ancestor_record_offset":37968,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":38240,"parent_record_offset":37968,"ancestor_record_offset":37968,"inlinee":33494,"source_line":425,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1681,"record_offset":38268,"parent_record_offset":37968,"ancestor_record_offset":37968,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1681,"record_offset":38172,"parent_record_offset":38144,"ancestor_record_offset":37968,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1681,"record_offset":38200,"parent_record_offset":38172,"ancestor_record_offset":37968,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1681,"record_offset":38108,"parent_record_offset":38012,"ancestor_record_offset":37968,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1681,"record_offset":38072,"parent_record_offset":38044,"ancestor_record_offset":37968,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1681,"record_offset":38044,"parent_record_offset":38012,"ancestor_record_offset":37968,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01681.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_triangles::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1439D
// 首条位置: {"module":1682,"inlinee":82845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] nalgebra::geometry::isometry::Isometry::transform_point
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1682,"record_offset":130188,"parent_record_offset":130156,"ancestor_record_offset":130156,"inlinee":82846,"source_line":306,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$35::clone
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":1682,"record_offset":130352,"parent_record_offset":130328,"ancestor_record_offset":130156,"inlinee":8861,"source_line":614,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":130540,"parent_record_offset":130384,"ancestor_record_offset":130156,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":131064,"parent_record_offset":131036,"ancestor_record_offset":130156,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1682,"record_offset":130628,"parent_record_offset":130384,"ancestor_record_offset":130156,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1682,"record_offset":131036,"parent_record_offset":131008,"ancestor_record_offset":130156,"inlinee":8868,"source_line":183,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1682,"record_offset":130384,"parent_record_offset":130292,"ancestor_record_offset":130156,"inlinee":8859,"source_line":2054,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1682,"record_offset":131008,"parent_record_offset":130980,"ancestor_record_offset":130156,"inlinee":8867,"source_line":272,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1682,"record_offset":130980,"parent_record_offset":130292,"ancestor_record_offset":130156,"inlinee":8865,"source_line":308,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1682,"record_offset":130716,"parent_record_offset":130292,"ancestor_record_offset":130156,"inlinee":8863,"source_line":467,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_point
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1682,"record_offset":130240,"parent_record_offset":130212,"ancestor_record_offset":130156,"inlinee":38254,"source_line":98,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$29::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1682,"record_offset":130212,"parent_record_offset":130188,"ancestor_record_offset":130156,"inlinee":40920,"source_line":78,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::point_ops::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1682,"record_offset":131232,"parent_record_offset":131204,"ancestor_record_offset":130156,"inlinee":38260,"source_line":187,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion::Quaternion::scalar
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion.rs
//     evidence={"module":1682,"record_offset":130328,"parent_record_offset":130292,"ancestor_record_offset":130156,"inlinee":40428,"source_line":214,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1682,"record_offset":130292,"parent_record_offset":130264,"ancestor_record_offset":130156,"inlinee":8857,"source_line":93,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$44::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1682,"record_offset":130264,"parent_record_offset":130240,"ancestor_record_offset":130156,"inlinee":38256,"source_line":93,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::translation_ops::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1682,"record_offset":131204,"parent_record_offset":130212,"ancestor_record_offset":130156,"inlinee":38258,"source_line":187,"archive":"_pdb/C/modules/01682.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::category_of
// 信息 ID: a98abf38bce3e284d62f24fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_colliders; module=1681, inline_record=44548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::is_shown_by_all
// 信息 ID: 89a768fba3bb474d36e6d4bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_colliders; module=1681, inline_record=44696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_colliders
// 信息 ID: e517882c7b0c7cdcaed91380
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 56bc6ac7b43a89d4520b70e3: void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1802f5c -> 0x180c380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x19324c6 -> 0x180c380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_colliders::closure$0
// 信息 ID: 4b5716dc036d40ad8112bf22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_colliders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_colliders; module=1681, inline_record=44600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape
// 信息 ID: 73345bffcd5980ec004d7214
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x180bcb9 -> 0x180ba90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_colliders @ 0x180c5f0 -> 0x180ba90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_triangles
// 信息 ID: 7f90cf975a70e8becda060b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=37968; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::visualize_colliders::visualize_shape; module=1681, inline_record=38300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_triangles::closure$0
// 信息 ID: 77f38450ddac48801cac181a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_triangles", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_triangles::closure_env$0> > > >; module=1682, inline_record=130156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
