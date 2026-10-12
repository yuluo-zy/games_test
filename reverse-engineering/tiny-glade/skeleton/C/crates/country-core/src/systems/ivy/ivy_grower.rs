// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ivy/ivy_grower.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cedf1a957c3846779d1cf4bc630a14ecad4765f70e7c76c9c395cc898e86f8e8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_grower.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 580, 1639, 1643, 1713, 1813, 1844, 3393
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 首条位置: {"module":580,"record_offset":8952,"rva":12314336,"va":5381023456}
// 编译实例 RVAs: 0xbbe6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-5349687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8c95eaeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381028386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12304466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8ae3f44ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381026653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7546258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e86eda8cff","size":5,"soft_fail":false,"target_va":5373479040,"va":5381025293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2297842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8f20f2300","size":5,"soft_fail":false,"target_va":5383325024,"va":5381027177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152da60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9887160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8b8dd9600","size":5,"soft_fail":false,"target_va":5390916192,"va":5381029027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f4620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10703266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8a251a300","size":5,"soft_fail":false,"target_va":5391730208,"va":5381026937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416045b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10771286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8565ba400","size":5,"soft_fail":false,"target_va":5391795632,"va":5381024341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141614070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10832803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8a34ba500","size":5,"soft_fail":false,"target_va":5391859824,"va":5381027016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11750606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8ce4cb300","size":5,"soft_fail":false,"target_va":5392780768,"va":5381030157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196e3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14347725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8cdedda00","size":5,"soft_fail":false,"target_va":5395375056,"va":5381027326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f620
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14353615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8cf04db00","size":5,"soft_fail":false,"target_va":5395379744,"va":5381026124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14356660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8b410db00","size":5,"soft_fail":false,"target_va":5395383936,"va":5381027271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419707a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14357681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8b114db00","size":5,"soft_fail":false,"target_va":5395384224,"va":5381026538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970d30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14359656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8681cdb00","size":5,"soft_fail":false,"target_va":5395385648,"va":5381025987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419734b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14367920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8b03cdb00","size":5,"soft_fail":false,"target_va":5395395760,"va":5381027835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35322681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e839fb1a02","size":5,"soft_fail":false,"target_va":5416350384,"va":5381027698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35355560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8a87b1b02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381029459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36032809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e829d12502","size":5,"soft_fail":false,"target_va":5417057792,"va":5381024978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35591985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e831171f02","size":5,"soft_fail":false,"target_va":5416621408,"va":5381029418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35592195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e803181f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5381029400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35592760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8381a1f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5381029443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35592159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8df171f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5381029598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t843679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e89fdf0c00","size":5,"soft_fail":false,"target_va":5381872736,"va":5381029052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e827810000","size":5,"soft_fail":false,"target_va":5381056704,"va":5381023636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35591735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e837161f02","size":5,"soft_fail":false,"target_va":5416621376,"va":5381029636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer::get_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1426813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8833aeaff","size":5,"soft_fail":false,"target_va":5379599072,"va":5381025880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::add_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1742380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8d469e5ff","size":5,"soft_fail":false,"target_va":5379286528,"va":5381028903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1740492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e83471e5ff","size":5,"soft_fail":false,"target_va":5379286032,"va":5381026519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::Ivy::last_segment_point_ids
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1741538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e81e6de5ff","size":5,"soft_fail":false,"target_va":5379286112,"va":5381027645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::wall_coord::WallCoord::into_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-124793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e88718feff","size":5,"soft_fail":false,"target_va":5380901776,"va":5381026564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-123581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8431dfeff","size":5,"soft_fail":false,"target_va":5380903536,"va":5381027112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::PrevWallHoles::wall_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t582762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e86ae40800","size":5,"soft_fail":false,"target_va":5381609344,"va":5381026577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::closest_segment_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e869170000","size":5,"soft_fail":false,"target_va":5381031184,"va":5381025186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e82d0c0000","size":5,"soft_fail":false,"target_va":5381031824,"va":5381028702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1035006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e80235f0ff","size":5,"soft_fail":false,"target_va":5379991920,"va":5381026921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2939757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e86ddb2c00","size":5,"soft_fail":false,"target_va":5383963792,"va":5381024030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2948264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8a8fc2c00","size":5,"soft_fail":false,"target_va":5383972224,"va":5381023955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2935718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8a6cb2c00","size":5,"soft_fail":false,"target_va":5383964912,"va":5381029189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2939219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e853d92c00","size":5,"soft_fail":false,"target_va":5383962816,"va":5381023592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35589773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e88d0e1f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5381029582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2275550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381023456,"raw_hex":"e8deb82200","size":5,"soft_fail":false,"target_va":5383302304,"va":5381026749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":12288,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":12316,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":580,"record_offset":11464,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":580,"record_offset":22552,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":21860,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":580,"record_offset":12692,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":12232,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":12176,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":25992,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":23536,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":13264,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":14516,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":580,"record_offset":10616,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":29411,"source_line":260,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":13492,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":32056,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":13464,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":31954,"source_line":57,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::set_last_point_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":22164,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":32060,"source_line":65,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":12372,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":33008,"source_line":176,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":580,"record_offset":22936,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":33009,"source_line":60,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":580,"record_offset":14544,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::check_for_wall_jump
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":19144,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":36974,"source_line":308,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_growth_point_proposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":16588,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":36972,"source_line":227,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":13876,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":33015,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":23324,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":21892,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":23064,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":10656,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":9212,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":9604,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$55::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":12720,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":21910,"source_line":2271,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":16560,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":23008,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::is_within_01_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":22908,"parent_record_offset":8952,"ancestor_record_offset":8952,"inlinee":36976,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":8952,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":8952,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21512,"parent_record_offset":21488,"ancestor_record_offset":8952,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21440,"parent_record_offset":21348,"ancestor_record_offset":8952,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":8952,"inlinee":13892,"source_line":2019,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":8952,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":22216,"parent_record_offset":22192,"ancestor_record_offset":8952,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":13740,"parent_record_offset":13540,"ancestor_record_offset":8952,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":13576,"parent_record_offset":13540,"ancestor_record_offset":8952,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":13604,"parent_record_offset":13576,"ancestor_record_offset":8952,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":22192,"parent_record_offset":22164,"ancestor_record_offset":8952,"inlinee":31970,"source_line":3634,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":13672,"parent_record_offset":13632,"ancestor_record_offset":8952,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":13776,"parent_record_offset":13740,"ancestor_record_offset":8952,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":8952,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21376,"parent_record_offset":21348,"ancestor_record_offset":8952,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":8952,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":8952,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":8952,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":8952,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":580,"record_offset":19548,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":580,"record_offset":19576,"parent_record_offset":19548,"ancestor_record_offset":8952,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":22740,"parent_record_offset":22700,"ancestor_record_offset":8952,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":8952,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":19324,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":19520,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":22672,"parent_record_offset":22636,"ancestor_record_offset":8952,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":22700,"parent_record_offset":22672,"ancestor_record_offset":8952,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":18212,"parent_record_offset":18120,"ancestor_record_offset":8952,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":12256,"parent_record_offset":12232,"ancestor_record_offset":8952,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":11780,"parent_record_offset":11552,"ancestor_record_offset":8952,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":8952,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":22576,"parent_record_offset":22552,"ancestor_record_offset":8952,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":580,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":8952,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":580,"record_offset":11528,"parent_record_offset":11488,"ancestor_record_offset":8952,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":580,"record_offset":22636,"parent_record_offset":22576,"ancestor_record_offset":8952,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":580,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":8952,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":10984,"parent_record_offset":10956,"ancestor_record_offset":8952,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":11012,"parent_record_offset":10984,"ancestor_record_offset":8952,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":10932,"parent_record_offset":10904,"ancestor_record_offset":8952,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":8952,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":11072,"parent_record_offset":10956,"ancestor_record_offset":8952,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":580,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":8952,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":15280,"parent_record_offset":15256,"ancestor_record_offset":8952,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":8952,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":14848,"parent_record_offset":14824,"ancestor_record_offset":8952,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":8952,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":8952,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":21036,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":8952,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":8952,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":15348,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":580,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":8952,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":580,"record_offset":19000,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":580,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":8952,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":19436,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":580,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":8952,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":11736,"parent_record_offset":11708,"ancestor_record_offset":8952,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":11596,"parent_record_offset":11552,"ancestor_record_offset":8952,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":21408,"parent_record_offset":21376,"ancestor_record_offset":8952,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":13632,"parent_record_offset":13604,"ancestor_record_offset":8952,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":12976,"parent_record_offset":12952,"ancestor_record_offset":8952,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":8952,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":14080,"parent_record_offset":14048,"ancestor_record_offset":8952,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":8952,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":8952,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19984,"parent_record_offset":19956,"ancestor_record_offset":8952,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":8952,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18860,"parent_record_offset":18836,"ancestor_record_offset":8952,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":580,"record_offset":10804,"parent_record_offset":10764,"ancestor_record_offset":8952,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":14480,"parent_record_offset":13876,"ancestor_record_offset":8952,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":17120,"parent_record_offset":16736,"ancestor_record_offset":8952,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":580,"record_offset":9552,"parent_record_offset":9524,"ancestor_record_offset":8952,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":580,"record_offset":11420,"parent_record_offset":10764,"ancestor_record_offset":8952,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":8952,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":11136,"parent_record_offset":10904,"ancestor_record_offset":8952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":580,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":8952,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":580,"record_offset":13128,"parent_record_offset":13104,"ancestor_record_offset":8952,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":8952,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":8952,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":8952,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":22860,"parent_record_offset":22700,"ancestor_record_offset":8952,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":580,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":8952,"inlinee":12688,"source_line":375,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$17::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":580,"record_offset":13396,"parent_record_offset":13312,"ancestor_record_offset":8952,"inlinee":12690,"source_line":376,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":580,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":8952,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":8952,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":11652,"parent_record_offset":11552,"ancestor_record_offset":8952,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":12020,"parent_record_offset":11552,"ancestor_record_offset":8952,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":8952,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":14444,"parent_record_offset":14416,"ancestor_record_offset":8952,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18268,"parent_record_offset":18240,"ancestor_record_offset":8952,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":11708,"parent_record_offset":11680,"ancestor_record_offset":8952,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":8952,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":580,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":8952,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8952,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":8952,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":10336,"parent_record_offset":10308,"ancestor_record_offset":8952,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":13340,"parent_record_offset":13312,"ancestor_record_offset":8952,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":9928,"parent_record_offset":9812,"ancestor_record_offset":8952,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":580,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":8952,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":580,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":8952,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":580,"record_offset":9640,"parent_record_offset":9604,"ancestor_record_offset":8952,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":580,"record_offset":9780,"parent_record_offset":9748,"ancestor_record_offset":8952,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":580,"record_offset":9748,"parent_record_offset":9712,"ancestor_record_offset":8952,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":580,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":8952,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":580,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":8952,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":580,"record_offset":9664,"parent_record_offset":9640,"ancestor_record_offset":8952,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":580,"record_offset":10252,"parent_record_offset":9712,"ancestor_record_offset":8952,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9316,"parent_record_offset":9284,"ancestor_record_offset":8952,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":8952,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9444,"parent_record_offset":9412,"ancestor_record_offset":8952,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9500,"parent_record_offset":9444,"ancestor_record_offset":8952,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9412,"parent_record_offset":9380,"ancestor_record_offset":8952,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9256,"parent_record_offset":9212,"ancestor_record_offset":8952,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":580,"record_offset":9524,"parent_record_offset":9500,"ancestor_record_offset":8952,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":580,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":8952,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":580,"record_offset":10096,"parent_record_offset":9812,"ancestor_record_offset":8952,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":580,"record_offset":10068,"parent_record_offset":9812,"ancestor_record_offset":8952,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":580,"record_offset":9872,"parent_record_offset":9812,"ancestor_record_offset":8952,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":16736,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":32056,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":580,"record_offset":17984,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":36742,"source_line":24,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":580,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":8952,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":580,"record_offset":18968,"parent_record_offset":18924,"ancestor_record_offset":8952,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":580,"record_offset":18924,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":33009,"source_line":60,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":18072,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":36973,"source_line":491,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":19768,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":36975,"source_line":471,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":14140,"parent_record_offset":13916,"ancestor_record_offset":8952,"inlinee":33017,"source_line":538,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":14384,"parent_record_offset":13876,"ancestor_record_offset":8952,"inlinee":33018,"source_line":521,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":13916,"parent_record_offset":13876,"ancestor_record_offset":8952,"inlinee":33016,"source_line":505,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":20856,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18892,"parent_record_offset":18072,"ancestor_record_offset":8952,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":20736,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18652,"parent_record_offset":18624,"ancestor_record_offset":8952,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19812,"parent_record_offset":19768,"ancestor_record_offset":8952,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":8952,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":14180,"parent_record_offset":13916,"ancestor_record_offset":8952,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":14048,"parent_record_offset":13916,"ancestor_record_offset":8952,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18836,"parent_record_offset":18072,"ancestor_record_offset":8952,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19084,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":13960,"parent_record_offset":13916,"ancestor_record_offset":8952,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20428,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":21064,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20268,"parent_record_offset":20236,"ancestor_record_offset":8952,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20236,"parent_record_offset":20204,"ancestor_record_offset":8952,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20204,"parent_record_offset":20176,"ancestor_record_offset":8952,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20176,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":21920,"parent_record_offset":21892,"ancestor_record_offset":8952,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":22068,"parent_record_offset":21892,"ancestor_record_offset":8952,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20020,"parent_record_offset":19144,"ancestor_record_offset":8952,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":19956,"parent_record_offset":19768,"ancestor_record_offset":8952,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":12896,"parent_record_offset":12864,"ancestor_record_offset":8952,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":12928,"parent_record_offset":12896,"ancestor_record_offset":8952,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":13016,"parent_record_offset":12864,"ancestor_record_offset":8952,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":12864,"parent_record_offset":12824,"ancestor_record_offset":8952,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":14824,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":12580,"parent_record_offset":12552,"ancestor_record_offset":8952,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":15376,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":14908,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":580,"record_offset":14768,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":580,"record_offset":15256,"parent_record_offset":15232,"ancestor_record_offset":8952,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":8952,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":12444,"parent_record_offset":12420,"ancestor_record_offset":8952,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":15232,"parent_record_offset":15124,"ancestor_record_offset":8952,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":8952,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":14636,"parent_record_offset":14604,"ancestor_record_offset":8952,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":8952,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":13104,"parent_record_offset":12824,"ancestor_record_offset":8952,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":15468,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12552,"parent_record_offset":12492,"ancestor_record_offset":8952,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":8952,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":8952,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":8952,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":14664,"parent_record_offset":14604,"ancestor_record_offset":8952,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12468,"parent_record_offset":12444,"ancestor_record_offset":8952,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12524,"parent_record_offset":12492,"ancestor_record_offset":8952,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":8952,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12492,"parent_record_offset":12468,"ancestor_record_offset":8952,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":14796,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":8952,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":15124,"parent_record_offset":14712,"ancestor_record_offset":8952,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":10692,"parent_record_offset":10656,"ancestor_record_offset":8952,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":24184,"parent_record_offset":24152,"ancestor_record_offset":8952,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":24636,"parent_record_offset":24432,"ancestor_record_offset":8952,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":580,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":8952,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":10904,"parent_record_offset":10764,"ancestor_record_offset":8952,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":580,"record_offset":11108,"parent_record_offset":10904,"ancestor_record_offset":8952,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":14580,"parent_record_offset":14544,"ancestor_record_offset":8952,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":12420,"parent_record_offset":12396,"ancestor_record_offset":8952,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":8952,"inlinee":20111,"source_line":576,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":8952,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":14268,"parent_record_offset":13916,"ancestor_record_offset":8952,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":20300,"parent_record_offset":20268,"ancestor_record_offset":8952,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":580,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":8952,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":580,"record_offset":10764,"parent_record_offset":10728,"ancestor_record_offset":8952,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":580,"record_offset":10728,"parent_record_offset":10692,"ancestor_record_offset":8952,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18152,"parent_record_offset":18120,"ancestor_record_offset":8952,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18344,"parent_record_offset":18312,"ancestor_record_offset":8952,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18312,"parent_record_offset":18072,"ancestor_record_offset":8952,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18120,"parent_record_offset":18072,"ancestor_record_offset":8952,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":15520,"parent_record_offset":14544,"ancestor_record_offset":8952,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":8952,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18500,"parent_record_offset":18344,"ancestor_record_offset":8952,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":18012,"parent_record_offset":16588,"ancestor_record_offset":8952,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":18624,"parent_record_offset":18344,"ancestor_record_offset":8952,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x906C
// 首条位置: {"module":580,"inlinee":36972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":580,"record_offset":19000,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::last_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":16736,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":32056,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::wall_coord::WallCoord::from_wall_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":580,"record_offset":17984,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":36742,"source_line":24,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/wall_coord.rs
//     evidence={"module":580,"record_offset":18924,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":33009,"source_line":60,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":18072,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":36973,"source_line":491,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19084,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":11675,"source_line":1563,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":18012,"parent_record_offset":16588,"ancestor_record_offset":16588,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":16588,"inlinee":13892,"source_line":2019,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":16588,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":16988,"parent_record_offset":16788,"ancestor_record_offset":16588,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":16824,"parent_record_offset":16788,"ancestor_record_offset":16588,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":580,"record_offset":16852,"parent_record_offset":16824,"ancestor_record_offset":16588,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":16920,"parent_record_offset":16880,"ancestor_record_offset":16588,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":17024,"parent_record_offset":16988,"ancestor_record_offset":16588,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":16944,"parent_record_offset":16920,"ancestor_record_offset":16588,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":17072,"parent_record_offset":17048,"ancestor_record_offset":16588,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":16588,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":18212,"parent_record_offset":18120,"ancestor_record_offset":16588,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":16588,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":16588,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18788,"parent_record_offset":18764,"ancestor_record_offset":16588,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":16588,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":16588,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18860,"parent_record_offset":18836,"ancestor_record_offset":16588,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":17120,"parent_record_offset":16736,"ancestor_record_offset":16588,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":19048,"parent_record_offset":19024,"ancestor_record_offset":16588,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":16588,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18268,"parent_record_offset":18240,"ancestor_record_offset":16588,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":19024,"parent_record_offset":19000,"ancestor_record_offset":16588,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":580,"record_offset":18968,"parent_record_offset":18924,"ancestor_record_offset":16588,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18892,"parent_record_offset":18072,"ancestor_record_offset":16588,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18652,"parent_record_offset":18624,"ancestor_record_offset":16588,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":16588,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18764,"parent_record_offset":18652,"ancestor_record_offset":16588,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18836,"parent_record_offset":18072,"ancestor_record_offset":16588,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18152,"parent_record_offset":18120,"ancestor_record_offset":16588,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18344,"parent_record_offset":18312,"ancestor_record_offset":16588,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18312,"parent_record_offset":18072,"ancestor_record_offset":16588,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18120,"parent_record_offset":18072,"ancestor_record_offset":16588,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":16588,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18500,"parent_record_offset":18344,"ancestor_record_offset":16588,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":18624,"parent_record_offset":18344,"ancestor_record_offset":16588,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x906E
// 首条位置: {"module":580,"inlinee":36974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":580,"record_offset":19548,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":19324,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":19520,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":21036,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":19436,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":21276,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":19352,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":580,"record_offset":19768,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":36975,"source_line":471,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":20856,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":20736,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":20112,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20428,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":21064,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20176,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20020,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":19708,"parent_record_offset":19144,"ancestor_record_offset":19144,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":19144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":19144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21512,"parent_record_offset":21488,"ancestor_record_offset":19144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":21440,"parent_record_offset":21348,"ancestor_record_offset":19144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21376,"parent_record_offset":21348,"ancestor_record_offset":19144,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":19144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":19144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":580,"record_offset":19576,"parent_record_offset":19548,"ancestor_record_offset":19144,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":19144,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":19144,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":19144,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":19640,"parent_record_offset":19604,"ancestor_record_offset":19144,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":580,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":19144,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":21408,"parent_record_offset":21376,"ancestor_record_offset":19144,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":20364,"parent_record_offset":20204,"ancestor_record_offset":19144,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19984,"parent_record_offset":19956,"ancestor_record_offset":19144,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19868,"parent_record_offset":19840,"ancestor_record_offset":19144,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":19144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":580,"record_offset":19400,"parent_record_offset":19376,"ancestor_record_offset":19144,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":580,"record_offset":19376,"parent_record_offset":19352,"ancestor_record_offset":19144,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":580,"record_offset":19604,"parent_record_offset":19576,"ancestor_record_offset":19144,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19928,"parent_record_offset":19768,"ancestor_record_offset":19144,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19812,"parent_record_offset":19768,"ancestor_record_offset":19144,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19840,"parent_record_offset":19768,"ancestor_record_offset":19144,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20268,"parent_record_offset":20236,"ancestor_record_offset":19144,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20236,"parent_record_offset":20204,"ancestor_record_offset":19144,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":20204,"parent_record_offset":20176,"ancestor_record_offset":19144,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":19144,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":19956,"parent_record_offset":19768,"ancestor_record_offset":19144,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":20300,"parent_record_offset":20268,"ancestor_record_offset":19144,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D54
// 首条位置: {"module":1643,"inlinee":81236}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":209384,"parent_record_offset":209364,"ancestor_record_offset":209364,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1643,"record_offset":209408,"parent_record_offset":209384,"ancestor_record_offset":209364,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DE2
// 首条位置: {"module":1643,"inlinee":81378}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":724576,"parent_record_offset":724536,"ancestor_record_offset":724536,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":724788,"parent_record_offset":724764,"ancestor_record_offset":724536,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":724700,"parent_record_offset":724660,"ancestor_record_offset":724536,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_grower.rs
//     evidence={"module":1643,"record_offset":724612,"parent_record_offset":724576,"ancestor_record_offset":724536,"inlinee":81380,"source_line":331,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724636,"parent_record_offset":724612,"ancestor_record_offset":724536,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724728,"parent_record_offset":724660,"ancestor_record_offset":724536,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724660,"parent_record_offset":724636,"ancestor_record_offset":724536,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1643,"record_offset":724764,"parent_record_offset":724728,"ancestor_record_offset":724536,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DE4
// 首条位置: {"module":1643,"inlinee":81380}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724636,"parent_record_offset":724612,"ancestor_record_offset":724612,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":724788,"parent_record_offset":724764,"ancestor_record_offset":724612,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":724700,"parent_record_offset":724660,"ancestor_record_offset":724612,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724728,"parent_record_offset":724660,"ancestor_record_offset":724612,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":724660,"parent_record_offset":724636,"ancestor_record_offset":724612,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1643,"record_offset":724764,"parent_record_offset":724728,"ancestor_record_offset":724612,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B4A
// 首条位置: {"module":528,"inlinee":31562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":39692,"parent_record_offset":39668,"ancestor_record_offset":39668,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00528.jsonl.gz"}
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
// 首条位置: {"module":580,"record_offset":26900,"rva":12322064,"va":5381031184}
// 编译实例 RVAs: 0xbc0510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35590390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381031184,"raw_hex":"e8f6101f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5381031813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2249912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381031184,"raw_hex":"e8b8542200","size":5,"soft_fail":false,"target_va":5383281568,"va":5381031651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":27832,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":27548,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":580,"record_offset":27084,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27892,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27720,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27860,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27196,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":27580,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":27948,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":27260,"parent_record_offset":26900,"ancestor_record_offset":26900,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":28032,"parent_record_offset":28008,"ancestor_record_offset":26900,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":26900,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":27516,"parent_record_offset":27260,"ancestor_record_offset":26900,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":27792,"parent_record_offset":27768,"ancestor_record_offset":26900,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":27688,"parent_record_offset":27580,"ancestor_record_offset":26900,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":27632,"parent_record_offset":27580,"ancestor_record_offset":26900,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":27224,"parent_record_offset":27196,"ancestor_record_offset":26900,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":26900,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":27984,"parent_record_offset":27948,"ancestor_record_offset":26900,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":27152,"parent_record_offset":27120,"ancestor_record_offset":26900,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":27120,"parent_record_offset":27084,"ancestor_record_offset":26900,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":580,"record_offset":27744,"parent_record_offset":27720,"ancestor_record_offset":26900,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27428,"parent_record_offset":27404,"ancestor_record_offset":26900,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":27292,"parent_record_offset":27260,"ancestor_record_offset":26900,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":27768,"parent_record_offset":27744,"ancestor_record_offset":26900,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":27404,"parent_record_offset":27260,"ancestor_record_offset":26900,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x906F
// 首条位置: {"module":580,"inlinee":36975}
// 编译实例 RVAs: 0xbc0790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2292898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381031824,"raw_hex":"e8a2fc2200","size":5,"soft_fail":false,"target_va":5383325024,"va":5381032121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f620
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14347852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381031824,"raw_hex":"e84ceeda00","size":5,"soft_fail":false,"target_va":5395379744,"va":5381031887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35589529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381031824,"raw_hex":"e8990d1f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5381032228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19928,"parent_record_offset":19768,"ancestor_record_offset":19768,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19812,"parent_record_offset":19768,"ancestor_record_offset":19768,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":19840,"parent_record_offset":19768,"ancestor_record_offset":19768,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":19956,"parent_record_offset":19768,"ancestor_record_offset":19768,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28796,"parent_record_offset":28104,"ancestor_record_offset":28104,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28276,"parent_record_offset":28104,"ancestor_record_offset":28104,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":28372,"parent_record_offset":28276,"ancestor_record_offset":28104,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":28684,"parent_record_offset":28660,"ancestor_record_offset":28104,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":29264,"parent_record_offset":29240,"ancestor_record_offset":28104,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":28724,"parent_record_offset":28568,"ancestor_record_offset":28104,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19984,"parent_record_offset":19956,"ancestor_record_offset":19768,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":19868,"parent_record_offset":19840,"ancestor_record_offset":19768,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":28400,"parent_record_offset":28372,"ancestor_record_offset":28104,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":28428,"parent_record_offset":28400,"ancestor_record_offset":28104,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":28628,"parent_record_offset":28596,"ancestor_record_offset":28104,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":28596,"parent_record_offset":28568,"ancestor_record_offset":28104,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":28104,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28104,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":28544,"parent_record_offset":28520,"ancestor_record_offset":28104,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29156,"parent_record_offset":29128,"ancestor_record_offset":28104,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29240,"parent_record_offset":29128,"ancestor_record_offset":28104,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":28468,"parent_record_offset":28276,"ancestor_record_offset":28104,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":28660,"parent_record_offset":28628,"ancestor_record_offset":28104,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28316,"parent_record_offset":28276,"ancestor_record_offset":28104,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28824,"parent_record_offset":28796,"ancestor_record_offset":28104,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28340,"parent_record_offset":28316,"ancestor_record_offset":28104,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":28976,"parent_record_offset":28824,"ancestor_record_offset":28104,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":29100,"parent_record_offset":28824,"ancestor_record_offset":28104,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":28520,"parent_record_offset":28276,"ancestor_record_offset":28104,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 491
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x906D
// 首条位置: {"module":580,"inlinee":36973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18892,"parent_record_offset":18072,"ancestor_record_offset":18072,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18836,"parent_record_offset":18072,"ancestor_record_offset":18072,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18312,"parent_record_offset":18072,"ancestor_record_offset":18072,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18120,"parent_record_offset":18072,"ancestor_record_offset":18072,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":18212,"parent_record_offset":18120,"ancestor_record_offset":18072,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18788,"parent_record_offset":18764,"ancestor_record_offset":18072,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":18072,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":18860,"parent_record_offset":18836,"ancestor_record_offset":18072,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":18072,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":18268,"parent_record_offset":18240,"ancestor_record_offset":18072,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18652,"parent_record_offset":18624,"ancestor_record_offset":18072,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18680,"parent_record_offset":18652,"ancestor_record_offset":18072,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":18764,"parent_record_offset":18652,"ancestor_record_offset":18072,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18152,"parent_record_offset":18120,"ancestor_record_offset":18072,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18344,"parent_record_offset":18312,"ancestor_record_offset":18072,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":18072,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":18500,"parent_record_offset":18344,"ancestor_record_offset":18072,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":18624,"parent_record_offset":18344,"ancestor_record_offset":18072,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
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
// 首条位置: {"module":580,"record_offset":29460,"rva":12323152,"va":5381032272}
// 编译实例 RVAs: 0xbc0950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_grower.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2292336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381032272,"raw_hex":"e870fa2200","size":5,"soft_fail":false,"target_va":5383325024,"va":5381032683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::intersection_aabb_segment2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2268374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381032272,"raw_hex":"e8d69c2200","size":5,"soft_fail":false,"target_va":5383300992,"va":5381032613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":30416,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":580,"record_offset":30028,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":28130,"source_line":68,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":580,"record_offset":30112,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":30620,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31764,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29824,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":30504,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":30000,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31540,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29668,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29772,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29720,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31108,"parent_record_offset":29460,"ancestor_record_offset":29460,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":580,"record_offset":30644,"parent_record_offset":30620,"ancestor_record_offset":29460,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":30204,"parent_record_offset":30112,"ancestor_record_offset":29460,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":30836,"parent_record_offset":30780,"ancestor_record_offset":29460,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":30144,"parent_record_offset":30112,"ancestor_record_offset":29460,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":30172,"parent_record_offset":30112,"ancestor_record_offset":29460,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":31012,"parent_record_offset":30928,"ancestor_record_offset":29460,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":31308,"parent_record_offset":31108,"ancestor_record_offset":29460,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":31276,"parent_record_offset":31136,"ancestor_record_offset":29460,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":30556,"parent_record_offset":30528,"ancestor_record_offset":29460,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":29904,"parent_record_offset":29880,"ancestor_record_offset":29460,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":29692,"parent_record_offset":29668,"ancestor_record_offset":29460,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":30584,"parent_record_offset":30528,"ancestor_record_offset":29460,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":29744,"parent_record_offset":29720,"ancestor_record_offset":29460,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":29460,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":30728,"parent_record_offset":30620,"ancestor_record_offset":29460,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":30928,"parent_record_offset":30728,"ancestor_record_offset":29460,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":30692,"parent_record_offset":30668,"ancestor_record_offset":29460,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":30668,"parent_record_offset":30644,"ancestor_record_offset":29460,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":29848,"parent_record_offset":29824,"ancestor_record_offset":29460,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":30528,"parent_record_offset":30504,"ancestor_record_offset":29460,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31364,"parent_record_offset":31340,"ancestor_record_offset":29460,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":29460,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31472,"parent_record_offset":31364,"ancestor_record_offset":29460,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":31164,"parent_record_offset":31136,"ancestor_record_offset":29460,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":29460,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":30756,"parent_record_offset":30728,"ancestor_record_offset":29460,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":29880,"parent_record_offset":29848,"ancestor_record_offset":29460,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31220,"parent_record_offset":31136,"ancestor_record_offset":29460,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31340,"parent_record_offset":31108,"ancestor_record_offset":29460,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":31136,"parent_record_offset":31108,"ancestor_record_offset":29460,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00580.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 39; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump
// 信息 ID: 07e660a8c699e927943c92c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=580, inline_record=19144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$0
// 信息 ID: 057ce7a135856523136d245c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0 <- alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; module=1643, inline_record=209364; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,alloc::alloc::Global,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$0>; module=1643, inline_record=209536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1
// 信息 ID: 41ab65557b161db4c7ed5e03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1643, inline_record=724536; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1643, inline_record=725416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$1::closure$0
// 信息 ID: fbee4544fd30d047e41a07ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1643, inline_record=724612; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> >; module=1643, inline_record=725476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure$2
// 信息 ID: 5a8a2ac709141889e3156bd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ivy::ivy_grower::check_for_wall_jump", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::driftsort_main<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2>,alloc::vec::Vec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::merge::merge<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::drift::sort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2 <- core::slice::sort::stable::quicksort::quicksort<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=528, inline_record=39668; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=528, inline_record=39752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=528, inline_record=39836; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=528, inline_record=39920; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1639, inline_record=32680; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1639, inline_record=32780; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1639, inline_record=32976; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$2> >; module=1639, inline_record=33096; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::closest_segment_index
// 信息 ID: 960913a81238351208b90e15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::is_point_still_valid @ 0xa5e184 -> 0xbc0510 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0xbbeda2 -> 0xbc0510 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_spawner::ivy_spawner @ 0xbc0eea -> 0xbc0510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_growth_point_proposal
// 信息 ID: f7f1d93316035bdddbcde977
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=580, inline_record=16588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction
// 信息 ID: 47e1fa8697bdf088b2148e29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=580, inline_record=18072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal
// 信息 ID: 27dcff6c126e3dbe9e95bb32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ivy::ivy_grower::ivy_grower @ 0xbbfb5e -> 0xbc0790 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=580, inline_record=19768; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment
// 信息 ID: 4aca05c35deaf6aee0cdcecc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0x16140f0 -> 0xbc0950 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,glam::f32::vec2::Vec2,country_core::resources::walls::WallId>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::ivy::ivy_grower::check_for_wall_jump::closure_env$1> > @ 0x16141e4 -> 0xbc0950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_grower::ivy_grower
// 信息 ID: d1ca5ef05f80f1a87bd5af29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6234e88d4718e720ced63e30: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)> > @ 0x1922ce2 -> 0xbbe6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)> @ 0x19be2d2 -> 0xbbe6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
