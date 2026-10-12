// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ee35198776488ede999620e9e0d8acfdf917a5bcab9e800aad8e946eb0a27a70
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_turret.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1529
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1529,"record_offset":280,"rva":21508240,"va":5390217360}
// 编译实例 RVAs: 0x1483090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14540966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e85a1f22ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390219649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-14542242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e85e1a22ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390220941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21497443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e89df9b7fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390219630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20984692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e88cccbffe","size":5,"soft_fail":false,"target_va":5369240528,"va":5390225215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20927362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e87eacc0fe","size":5,"soft_fail":false,"target_va":5369296688,"va":5390224045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20676810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8367fc4fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5390218805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12097567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8e16747ff","size":5,"soft_fail":false,"target_va":5378120480,"va":5390218042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12097652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e88c6747ff","size":5,"soft_fail":false,"target_va":5378120496,"va":5390218143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12094793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8b77247ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390221732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11719142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e81a2e4dff","size":5,"soft_fail":false,"target_va":5378502608,"va":5390221745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a620a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10622208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e800eb5dff","size":5,"soft_fail":false,"target_va":5379596448,"va":5390218651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deb2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6915252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e84c7b96ff","size":5,"soft_fail":false,"target_va":5383303904,"va":5390219151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6895710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8a2c796ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390220729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6894180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e89ccd96ff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390219231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-601596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e804d2f6ff","size":5,"soft_fail":false,"target_va":5389619632,"va":5390221223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-525988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e85cf9f7ff","size":5,"soft_fail":false,"target_va":5389695968,"va":5390221951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-514029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e81328f8ff","size":5,"soft_fail":false,"target_va":5389704672,"va":5390218696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26129380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8e4b38e01","size":5,"soft_fail":false,"target_va":5416350496,"va":5390221111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fb00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26131637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8b5bc8e01","size":5,"soft_fail":false,"target_va":5416352512,"va":5390220870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26838160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e890849901","size":5,"soft_fail":false,"target_va":5417057664,"va":5390219499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26838904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e878879901","size":5,"soft_fail":false,"target_va":5417057936,"va":5390219027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26160916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8142f8f01","size":5,"soft_fail":false,"target_va":5416380336,"va":5390219415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26399405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8add29201","size":5,"soft_fail":false,"target_va":5416620794,"va":5390221384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26398994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e812d19201","size":5,"soft_fail":false,"target_va":5416621408,"va":5390222409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26399324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e85cd29201","size":5,"soft_fail":false,"target_va":5416621600,"va":5390222271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26399148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8acd19201","size":5,"soft_fail":false,"target_va":5416621762,"va":5390222609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26399904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8a0d49201","size":5,"soft_fail":false,"target_va":5416622240,"va":5390222331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e87e1e0000","size":5,"soft_fail":false,"target_va":5390225344,"va":5390217533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26398574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e86ecf9201","size":5,"soft_fail":false,"target_va":5416621376,"va":5390222797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11740259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e89ddb4cff","size":5,"soft_fail":false,"target_va":5378481680,"va":5390221934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10311293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e883a962ff","size":5,"soft_fail":false,"target_va":5379908016,"va":5390219304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10310381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e813ad62ff","size":5,"soft_fail":false,"target_va":5379908992,"va":5390219368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8986705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8afdf76ff","size":5,"soft_fail":false,"target_va":5381231904,"va":5390218604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9315041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e81fdd71ff","size":5,"soft_fail":false,"target_va":5380903536,"va":5390218572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26354337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8a1229201","size":5,"soft_fail":false,"target_va":5416576832,"va":5390222490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t26353586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8b21f9201","size":5,"soft_fail":false,"target_va":5416576144,"va":5390222553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6236491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8b5d6a0ff","size":5,"soft_fail":false,"target_va":5383984848,"va":5390221334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6236681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8f7d5a0ff","size":5,"soft_fail":false,"target_va":5383985360,"va":5390222036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6254094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8f291a0ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390217881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6245587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e82db3a0ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390217806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6257211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8c585a0ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390222118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6254684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8a48fa0ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390217495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6813462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8ea0898ff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390221713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26474584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e858f89301","size":5,"soft_fail":false,"target_va":5416697008,"va":5390222419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-452132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8dc19f9ff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390223263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26396826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e89ac89201","size":5,"soft_fail":false,"target_va":5416619360,"va":5390222529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6918083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e83d7096ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390220318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6918018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e87e7096ff","size":5,"soft_fail":false,"target_va":5383302288,"va":5390220301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6918065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e84f7096ff","size":5,"soft_fail":false,"target_va":5383302224,"va":5390220284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6912962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e83e8496ff","size":5,"soft_fail":false,"target_va":5383306512,"va":5390219469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6912363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e8958696ff","size":5,"soft_fail":false,"target_va":5383307120,"va":5390219478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6943348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390217360,"raw_hex":"e88c0d96ff","size":5,"soft_fail":false,"target_va":5383276416,"va":5390219759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5390217360,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5390221263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":10112,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":2804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":4412,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":5268,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":2956,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1529,"record_offset":11860,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1529,"record_offset":9364,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1529,"record_offset":7768,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1529,"record_offset":3860,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1529,"record_offset":4304,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1529,"record_offset":5184,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":4240,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1529,"record_offset":8460,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1529,"record_offset":5740,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":3068,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1529,"record_offset":10140,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1529,"record_offset":10236,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1529,"record_offset":2860,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1529,"record_offset":8292,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3680,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3892,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7908,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":8180,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7656,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7852,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7712,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1529,"record_offset":6052,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1529,"record_offset":4664,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":1996,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":592,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":984,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1529,"record_offset":5212,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1529,"record_offset":4636,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
//     evidence={"module":1529,"record_offset":12044,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":80066,"source_line":216,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1529,"record_offset":3148,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1529,"record_offset":6528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":5608,"parent_record_offset":5408,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1529,"record_offset":8624,"parent_record_offset":8532,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":4512,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":5340,"parent_record_offset":5304,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":8560,"parent_record_offset":8532,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":5372,"parent_record_offset":5340,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1529,"record_offset":5448,"parent_record_offset":5408,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":4480,"parent_record_offset":4440,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":9508,"parent_record_offset":9464,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":5304,"parent_record_offset":5268,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1529,"record_offset":3452,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1529,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1529,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1529,"record_offset":9436,"parent_record_offset":9396,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1529,"record_offset":9396,"parent_record_offset":9364,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1529,"record_offset":9464,"parent_record_offset":9396,"ancestor_record_offset":280,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1529,"record_offset":5476,"parent_record_offset":5448,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2324,"parent_record_offset":2296,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2272,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2412,"parent_record_offset":2296,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1529,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1529,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1529,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1529,"record_offset":4368,"parent_record_offset":4336,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1529,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":280,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1529,"record_offset":8372,"parent_record_offset":8292,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1529,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":280,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1529,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":280,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1529,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1529,"record_offset":5568,"parent_record_offset":5504,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1529,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1529,"record_offset":6292,"parent_record_offset":6264,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1529,"record_offset":6088,"parent_record_offset":6052,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1529,"record_offset":4336,"parent_record_offset":4304,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1529,"record_offset":6320,"parent_record_offset":6292,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1529,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1529,"record_offset":5504,"parent_record_offset":5476,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1529,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1529,"record_offset":5536,"parent_record_offset":5504,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1529,"record_offset":8592,"parent_record_offset":8560,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1529,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1529,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1529,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":280,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1529,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1529,"record_offset":2144,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1529,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1529,"record_offset":6560,"parent_record_offset":6528,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":10504,"parent_record_offset":10236,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":6264,"parent_record_offset":6236,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":10316,"parent_record_offset":10236,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":10536,"parent_record_offset":10236,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1529,"record_offset":6496,"parent_record_offset":6052,"ancestor_record_offset":280,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1529,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1529,"record_offset":2760,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1529,"record_offset":2476,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1529,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1529,"record_offset":6172,"parent_record_offset":6116,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1529,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1529,"record_offset":11928,"parent_record_offset":11860,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1529,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1529,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1529,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1529,"record_offset":4600,"parent_record_offset":4440,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1529,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1529,"record_offset":1660,"parent_record_offset":1632,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1529,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1529,"record_offset":10476,"parent_record_offset":10236,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1529,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1529,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1529,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":6116,"parent_record_offset":6088,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1529,"record_offset":1308,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1529,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1529,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1529,"record_offset":1160,"parent_record_offset":1128,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1529,"record_offset":1128,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1529,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1529,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1529,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1529,"record_offset":1632,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":696,"parent_record_offset":664,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":880,"parent_record_offset":824,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":792,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":636,"parent_record_offset":592,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1529,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1529,"record_offset":1192,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1529,"record_offset":1476,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1529,"record_offset":1448,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1529,"record_offset":1252,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1529,"record_offset":12564,"parent_record_offset":12500,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1529,"record_offset":10388,"parent_record_offset":10352,"ancestor_record_offset":280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1529,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1529,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1529,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":280,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1529,"record_offset":10572,"parent_record_offset":10536,"ancestor_record_offset":280,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1529,"record_offset":12472,"parent_record_offset":12444,"ancestor_record_offset":280,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1529,"record_offset":2920,"parent_record_offset":2892,"ancestor_record_offset":280,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1529,"record_offset":2892,"parent_record_offset":2860,"ancestor_record_offset":280,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1529,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":280,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1529,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":280,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3760,"parent_record_offset":3704,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7960,"parent_record_offset":7932,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3360,"parent_record_offset":3248,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1529,"record_offset":3704,"parent_record_offset":3680,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1529,"record_offset":6208,"parent_record_offset":6052,"ancestor_record_offset":280,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1529,"record_offset":10352,"parent_record_offset":10236,"ancestor_record_offset":280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":1529,"record_offset":12500,"parent_record_offset":12472,"ancestor_record_offset":280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1529,"record_offset":10444,"parent_record_offset":10352,"ancestor_record_offset":280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":1529,"record_offset":12536,"parent_record_offset":12500,"ancestor_record_offset":280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":2032,"parent_record_offset":1996,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":11044,"parent_record_offset":11012,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":11496,"parent_record_offset":11292,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1529,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":2244,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1529,"record_offset":2448,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1529,"record_offset":3792,"parent_record_offset":3760,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1529,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1529,"record_offset":2104,"parent_record_offset":2068,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1529,"record_offset":2068,"parent_record_offset":2032,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1529,"record_offset":3184,"parent_record_offset":3148,"ancestor_record_offset":280,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1529,"record_offset":3424,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1529,"record_offset":3548,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1529,"record_offset":3220,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x138C2
// 首条位置: {"module":1529,"inlinee":80066}
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
// 完整结构及原始类型记录: circle_turret.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_turret.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_turret.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 信息 ID: 0673d30e9916743298a594c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x835dd7 -> 0x1483090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 信息 ID: a68076dfba8a4caa665b437b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret; module=1529, inline_record=12044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
