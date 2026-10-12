// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/ivy/ivy_grower.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b05c59496c9a2b1542af282f2dfb0a91def7eef95a3a2192adc293e14f55c818
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_grower.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 512, 560, 574, 1585, 1592, 1654, 1736, 3208
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::ivy_grower
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":26584,"rva":9742608,"va":5378451728}
// 编译实例 RVAs: 0x94a910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9501843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e86d036fff","size":5,"soft_fail":false,"target_va":5368953728,"va":5378455566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3777410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e87e5cc6ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378453741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3375435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8b57eccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378456934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4baa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2095772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e89cfa1f00","size":5,"soft_fail":false,"target_va":5380553376,"va":5378457599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3478432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8a0133500","size":5,"soft_fail":false,"target_va":5381934528,"va":5378456091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f6b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10138197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e855b29a00","size":5,"soft_fail":false,"target_va":5388594016,"va":5378455814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141308470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10210063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e80fcb9b00","size":5,"soft_fail":false,"target_va":5388665968,"va":5378455900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141309480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10217316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e864e79b00","size":5,"soft_fail":false,"target_va":5388670080,"va":5378452759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141338fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10407931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8fbcf9e00","size":5,"soft_fail":false,"target_va":5388865504,"va":5378457568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11646900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8b4b7b100","size":5,"soft_fail":false,"target_va":5390103152,"va":5378456247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11650829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e80dc7b100","size":5,"soft_fail":false,"target_va":5390105824,"va":5378454990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141468f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11654414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e80ed5b100","size":5,"soft_fail":false,"target_va":5390110608,"va":5378456189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11655259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e85bd8b100","size":5,"soft_fail":false,"target_va":5390110720,"va":5378455456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469a20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11658469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8e5e4b100","size":5,"soft_fail":false,"target_va":5390113312,"va":5378454838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11667578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e87a08b200","size":5,"soft_fail":false,"target_va":5390123968,"va":5378456385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33211990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e856c6fa01","size":5,"soft_fail":false,"target_va":5411669968,"va":5378457973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33394534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8668ffd01","size":5,"soft_fail":false,"target_va":5411847904,"va":5378453365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4097699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e85d79c1ff","size":5,"soft_fail":false,"target_va":5374356528,"va":5378454222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33083501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e86dd0f801","size":5,"soft_fail":false,"target_va":5411541584,"va":5378458078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33083710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e83ed1f801","size":5,"soft_fail":false,"target_va":5411541776,"va":5378458061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33083957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e835d2f801","size":5,"soft_fail":false,"target_va":5411542064,"va":5378458102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33084114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8d2d2f801","size":5,"soft_fail":false,"target_va":5411542296,"va":5378458177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e857e0ffff","size":5,"soft_fail":false,"target_va":5378443808,"va":5378451908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33091377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e831eff801","size":5,"soft_fail":false,"target_va":5411543360,"va":5378451978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer::get_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9343167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8bf908e00","size":5,"soft_fail":false,"target_va":5387797904,"va":5378454732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::add_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-928539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8e5d4f1ff","size":5,"soft_fail":false,"target_va":5377528928,"va":5378457462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-926364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e864ddf1ff","size":5,"soft_fail":false,"target_va":5377529072,"va":5378455431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::last_segment_point_ids
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-925013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8abe2f1ff","size":5,"soft_fail":false,"target_va":5377529152,"va":5378454160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::wall_coord::WallCoord::into_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1892898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e822e21c00","size":5,"soft_fail":false,"target_va":5380348384,"va":5378455481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1336610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e822651400","size":5,"soft_fail":false,"target_va":5379792624,"va":5378456009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::PrevWallHoles::wall_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6534453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e835b56300","size":5,"soft_fail":false,"target_va":5384989952,"va":5378455494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::closest_segment_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e891170000","size":5,"soft_fail":false,"target_va":5378459648,"va":5378453610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8f10b0000","size":5,"soft_fail":false,"target_va":5378460320,"va":5378457258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1417048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8589f1500","size":5,"soft_fail":false,"target_va":5379872848,"va":5378455795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2933081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e859c12c00","size":5,"soft_fail":false,"target_va":5381385536,"va":5378452450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2938270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e89ed52c00","size":5,"soft_fail":false,"target_va":5381390640,"va":5378452365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2928723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e853b02c00","size":5,"soft_fail":false,"target_va":5381386480,"va":5378457752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2932831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e85fc02c00","size":5,"soft_fail":false,"target_va":5381384704,"va":5378451868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10366670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8ce2e9e00","size":5,"soft_fail":false,"target_va":5388825296,"va":5378458621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33079050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e80abff801","size":5,"soft_fail":false,"target_va":5411537104,"va":5378458049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9390302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378451728,"raw_hex":"e8de488f00","size":5,"soft_fail":false,"target_va":5387845952,"va":5378455645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":30124,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":512,"record_offset":30152,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":512,"record_offset":29300,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":512,"record_offset":34144,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":33468,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":512,"record_offset":30068,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":512,"record_offset":30012,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":15461,"source_line":911,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":44100,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":41460,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":31296,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":32344,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":512,"record_offset":27300,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":27169,"source_line":253,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":31524,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":26825,"source_line":61,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":31496,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":26766,"source_line":57,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::set_last_point_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":33772,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":26826,"source_line":65,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":30208,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":29384,"source_line":176,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":512,"record_offset":40912,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":29393,"source_line":60,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":512,"record_offset":32372,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::check_for_wall_jump
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":38160,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":29394,"source_line":308,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_growth_point_proposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":35596,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":29389,"source_line":227,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":512,"record_offset":31908,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":29385,"source_line":130,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":41276,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":33500,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":41044,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":28316,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":26840,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":27340,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":512,"record_offset":30596,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":14696,"source_line":2586,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":35568,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":40984,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::is_within_01_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":40884,"parent_record_offset":26584,"ancestor_record_offset":26584,"inlinee":28021,"source_line":218,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":40364,"parent_record_offset":40340,"ancestor_record_offset":26584,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":40340,"parent_record_offset":40252,"ancestor_record_offset":26584,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":31548,"parent_record_offset":31524,"ancestor_record_offset":26584,"inlinee":11299,"source_line":1653,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":31572,"parent_record_offset":31548,"ancestor_record_offset":26584,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":33824,"parent_record_offset":33800,"ancestor_record_offset":26584,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":31772,"parent_record_offset":31572,"ancestor_record_offset":26584,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":31608,"parent_record_offset":31572,"ancestor_record_offset":26584,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":31636,"parent_record_offset":31608,"ancestor_record_offset":26584,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":33800,"parent_record_offset":33772,"ancestor_record_offset":26584,"inlinee":26749,"source_line":2932,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":31704,"parent_record_offset":31664,"ancestor_record_offset":26584,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":31808,"parent_record_offset":31772,"ancestor_record_offset":26584,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":31728,"parent_record_offset":31704,"ancestor_record_offset":26584,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40280,"parent_record_offset":40252,"ancestor_record_offset":26584,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40252,"parent_record_offset":40228,"ancestor_record_offset":26584,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":31856,"parent_record_offset":31832,"ancestor_record_offset":26584,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":31832,"parent_record_offset":31808,"ancestor_record_offset":26584,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40228,"parent_record_offset":40204,"ancestor_record_offset":26584,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":512,"record_offset":38560,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":512,"record_offset":38588,"parent_record_offset":38560,"ancestor_record_offset":26584,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":34308,"parent_record_offset":34264,"ancestor_record_offset":26584,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":29640,"parent_record_offset":29616,"ancestor_record_offset":26584,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":38336,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":38532,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":34264,"parent_record_offset":34228,"ancestor_record_offset":26584,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":37208,"parent_record_offset":37116,"ancestor_record_offset":26584,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":30092,"parent_record_offset":30068,"ancestor_record_offset":26584,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":29616,"parent_record_offset":29388,"ancestor_record_offset":26584,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":512,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":26584,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":512,"record_offset":34168,"parent_record_offset":34144,"ancestor_record_offset":26584,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":512,"record_offset":34228,"parent_record_offset":34168,"ancestor_record_offset":26584,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":512,"record_offset":29364,"parent_record_offset":29324,"ancestor_record_offset":26584,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":512,"record_offset":29388,"parent_record_offset":29364,"ancestor_record_offset":26584,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":512,"record_offset":29324,"parent_record_offset":29300,"ancestor_record_offset":26584,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28652,"parent_record_offset":28624,"ancestor_record_offset":26584,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28704,"parent_record_offset":28680,"ancestor_record_offset":26584,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28680,"parent_record_offset":28652,"ancestor_record_offset":26584,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28600,"parent_record_offset":28572,"ancestor_record_offset":26584,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28624,"parent_record_offset":28600,"ancestor_record_offset":26584,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28796,"parent_record_offset":28624,"ancestor_record_offset":26584,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":512,"record_offset":28908,"parent_record_offset":28884,"ancestor_record_offset":26584,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":512,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":26584,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":512,"record_offset":30444,"parent_record_offset":30416,"ancestor_record_offset":26584,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":512,"record_offset":32700,"parent_record_offset":32676,"ancestor_record_offset":26584,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":512,"record_offset":30504,"parent_record_offset":30472,"ancestor_record_offset":26584,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":26584,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":39968,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":512,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":26584,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":512,"record_offset":33260,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":512,"record_offset":30036,"parent_record_offset":30012,"ancestor_record_offset":26584,"inlinee":15463,"source_line":23,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":512,"record_offset":38012,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":512,"record_offset":38496,"parent_record_offset":38472,"ancestor_record_offset":26584,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":512,"record_offset":38448,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":512,"record_offset":28728,"parent_record_offset":28704,"ancestor_record_offset":26584,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":29572,"parent_record_offset":29544,"ancestor_record_offset":26584,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":29432,"parent_record_offset":29388,"ancestor_record_offset":26584,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":40308,"parent_record_offset":40280,"ancestor_record_offset":26584,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":31664,"parent_record_offset":31636,"ancestor_record_offset":26584,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":512,"record_offset":27000,"parent_record_offset":26968,"ancestor_record_offset":26584,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":512,"record_offset":30900,"parent_record_offset":30868,"ancestor_record_offset":26584,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":512,"record_offset":30868,"parent_record_offset":30836,"ancestor_record_offset":26584,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":26584,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":33560,"parent_record_offset":33528,"ancestor_record_offset":26584,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":32000,"parent_record_offset":31976,"ancestor_record_offset":26584,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":39000,"parent_record_offset":38968,"ancestor_record_offset":26584,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":38124,"parent_record_offset":38100,"ancestor_record_offset":26584,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":26584,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":512,"record_offset":28468,"parent_record_offset":28424,"ancestor_record_offset":26584,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":32304,"parent_record_offset":31908,"ancestor_record_offset":26584,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":36116,"parent_record_offset":35748,"ancestor_record_offset":26584,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":512,"record_offset":27248,"parent_record_offset":27220,"ancestor_record_offset":26584,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":512,"record_offset":29256,"parent_record_offset":28424,"ancestor_record_offset":26584,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":28860,"parent_record_offset":28572,"ancestor_record_offset":26584,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":512,"record_offset":29904,"parent_record_offset":29880,"ancestor_record_offset":26584,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":512,"record_offset":31160,"parent_record_offset":31136,"ancestor_record_offset":26584,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":512,"record_offset":27752,"parent_record_offset":27728,"ancestor_record_offset":26584,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":512,"record_offset":27728,"parent_record_offset":27704,"ancestor_record_offset":26584,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":512,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":26584,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":28752,"parent_record_offset":28728,"ancestor_record_offset":26584,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":512,"record_offset":31320,"parent_record_offset":31296,"ancestor_record_offset":26584,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":512,"record_offset":31428,"parent_record_offset":31344,"ancestor_record_offset":26584,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":29488,"parent_record_offset":29388,"ancestor_record_offset":26584,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":29856,"parent_record_offset":29388,"ancestor_record_offset":26584,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37236,"parent_record_offset":37208,"ancestor_record_offset":26584,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":32268,"parent_record_offset":32240,"ancestor_record_offset":26584,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37264,"parent_record_offset":37236,"ancestor_record_offset":26584,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":29544,"parent_record_offset":29516,"ancestor_record_offset":26584,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":29516,"parent_record_offset":29488,"ancestor_record_offset":26584,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":512,"record_offset":29880,"parent_record_offset":29856,"ancestor_record_offset":26584,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":27680,"parent_record_offset":27656,"ancestor_record_offset":26584,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":27704,"parent_record_offset":27680,"ancestor_record_offset":26584,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":28008,"parent_record_offset":27980,"ancestor_record_offset":26584,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":31372,"parent_record_offset":31344,"ancestor_record_offset":26584,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":27656,"parent_record_offset":27540,"ancestor_record_offset":26584,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":512,"record_offset":38616,"parent_record_offset":38588,"ancestor_record_offset":26584,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":512,"record_offset":26968,"parent_record_offset":26940,"ancestor_record_offset":26584,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":512,"record_offset":27368,"parent_record_offset":27340,"ancestor_record_offset":26584,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":512,"record_offset":27508,"parent_record_offset":27476,"ancestor_record_offset":26584,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":512,"record_offset":27476,"parent_record_offset":27440,"ancestor_record_offset":26584,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":512,"record_offset":27416,"parent_record_offset":27392,"ancestor_record_offset":26584,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":512,"record_offset":27440,"parent_record_offset":27416,"ancestor_record_offset":26584,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":512,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":26584,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":512,"record_offset":27980,"parent_record_offset":27440,"ancestor_record_offset":26584,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":26940,"parent_record_offset":26904,"ancestor_record_offset":26584,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":26904,"parent_record_offset":26876,"ancestor_record_offset":26584,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":27100,"parent_record_offset":27068,"ancestor_record_offset":26584,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":27196,"parent_record_offset":27100,"ancestor_record_offset":26584,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":27068,"parent_record_offset":27044,"ancestor_record_offset":26584,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":26876,"parent_record_offset":26840,"ancestor_record_offset":26584,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":512,"record_offset":27220,"parent_record_offset":27196,"ancestor_record_offset":26584,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":512,"record_offset":27540,"parent_record_offset":27508,"ancestor_record_offset":26584,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":512,"record_offset":27824,"parent_record_offset":27540,"ancestor_record_offset":26584,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":512,"record_offset":27796,"parent_record_offset":27540,"ancestor_record_offset":26584,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":512,"record_offset":27600,"parent_record_offset":27540,"ancestor_record_offset":26584,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":35748,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":26825,"source_line":61,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":512,"record_offset":36980,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":29391,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":512,"record_offset":31452,"parent_record_offset":31428,"ancestor_record_offset":26584,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":512,"record_offset":37980,"parent_record_offset":37940,"ancestor_record_offset":26584,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":512,"record_offset":37940,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":29393,"source_line":60,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":37068,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":29392,"source_line":491,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":38780,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":29395,"source_line":471,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":512,"record_offset":32088,"parent_record_offset":31940,"ancestor_record_offset":26584,"inlinee":28131,"source_line":538,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":512,"record_offset":32208,"parent_record_offset":31908,"ancestor_record_offset":26584,"inlinee":29387,"source_line":521,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":512,"record_offset":31940,"parent_record_offset":31908,"ancestor_record_offset":26584,"inlinee":29386,"source_line":505,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":39816,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37908,"parent_record_offset":37068,"ancestor_record_offset":26584,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":39724,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37668,"parent_record_offset":37640,"ancestor_record_offset":26584,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38824,"parent_record_offset":38780,"ancestor_record_offset":26584,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37696,"parent_record_offset":37668,"ancestor_record_offset":26584,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":32120,"parent_record_offset":31940,"ancestor_record_offset":26584,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":32032,"parent_record_offset":31940,"ancestor_record_offset":26584,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37852,"parent_record_offset":37068,"ancestor_record_offset":26584,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38100,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":31976,"parent_record_offset":31940,"ancestor_record_offset":26584,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39452,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39996,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39252,"parent_record_offset":39216,"ancestor_record_offset":26584,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39216,"parent_record_offset":39180,"ancestor_record_offset":26584,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39180,"parent_record_offset":39152,"ancestor_record_offset":26584,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39152,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":33528,"parent_record_offset":33500,"ancestor_record_offset":26584,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":33676,"parent_record_offset":33500,"ancestor_record_offset":26584,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39064,"parent_record_offset":38160,"ancestor_record_offset":26584,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":38968,"parent_record_offset":38780,"ancestor_record_offset":26584,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":512,"record_offset":30796,"parent_record_offset":30756,"ancestor_record_offset":26584,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":512,"record_offset":30836,"parent_record_offset":30796,"ancestor_record_offset":26584,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":512,"record_offset":30944,"parent_record_offset":30756,"ancestor_record_offset":26584,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":512,"record_offset":30756,"parent_record_offset":30704,"ancestor_record_offset":26584,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":32676,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":30416,"parent_record_offset":30388,"ancestor_record_offset":26584,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":33288,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":30472,"parent_record_offset":30416,"ancestor_record_offset":26584,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":31036,"parent_record_offset":30704,"ancestor_record_offset":26584,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":512,"record_offset":32760,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":512,"record_offset":32648,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":512,"record_offset":33168,"parent_record_offset":33144,"ancestor_record_offset":26584,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":32432,"parent_record_offset":32408,"ancestor_record_offset":26584,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":32456,"parent_record_offset":32432,"ancestor_record_offset":26584,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":30280,"parent_record_offset":30256,"ancestor_record_offset":26584,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":33144,"parent_record_offset":32976,"ancestor_record_offset":26584,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":30644,"parent_record_offset":30620,"ancestor_record_offset":26584,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":512,"record_offset":32488,"parent_record_offset":32456,"ancestor_record_offset":26584,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":33084,"parent_record_offset":33000,"ancestor_record_offset":26584,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":33028,"parent_record_offset":33000,"ancestor_record_offset":26584,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":31136,"parent_record_offset":30704,"ancestor_record_offset":26584,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":33380,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30388,"parent_record_offset":30328,"ancestor_record_offset":26584,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30704,"parent_record_offset":30668,"ancestor_record_offset":26584,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":26584,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":32540,"parent_record_offset":32516,"ancestor_record_offset":26584,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":32516,"parent_record_offset":32456,"ancestor_record_offset":26584,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30304,"parent_record_offset":30280,"ancestor_record_offset":26584,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30360,"parent_record_offset":30328,"ancestor_record_offset":26584,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":32564,"parent_record_offset":32540,"ancestor_record_offset":26584,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30328,"parent_record_offset":30304,"ancestor_record_offset":26584,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":32620,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":30668,"parent_record_offset":30644,"ancestor_record_offset":26584,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":512,"record_offset":32976,"parent_record_offset":32564,"ancestor_record_offset":26584,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":28352,"parent_record_offset":28316,"ancestor_record_offset":26584,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":41996,"parent_record_offset":41964,"ancestor_record_offset":26584,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":42512,"parent_record_offset":42252,"ancestor_record_offset":26584,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":512,"record_offset":28496,"parent_record_offset":28468,"ancestor_record_offset":26584,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":28572,"parent_record_offset":28424,"ancestor_record_offset":26584,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":512,"record_offset":28832,"parent_record_offset":28572,"ancestor_record_offset":26584,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":512,"record_offset":32408,"parent_record_offset":32372,"ancestor_record_offset":26584,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":512,"record_offset":30256,"parent_record_offset":30232,"ancestor_record_offset":26584,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":512,"record_offset":30232,"parent_record_offset":30208,"ancestor_record_offset":26584,"inlinee":13598,"source_line":473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":512,"record_offset":30620,"parent_record_offset":30596,"ancestor_record_offset":26584,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":32176,"parent_record_offset":31940,"ancestor_record_offset":26584,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":39288,"parent_record_offset":39252,"ancestor_record_offset":26584,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":512,"record_offset":28524,"parent_record_offset":28496,"ancestor_record_offset":26584,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":512,"record_offset":28424,"parent_record_offset":28388,"ancestor_record_offset":26584,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":512,"record_offset":28388,"parent_record_offset":28352,"ancestor_record_offset":26584,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37148,"parent_record_offset":37116,"ancestor_record_offset":26584,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37344,"parent_record_offset":37308,"ancestor_record_offset":26584,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37308,"parent_record_offset":37068,"ancestor_record_offset":26584,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37116,"parent_record_offset":37068,"ancestor_record_offset":26584,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":33436,"parent_record_offset":32372,"ancestor_record_offset":26584,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":26584,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37504,"parent_record_offset":37344,"ancestor_record_offset":26584,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":37008,"parent_record_offset":35596,"ancestor_record_offset":26584,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":37640,"parent_record_offset":37344,"ancestor_record_offset":26584,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72CD
// 首条位置: {"module":512,"inlinee":29389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":512,"record_offset":38012,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":512,"record_offset":35748,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":26825,"source_line":61,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":512,"record_offset":36980,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":29391,"source_line":24,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":512,"record_offset":37940,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":29393,"source_line":60,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":37068,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":29392,"source_line":491,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38100,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":37008,"parent_record_offset":35596,"ancestor_record_offset":35596,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":35776,"parent_record_offset":35748,"ancestor_record_offset":35596,"inlinee":11299,"source_line":1653,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":35800,"parent_record_offset":35776,"ancestor_record_offset":35596,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":35996,"parent_record_offset":35800,"ancestor_record_offset":35596,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":35832,"parent_record_offset":35800,"ancestor_record_offset":35596,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":512,"record_offset":35860,"parent_record_offset":35832,"ancestor_record_offset":35596,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":35928,"parent_record_offset":35888,"ancestor_record_offset":35596,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":36020,"parent_record_offset":35996,"ancestor_record_offset":35596,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":35952,"parent_record_offset":35928,"ancestor_record_offset":35596,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35596,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":36044,"parent_record_offset":36020,"ancestor_record_offset":35596,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":37208,"parent_record_offset":37116,"ancestor_record_offset":35596,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":35596,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":35888,"parent_record_offset":35860,"ancestor_record_offset":35596,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37804,"parent_record_offset":37780,"ancestor_record_offset":35596,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37720,"parent_record_offset":37696,"ancestor_record_offset":35596,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":38124,"parent_record_offset":38100,"ancestor_record_offset":35596,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":35596,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":36116,"parent_record_offset":35748,"ancestor_record_offset":35596,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":512,"record_offset":38064,"parent_record_offset":38036,"ancestor_record_offset":35596,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37236,"parent_record_offset":37208,"ancestor_record_offset":35596,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37264,"parent_record_offset":37236,"ancestor_record_offset":35596,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":38036,"parent_record_offset":38012,"ancestor_record_offset":35596,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":512,"record_offset":37980,"parent_record_offset":37940,"ancestor_record_offset":35596,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37908,"parent_record_offset":37068,"ancestor_record_offset":35596,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37668,"parent_record_offset":37640,"ancestor_record_offset":35596,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37696,"parent_record_offset":37668,"ancestor_record_offset":35596,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37780,"parent_record_offset":37668,"ancestor_record_offset":35596,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37852,"parent_record_offset":37068,"ancestor_record_offset":35596,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37148,"parent_record_offset":37116,"ancestor_record_offset":35596,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37344,"parent_record_offset":37308,"ancestor_record_offset":35596,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37308,"parent_record_offset":37068,"ancestor_record_offset":35596,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37116,"parent_record_offset":37068,"ancestor_record_offset":35596,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":35596,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37504,"parent_record_offset":37344,"ancestor_record_offset":35596,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":37640,"parent_record_offset":37344,"ancestor_record_offset":35596,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72D2
// 首条位置: {"module":512,"inlinee":29394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":512,"record_offset":38560,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":38336,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":38532,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":39968,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":512,"record_offset":38448,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":40180,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":512,"record_offset":38364,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":512,"record_offset":38780,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":29395,"source_line":471,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":39816,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":39724,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":39388,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39452,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39996,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39152,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39064,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":38720,"parent_record_offset":38160,"ancestor_record_offset":38160,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":40364,"parent_record_offset":40340,"ancestor_record_offset":38160,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":40340,"parent_record_offset":40252,"ancestor_record_offset":38160,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40280,"parent_record_offset":40252,"ancestor_record_offset":38160,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40252,"parent_record_offset":40228,"ancestor_record_offset":38160,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":512,"record_offset":40228,"parent_record_offset":40204,"ancestor_record_offset":38160,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":512,"record_offset":38588,"parent_record_offset":38560,"ancestor_record_offset":38160,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":38472,"parent_record_offset":38448,"ancestor_record_offset":38160,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":38744,"parent_record_offset":38720,"ancestor_record_offset":38160,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":512,"record_offset":38652,"parent_record_offset":38616,"ancestor_record_offset":38160,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":512,"record_offset":38496,"parent_record_offset":38472,"ancestor_record_offset":38160,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":40308,"parent_record_offset":40280,"ancestor_record_offset":38160,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":39324,"parent_record_offset":39180,"ancestor_record_offset":38160,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":39000,"parent_record_offset":38968,"ancestor_record_offset":38160,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":38880,"parent_record_offset":38852,"ancestor_record_offset":38160,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":40204,"parent_record_offset":40180,"ancestor_record_offset":38160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":512,"record_offset":38412,"parent_record_offset":38388,"ancestor_record_offset":38160,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":512,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":38160,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":512,"record_offset":38616,"parent_record_offset":38588,"ancestor_record_offset":38160,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38940,"parent_record_offset":38780,"ancestor_record_offset":38160,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38824,"parent_record_offset":38780,"ancestor_record_offset":38160,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38852,"parent_record_offset":38780,"ancestor_record_offset":38160,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39252,"parent_record_offset":39216,"ancestor_record_offset":38160,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39216,"parent_record_offset":39180,"ancestor_record_offset":38160,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":39180,"parent_record_offset":39152,"ancestor_record_offset":38160,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":40020,"parent_record_offset":39996,"ancestor_record_offset":38160,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":38968,"parent_record_offset":38780,"ancestor_record_offset":38160,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":39288,"parent_record_offset":39252,"ancestor_record_offset":38160,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A7B
// 首条位置: {"module":1585,"inlinee":72315}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1585,"record_offset":106676,"parent_record_offset":106652,"ancestor_record_offset":106652,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1585,"record_offset":106704,"parent_record_offset":106676,"ancestor_record_offset":106652,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D56
// 首条位置: {"module":560,"inlinee":32086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":387444,"parent_record_offset":387408,"ancestor_record_offset":387408,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":387568,"parent_record_offset":387528,"ancestor_record_offset":387408,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":1585,"record_offset":387480,"parent_record_offset":387444,"ancestor_record_offset":387408,"inlinee":32088,"source_line":331,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387504,"parent_record_offset":387480,"ancestor_record_offset":387408,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387596,"parent_record_offset":387528,"ancestor_record_offset":387408,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387528,"parent_record_offset":387504,"ancestor_record_offset":387408,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1585,"record_offset":387632,"parent_record_offset":387596,"ancestor_record_offset":387408,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D58
// 首条位置: {"module":560,"inlinee":32088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387504,"parent_record_offset":387480,"ancestor_record_offset":387480,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":387568,"parent_record_offset":387528,"ancestor_record_offset":387480,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387596,"parent_record_offset":387528,"ancestor_record_offset":387480,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":387528,"parent_record_offset":387504,"ancestor_record_offset":387480,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1585,"record_offset":387632,"parent_record_offset":387596,"ancestor_record_offset":387480,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8037
// 首条位置: {"module":574,"inlinee":32823}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":574,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":7708,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::closest_segment_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":44776,"rva":9750528,"va":5378459648}
// 编译实例 RVAs: 0x94c800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33081751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378459648,"raw_hex":"e897c9f801","size":5,"soft_fail":false,"target_va":5411542064,"va":5378460308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3422963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378459648,"raw_hex":"e8f33a3400","size":5,"soft_fail":false,"target_va":5381883088,"va":5378460120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":45636,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":45380,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":512,"record_offset":44960,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45752,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45552,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45664,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45696,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":512,"record_offset":45412,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":512,"record_offset":45808,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":25768,"source_line":136,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":45092,"parent_record_offset":44776,"ancestor_record_offset":44776,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":512,"record_offset":45892,"parent_record_offset":45868,"ancestor_record_offset":44776,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":512,"record_offset":45916,"parent_record_offset":45892,"ancestor_record_offset":44776,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":45348,"parent_record_offset":45092,"ancestor_record_offset":44776,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":45520,"parent_record_offset":45412,"ancestor_record_offset":44776,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":45464,"parent_record_offset":45412,"ancestor_record_offset":44776,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":45148,"parent_record_offset":45124,"ancestor_record_offset":44776,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":512,"record_offset":45868,"parent_record_offset":45844,"ancestor_record_offset":44776,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":512,"record_offset":45844,"parent_record_offset":45808,"ancestor_record_offset":44776,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":512,"record_offset":45048,"parent_record_offset":44996,"ancestor_record_offset":44776,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":512,"record_offset":44996,"parent_record_offset":44960,"ancestor_record_offset":44776,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":512,"record_offset":45576,"parent_record_offset":45552,"ancestor_record_offset":44776,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45260,"parent_record_offset":45236,"ancestor_record_offset":44776,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":45124,"parent_record_offset":45092,"ancestor_record_offset":44776,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":45600,"parent_record_offset":45576,"ancestor_record_offset":44776,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":45236,"parent_record_offset":45092,"ancestor_record_offset":44776,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72D3
// 首条位置: {"module":512,"inlinee":29395}
// 编译实例 RVAs: 0x94caa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3473906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378460320,"raw_hex":"e8f2013500","size":5,"soft_fail":false,"target_va":5381934528,"va":5378460617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11645436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378460320,"raw_hex":"e8fcb1b100","size":5,"soft_fail":false,"target_va":5390105824,"va":5378460383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33081562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378460320,"raw_hex":"e8dac8f801","size":5,"soft_fail":false,"target_va":5411542296,"va":5378460729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38940,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38824,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":38852,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":512,"record_offset":38968,"parent_record_offset":38780,"ancestor_record_offset":38780,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46628,"parent_record_offset":45964,"ancestor_record_offset":45964,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46136,"parent_record_offset":45964,"ancestor_record_offset":45964,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":46232,"parent_record_offset":46136,"ancestor_record_offset":45964,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":47096,"parent_record_offset":47072,"ancestor_record_offset":45964,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":46556,"parent_record_offset":46428,"ancestor_record_offset":45964,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":39000,"parent_record_offset":38968,"ancestor_record_offset":38780,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":38880,"parent_record_offset":38852,"ancestor_record_offset":38780,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":46260,"parent_record_offset":46232,"ancestor_record_offset":45964,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":46288,"parent_record_offset":46260,"ancestor_record_offset":45964,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46488,"parent_record_offset":46456,"ancestor_record_offset":45964,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46456,"parent_record_offset":46428,"ancestor_record_offset":45964,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46428,"parent_record_offset":46404,"ancestor_record_offset":45964,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46960,"parent_record_offset":46932,"ancestor_record_offset":45964,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46404,"parent_record_offset":46380,"ancestor_record_offset":45964,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46988,"parent_record_offset":46960,"ancestor_record_offset":45964,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47072,"parent_record_offset":46960,"ancestor_record_offset":45964,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":46328,"parent_record_offset":46136,"ancestor_record_offset":45964,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":46520,"parent_record_offset":46488,"ancestor_record_offset":45964,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46176,"parent_record_offset":46136,"ancestor_record_offset":45964,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46656,"parent_record_offset":46628,"ancestor_record_offset":45964,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46200,"parent_record_offset":46176,"ancestor_record_offset":45964,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":46808,"parent_record_offset":46656,"ancestor_record_offset":45964,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":46932,"parent_record_offset":46656,"ancestor_record_offset":45964,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":46380,"parent_record_offset":46136,"ancestor_record_offset":45964,"inlinee":25387,"source_line":68,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 491
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x72D0
// 首条位置: {"module":512,"inlinee":29392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37908,"parent_record_offset":37068,"ancestor_record_offset":37068,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37852,"parent_record_offset":37068,"ancestor_record_offset":37068,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37308,"parent_record_offset":37068,"ancestor_record_offset":37068,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37116,"parent_record_offset":37068,"ancestor_record_offset":37068,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":37208,"parent_record_offset":37116,"ancestor_record_offset":37068,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37804,"parent_record_offset":37780,"ancestor_record_offset":37068,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37720,"parent_record_offset":37696,"ancestor_record_offset":37068,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":37876,"parent_record_offset":37852,"ancestor_record_offset":37068,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37236,"parent_record_offset":37208,"ancestor_record_offset":37068,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":37264,"parent_record_offset":37236,"ancestor_record_offset":37068,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37668,"parent_record_offset":37640,"ancestor_record_offset":37068,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37696,"parent_record_offset":37668,"ancestor_record_offset":37068,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":37780,"parent_record_offset":37668,"ancestor_record_offset":37068,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37148,"parent_record_offset":37116,"ancestor_record_offset":37068,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37344,"parent_record_offset":37308,"ancestor_record_offset":37068,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":37068,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":512,"record_offset":37504,"parent_record_offset":37344,"ancestor_record_offset":37068,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":37640,"parent_record_offset":37344,"ancestor_record_offset":37068,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 513
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":47332,"rva":9751648,"va":5378460768}
// 编译实例 RVAs: 0x94cc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3473331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378460768,"raw_hex":"e8b3ff3400","size":5,"soft_fail":false,"target_va":5381934528,"va":5378461192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::intersection_aabb_segment2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9387382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378460768,"raw_hex":"e8763d8f00","size":5,"soft_fail":false,"target_va":5387848496,"va":5378461109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":512,"record_offset":48260,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":512,"record_offset":47872,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":25976,"source_line":60,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":512,"record_offset":47956,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":48528,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49612,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":29397,"source_line":429,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47696,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":48412,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47844,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49388,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47540,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47644,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47592,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":48956,"parent_record_offset":47332,"ancestor_record_offset":47332,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":512,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":47332,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":48048,"parent_record_offset":47956,"ancestor_record_offset":47332,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":48744,"parent_record_offset":48688,"ancestor_record_offset":47332,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":47988,"parent_record_offset":47956,"ancestor_record_offset":47332,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":512,"record_offset":48016,"parent_record_offset":47956,"ancestor_record_offset":47332,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":49156,"parent_record_offset":48956,"ancestor_record_offset":47332,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":49124,"parent_record_offset":48984,"ancestor_record_offset":47332,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":512,"record_offset":48464,"parent_record_offset":48436,"ancestor_record_offset":47332,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":47564,"parent_record_offset":47540,"ancestor_record_offset":47332,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":48492,"parent_record_offset":48436,"ancestor_record_offset":47332,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":47616,"parent_record_offset":47592,"ancestor_record_offset":47332,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":512,"record_offset":47668,"parent_record_offset":47644,"ancestor_record_offset":47332,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":512,"record_offset":48636,"parent_record_offset":48528,"ancestor_record_offset":47332,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":48836,"parent_record_offset":48636,"ancestor_record_offset":47332,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":48600,"parent_record_offset":48576,"ancestor_record_offset":47332,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":512,"record_offset":48576,"parent_record_offset":48552,"ancestor_record_offset":47332,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":47720,"parent_record_offset":47696,"ancestor_record_offset":47332,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":48436,"parent_record_offset":48412,"ancestor_record_offset":47332,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49212,"parent_record_offset":49188,"ancestor_record_offset":47332,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49236,"parent_record_offset":49212,"ancestor_record_offset":47332,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49320,"parent_record_offset":49212,"ancestor_record_offset":47332,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":512,"record_offset":49012,"parent_record_offset":48984,"ancestor_record_offset":47332,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":48688,"parent_record_offset":48664,"ancestor_record_offset":47332,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":512,"record_offset":48664,"parent_record_offset":48636,"ancestor_record_offset":47332,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":512,"record_offset":47752,"parent_record_offset":47720,"ancestor_record_offset":47332,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":49068,"parent_record_offset":48984,"ancestor_record_offset":47332,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":49188,"parent_record_offset":48956,"ancestor_record_offset":47332,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":512,"record_offset":48984,"parent_record_offset":48956,"ancestor_record_offset":47332,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00512.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_grower.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_grower.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_grower.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::history::replay_mode::HoldToFastReplay> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 信息 ID: 67337cccb5ec37603ac70538
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=38160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 信息 ID: 3b8891e82b52b5ed9576a243
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0 <- alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; module=1585, inline_record=106652; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; module=1585, inline_record=106860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 信息 ID: 1d5ba75459058209d4ed4e37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1585, inline_record=387408; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1585, inline_record=388280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 信息 ID: f1852bac667a0ab5fa4da083
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1585, inline_record=387480; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1585, inline_record=388344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 信息 ID: c66e0ee6c5c344b3fc2f2481
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::driftsort_main<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::vec::Vec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::drift::sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=574, inline_record=7708; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=574, inline_record=8024; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=13308; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=13408; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=13604; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=13724; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=13912; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1592, inline_record=14248; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::closest_segment_index
// 信息 ID: c15e4e70f1be9f5f01e1567c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x94b06a -> 0x94c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0x94d203 -> 0x94c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0x998f27 -> 0x94c800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 信息 ID: 40463cc59333fb377c47e490
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=35596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 信息 ID: f633dbd6e2263037301d0bdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=37068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 信息 ID: da574126ad75c18cef766ba2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0x94beaa -> 0x94caa0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=512, inline_record=38780; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment
// 信息 ID: f041616a858e48f36396f9fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0x1308501 -> 0x94cc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0x13085fa -> 0x94cc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::ivy_grower
// 信息 ID: be291a557745257d163d52b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 58774f2059b3dde230196834: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)> > @ 0xa9d5db -> 0x94a910 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::reader::EventReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)> > > @ 0x14d6462 -> 0x94a910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
