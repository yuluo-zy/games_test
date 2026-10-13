// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7fd23031bea3bc4ee4a8b3503dd142adf26c39d1ecf688fe0555978a6e9a40c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle_large_brackets.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1484
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1484,"record_offset":1660,"rva":19218368,"va":5387927488}
// 编译实例 RVAs: 0x1253fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18664774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ba32e3fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387932433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18327664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e89057e8fe","size":5,"soft_fail":false,"target_va":5369600800,"va":5387928459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16294217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8b75e07ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5387928628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13122356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ccc437ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387935775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12847988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e88cf43bff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387929471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12848464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8b0f23bff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387929963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10190254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8528264ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387932425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7873639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e899db87ff","size":5,"soft_fail":false,"target_va":5380057424,"va":5387931058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5996925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8837ea4ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387931480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-378304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8403afaff","size":5,"soft_fail":false,"target_va":5387550576,"va":5387928875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f97b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-373545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8d74cfaff","size":5,"soft_fail":false,"target_va":5387556784,"va":5387930324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fa6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-369310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8625dfaff","size":5,"soft_fail":false,"target_va":5387560624,"va":5387929929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fcf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-358406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8fa87faff","size":5,"soft_fail":false,"target_va":5387570944,"va":5387929345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-304709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8bb59fbff","size":5,"soft_fail":false,"target_va":5387623488,"va":5387928192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-298857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e89770fbff","size":5,"soft_fail":false,"target_va":5387633712,"va":5387932564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-57650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ce1effff","size":5,"soft_fail":false,"target_va":5387874224,"va":5387931869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23918879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e81ff96c01","size":5,"soft_fail":false,"target_va":5411847152,"va":5387928268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23917499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8bbf36c01","size":5,"soft_fail":false,"target_va":5411847168,"va":5387929664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23917178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e87af26c01","size":5,"soft_fail":false,"target_va":5411847952,"va":5387930769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13570277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e81bef30ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5387929312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23607465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8a9386801","size":5,"soft_fail":false,"target_va":5411540963,"va":5387933493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23608175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e86f3b6801","size":5,"soft_fail":false,"target_va":5411541584,"va":5387933404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23608452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8843c6801","size":5,"soft_fail":false,"target_va":5411541776,"va":5387933319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23549063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e887546701","size":5,"soft_fail":false,"target_va":5411482433,"va":5387933365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23608890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e83a3e6801","size":5,"soft_fail":false,"target_va":5411542064,"va":5387933169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23608759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8b73d6801","size":5,"soft_fail":false,"target_va":5411541904,"va":5387933140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e825fbffff","size":5,"soft_fail":false,"target_va":5387926416,"va":5387927654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23615624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e888586801","size":5,"soft_fail":false,"target_va":5411543360,"va":5387927731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7511824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8f0608dff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387928795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8136156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e824da83ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5387928775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple_exclude_wall_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9995321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8c77b67ff","size":5,"soft_fail":false,"target_va":5377936464,"va":5387931780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-349921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e81fa9faff","size":5,"soft_fail":false,"target_va":5387580928,"va":5387930844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23558979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8437b6701","size":5,"soft_fail":false,"target_va":5411492368,"va":5387933384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23561381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8a5846701","size":5,"soft_fail":false,"target_va":5411494800,"va":5387933414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6104751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e851d9a2ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387928346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6108825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e867c9a2ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387932836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6542619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8e52a9cff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387928150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6537430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e82a3f9cff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387928065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6546449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ef1b9cff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387932924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6542929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8af299cff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387927628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23707519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e87fbf6901","size":5,"soft_fail":false,"target_va":5411640768,"va":5387933244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23603792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8502a6801","size":5,"soft_fail":false,"target_va":5411537104,"va":5387933307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-85713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e82fb1feff","size":5,"soft_fail":false,"target_va":5387845616,"va":5387931324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-85268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ecb2feff","size":5,"soft_fail":false,"target_va":5387846080,"va":5387931343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-82920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e818bcfeff","size":5,"soft_fail":false,"target_va":5387848464,"va":5387931379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6004812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8b45fa4ff","size":5,"soft_fail":false,"target_va":5381924400,"va":5387929207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6006125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8935aa4ff","size":5,"soft_fail":false,"target_va":5381923136,"va":5387929256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6008918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8aa4fa4ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387929169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6008774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e83a50a4ff","size":5,"soft_fail":false,"target_va":5381920464,"va":5387929233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6011064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e84847a4ff","size":5,"soft_fail":false,"target_va":5381920064,"va":5387931123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6047588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e89cb8a3ff","size":5,"soft_fail":false,"target_va":5381884480,"va":5387932063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6045205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387927488,"raw_hex":"e8ebc1a3ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387931104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387927488,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5387928288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":6508,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":7476,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":14828,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1484,"record_offset":17392,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":15700,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1484,"record_offset":12324,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1484,"record_offset":7672,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":12240,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":4648,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":10884,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":6204,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1484,"record_offset":6092,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1484,"record_offset":4408,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1484,"record_offset":4684,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1484,"record_offset":15004,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":45013,"source_line":300,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":1484,"record_offset":15224,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":16112,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":15588,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":15644,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":14948,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":15876,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":17216,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":15728,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":17304,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":3424,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":2000,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":2448,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":6536,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":6356,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":13732,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":5116,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":14120,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
//     evidence={"module":1484,"record_offset":17420,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":71299,"source_line":73,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1484,"record_offset":16632,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1484,"record_offset":12072,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4633,"source_line":132,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1484,"record_offset":7812,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":1660,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":1660,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":9292,"parent_record_offset":9268,"ancestor_record_offset":1660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":8324,"parent_record_offset":8148,"ancestor_record_offset":1660,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":9268,"parent_record_offset":9180,"ancestor_record_offset":1660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":1660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":8088,"parent_record_offset":8060,"ancestor_record_offset":1660,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":9208,"parent_record_offset":9180,"ancestor_record_offset":1660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":1660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":7600,"parent_record_offset":7576,"ancestor_record_offset":1660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":1660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":1660,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":1660,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":1660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1484,"record_offset":8184,"parent_record_offset":8148,"ancestor_record_offset":1660,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1484,"record_offset":8020,"parent_record_offset":7984,"ancestor_record_offset":1660,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1484,"record_offset":7984,"parent_record_offset":7912,"ancestor_record_offset":1660,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":7520,"parent_record_offset":7476,"ancestor_record_offset":1660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":7956,"parent_record_offset":7912,"ancestor_record_offset":1660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":8060,"parent_record_offset":8020,"ancestor_record_offset":1660,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":7876,"parent_record_offset":7812,"ancestor_record_offset":1660,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":1660,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":7912,"parent_record_offset":7876,"ancestor_record_offset":1660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1484,"record_offset":17544,"parent_record_offset":17520,"ancestor_record_offset":1660,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1484,"record_offset":17592,"parent_record_offset":17568,"ancestor_record_offset":1660,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1484,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":1660,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1484,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":1660,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1484,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":1660,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3760,"parent_record_offset":3732,"ancestor_record_offset":1660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":5400,"parent_record_offset":5372,"ancestor_record_offset":1660,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":1660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":1660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3708,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":1660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":3904,"parent_record_offset":3732,"ancestor_record_offset":1660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1484,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":1660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":1660,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":14852,"parent_record_offset":14828,"ancestor_record_offset":1660,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":1660,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":1660,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":1660,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":4476,"parent_record_offset":4436,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":1660,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1484,"record_offset":13212,"parent_record_offset":12596,"ancestor_record_offset":1660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1484,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":1660,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1484,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":1660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1484,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":1660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":15164,"parent_record_offset":15004,"ancestor_record_offset":1660,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":15104,"parent_record_offset":15004,"ancestor_record_offset":1660,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":16896,"parent_record_offset":16872,"ancestor_record_offset":1660,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":1660,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":15040,"parent_record_offset":15004,"ancestor_record_offset":1660,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":12860,"parent_record_offset":12832,"ancestor_record_offset":1660,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1484,"record_offset":13456,"parent_record_offset":13276,"ancestor_record_offset":1660,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1484,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":1660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1484,"record_offset":7296,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1484,"record_offset":12392,"parent_record_offset":12360,"ancestor_record_offset":1660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1484,"record_offset":12360,"parent_record_offset":12324,"ancestor_record_offset":1660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1484,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":1660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1484,"record_offset":12488,"parent_record_offset":12456,"ancestor_record_offset":1660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1484,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":1660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1484,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":1660,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1484,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":1660,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":1660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1484,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":1660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":18608,"parent_record_offset":18580,"ancestor_record_offset":1660,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":1660,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":1660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":1660,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":1660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1484,"record_offset":2148,"parent_record_offset":2116,"ancestor_record_offset":1660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1484,"record_offset":7084,"parent_record_offset":7056,"ancestor_record_offset":1660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1484,"record_offset":7056,"parent_record_offset":7028,"ancestor_record_offset":1660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12800,"parent_record_offset":12692,"ancestor_record_offset":1660,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12744,"parent_record_offset":12692,"ancestor_record_offset":1660,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":1660,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":1660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1484,"record_offset":3576,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1484,"record_offset":12560,"parent_record_offset":12524,"ancestor_record_offset":1660,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1484,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":1660,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1484,"record_offset":13316,"parent_record_offset":13276,"ancestor_record_offset":1660,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":12428,"parent_record_offset":12392,"ancestor_record_offset":1660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":4948,"parent_record_offset":4684,"ancestor_record_offset":1660,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":12524,"parent_record_offset":12488,"ancestor_record_offset":1660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":13520,"parent_record_offset":12360,"ancestor_record_offset":1660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":4760,"parent_record_offset":4684,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1484,"record_offset":4984,"parent_record_offset":4684,"ancestor_record_offset":1660,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1484,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":1660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1484,"record_offset":4364,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1484,"record_offset":8424,"parent_record_offset":8020,"ancestor_record_offset":1660,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":3968,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1484,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":1660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1484,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":1660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":1660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":1660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":13628,"parent_record_offset":13568,"ancestor_record_offset":1660,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":1660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":1660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":13652,"parent_record_offset":13628,"ancestor_record_offset":1660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1484,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":1660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1484,"record_offset":4588,"parent_record_offset":4560,"ancestor_record_offset":1660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1484,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1484,"record_offset":4920,"parent_record_offset":4684,"ancestor_record_offset":1660,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1484,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":1660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":1660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":1660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":2764,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":6268,"parent_record_offset":6204,"ancestor_record_offset":1660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1484,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1484,"record_offset":2476,"parent_record_offset":2448,"ancestor_record_offset":1660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1484,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1484,"record_offset":2584,"parent_record_offset":2548,"ancestor_record_offset":1660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1484,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1484,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1484,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1484,"record_offset":3088,"parent_record_offset":2548,"ancestor_record_offset":1660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2248,"parent_record_offset":2216,"ancestor_record_offset":1660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2344,"parent_record_offset":2248,"ancestor_record_offset":1660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":4504,"parent_record_offset":4476,"ancestor_record_offset":1660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2032,"parent_record_offset":2000,"ancestor_record_offset":1660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1484,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":1660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1484,"record_offset":2648,"parent_record_offset":2616,"ancestor_record_offset":1660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1484,"record_offset":2932,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1484,"record_offset":2904,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1484,"record_offset":2708,"parent_record_offset":2648,"ancestor_record_offset":1660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1484,"record_offset":19300,"parent_record_offset":19236,"ancestor_record_offset":1660,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1484,"record_offset":4832,"parent_record_offset":4796,"ancestor_record_offset":1660,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1484,"record_offset":19324,"parent_record_offset":19300,"ancestor_record_offset":1660,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1484,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":1660,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1484,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":1660,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1484,"record_offset":5012,"parent_record_offset":4984,"ancestor_record_offset":1660,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1484,"record_offset":19208,"parent_record_offset":19180,"ancestor_record_offset":1660,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1484,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":1660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":1484,"record_offset":16660,"parent_record_offset":16632,"ancestor_record_offset":1660,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":16704,"parent_record_offset":16660,"ancestor_record_offset":1660,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":16776,"parent_record_offset":16704,"ancestor_record_offset":1660,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":1660,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":16872,"parent_record_offset":16660,"ancestor_record_offset":1660,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":16204,"parent_record_offset":16136,"ancestor_record_offset":1660,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1484,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":1660,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":15976,"parent_record_offset":15940,"ancestor_record_offset":1660,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":15940,"parent_record_offset":15904,"ancestor_record_offset":1660,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":15904,"parent_record_offset":15876,"ancestor_record_offset":1660,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1484,"record_offset":6996,"parent_record_offset":6968,"ancestor_record_offset":1660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1484,"record_offset":7028,"parent_record_offset":6996,"ancestor_record_offset":1660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1484,"record_offset":14476,"parent_record_offset":14304,"ancestor_record_offset":1660,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1484,"record_offset":6968,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":6828,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":1660,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":7324,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":13996,"parent_record_offset":13940,"ancestor_record_offset":1660,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":14564,"parent_record_offset":14252,"ancestor_record_offset":1660,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1484,"record_offset":6912,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1484,"record_offset":6800,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1484,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":1660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":1660,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":1660,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":1660,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":13780,"parent_record_offset":13756,"ancestor_record_offset":1660,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":5560,"parent_record_offset":5200,"ancestor_record_offset":1660,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":1660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":1660,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":1660,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":1660,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1484,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":1660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":14664,"parent_record_offset":14252,"ancestor_record_offset":1660,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":7416,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":13912,"parent_record_offset":13852,"ancestor_record_offset":1660,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":14252,"parent_record_offset":14216,"ancestor_record_offset":1660,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":1660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":6668,"parent_record_offset":6608,"ancestor_record_offset":1660,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":13828,"parent_record_offset":13804,"ancestor_record_offset":1660,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18548,"parent_record_offset":18516,"ancestor_record_offset":1660,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":13884,"parent_record_offset":13852,"ancestor_record_offset":1660,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18460,"parent_record_offset":18436,"ancestor_record_offset":1660,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18516,"parent_record_offset":18460,"ancestor_record_offset":1660,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18488,"parent_record_offset":18460,"ancestor_record_offset":1660,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":1660,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":6772,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18580,"parent_record_offset":18548,"ancestor_record_offset":1660,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":18436,"parent_record_offset":18412,"ancestor_record_offset":1660,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":1660,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1484,"record_offset":7128,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1484,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":1660,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1484,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":1660,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1484,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":1660,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1484,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":1660,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1484,"record_offset":14144,"parent_record_offset":14120,"ancestor_record_offset":1660,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1484,"record_offset":12692,"parent_record_offset":12640,"ancestor_record_offset":1660,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1484,"record_offset":4796,"parent_record_offset":4684,"ancestor_record_offset":1660,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1484,"record_offset":19236,"parent_record_offset":19208,"ancestor_record_offset":1660,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1484,"record_offset":4888,"parent_record_offset":4796,"ancestor_record_offset":1660,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1484,"record_offset":19272,"parent_record_offset":19236,"ancestor_record_offset":1660,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":3460,"parent_record_offset":3424,"ancestor_record_offset":1660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":19432,"parent_record_offset":19400,"ancestor_record_offset":1660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":19948,"parent_record_offset":19688,"ancestor_record_offset":1660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1484,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":1660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":3680,"parent_record_offset":3532,"ancestor_record_offset":1660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1484,"record_offset":3940,"parent_record_offset":3680,"ancestor_record_offset":1660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1484,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":1660,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1484,"record_offset":16012,"parent_record_offset":15976,"ancestor_record_offset":1660,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1484,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":1660,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1484,"record_offset":5372,"parent_record_offset":5316,"ancestor_record_offset":1660,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1484,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":1660,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1484,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1484,"record_offset":3532,"parent_record_offset":3496,"ancestor_record_offset":1660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1484,"record_offset":3496,"parent_record_offset":3460,"ancestor_record_offset":1660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
//     evidence={"module":1484,"record_offset":12596,"parent_record_offset":12560,"ancestor_record_offset":1660,"inlinee":71298,"source_line":138,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1484,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":1660,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1484,"record_offset":13276,"parent_record_offset":13244,"ancestor_record_offset":1660,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1484,"record_offset":13184,"parent_record_offset":12596,"ancestor_record_offset":1660,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":12832,"parent_record_offset":12640,"ancestor_record_offset":1660,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":1660,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":12640,"parent_record_offset":12596,"ancestor_record_offset":1660,"inlinee":24689,"source_line":55,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":13244,"parent_record_offset":13212,"ancestor_record_offset":1660,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11683
// 首条位置: {"module":1484,"inlinee":71299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11682
// 首条位置: {"module":1484,"inlinee":71298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1484,"record_offset":13212,"parent_record_offset":12596,"ancestor_record_offset":12596,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1484,"record_offset":13184,"parent_record_offset":12596,"ancestor_record_offset":12596,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":12640,"parent_record_offset":12596,"ancestor_record_offset":12596,"inlinee":24689,"source_line":55,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":12596,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":12596,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":12860,"parent_record_offset":12832,"ancestor_record_offset":12596,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1484,"record_offset":13456,"parent_record_offset":13276,"ancestor_record_offset":12596,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12800,"parent_record_offset":12692,"ancestor_record_offset":12596,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12744,"parent_record_offset":12692,"ancestor_record_offset":12596,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":12596,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1484,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":12596,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1484,"record_offset":13316,"parent_record_offset":13276,"ancestor_record_offset":12596,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1484,"record_offset":12692,"parent_record_offset":12640,"ancestor_record_offset":12596,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1484,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":12596,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1484,"record_offset":13276,"parent_record_offset":13244,"ancestor_record_offset":12596,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":12832,"parent_record_offset":12640,"ancestor_record_offset":12596,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":12596,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1484,"record_offset":13244,"parent_record_offset":13212,"ancestor_record_offset":12596,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01484.jsonl.gz"}
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
// 完整结构及原始类型记录: rectangle_large_brackets.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle_large_brackets.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle_large_brackets.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets
// 信息 ID: bf2213c88105831650eac2d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets @ 0x789ccf -> 0x1253fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
// 信息 ID: ef69ba66bed2444ee63e9ac4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1484, inline_record=17420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
// 信息 ID: f78ad3905948a9d5f2316a10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1484, inline_record=12596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
