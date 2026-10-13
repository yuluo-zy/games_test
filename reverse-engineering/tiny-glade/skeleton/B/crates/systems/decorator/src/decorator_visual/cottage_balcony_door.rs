// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d1fcfb746d6e6cacdc39c3561342d4931ea54422d1bb269d1a10928efd7c2be2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 444
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":444,"record_offset":1924,"rva":8338848,"va":5377047968}
// 编译实例 RVAs: 0x7f3da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140257ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5887940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e83c28a6ff","size":5,"soft_fail":false,"target_va":5371165600,"va":5377053535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2373201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8afc9dbff","size":5,"soft_fail":false,"target_va":5374676336,"va":5377049532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2372506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e866ccdbff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377049189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t257473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8c1ed0300","size":5,"soft_fail":false,"target_va":5377307216,"va":5377049738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t819661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8cd810c00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377049422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1283650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e842961300","size":5,"soft_fail":false,"target_va":5378336448,"va":5377052793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2257033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e889702200","size":5,"soft_fail":false,"target_va":5379305728,"va":5377048690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3348465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8f1173300","size":5,"soft_fail":false,"target_va":5380398224,"va":5377049754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3353110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8162a3300","size":5,"soft_fail":false,"target_va":5380401776,"va":5377048661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3565685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e875683600","size":5,"soft_fail":false,"target_va":5380621856,"va":5377056166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4839832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e898d94900","size":5,"soft_fail":false,"target_va":5381889712,"va":5377049875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4884363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e88b874a00","size":5,"soft_fail":false,"target_va":5381934528,"va":5377050160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4884478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8fe874a00","size":5,"soft_fail":false,"target_va":5381934560,"va":5377050077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10499834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8fa36a000","size":5,"soft_fail":false,"target_va":5387550144,"va":5377050305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10914518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8d68aa600","size":5,"soft_fail":false,"target_va":5387964304,"va":5377049781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412898e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11093565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e83d46a900","size":5,"soft_fail":false,"target_va":5388146912,"va":5377053342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cc30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11107594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e80a7da900","size":5,"soft_fail":false,"target_va":5388160048,"va":5377052449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-2692469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e88bead6ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5377051504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34486586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e83a390e02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377054993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34487252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8d43b0e02","size":5,"soft_fail":false,"target_va":5411541808,"va":5377054551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34487249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8d13b0e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377054522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34486970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8ba3a0e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377055089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34487049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8093b0e02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377055242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34487235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8c33b0e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377054664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8dcfaffff","size":5,"soft_fail":false,"target_va":5377046816,"va":5377048127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34495151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8af5a0e02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377048204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t258207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e89ff00300","size":5,"soft_fail":false,"target_va":5377307920,"va":5377049708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t255784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e828e70300","size":5,"soft_fail":false,"target_va":5377307232,"va":5377051443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2256609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8e16e2200","size":5,"soft_fail":false,"target_va":5379305584,"va":5377048970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3352132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e844263300","size":5,"soft_fail":false,"target_va":5380401520,"va":5377049383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3367510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e856623300","size":5,"soft_fail":false,"target_va":5380417344,"va":5377049829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4336899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8032d4200","size":5,"soft_fail":false,"target_va":5381385536,"va":5377048632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4342088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e848414200","size":5,"soft_fail":false,"target_va":5381390640,"va":5377048547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4332073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8291a4200","size":5,"soft_fail":false,"target_va":5381386480,"va":5377054402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4336602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8da2b4200","size":5,"soft_fail":false,"target_va":5381384704,"va":5377048097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34589881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8b9cc0f02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377054914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34482118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8c6270e02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377054981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t198980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e844090300","size":5,"soft_fail":false,"target_va":5377249968,"va":5377050983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t202037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e835150300","size":5,"soft_fail":false,"target_va":5377251504,"va":5377049462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e81ae1ffff","size":5,"soft_fail":false,"target_va":5377045312,"va":5377053217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4854707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e8b3134a00","size":5,"soft_fail":false,"target_va":5381907008,"va":5377052296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4840221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e81ddb4900","size":5,"soft_fail":false,"target_va":5381890192,"va":5377049966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4836232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377047968,"raw_hex":"e888cb4900","size":5,"soft_fail":false,"target_va":5381886416,"va":5377050179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":13340,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":444,"record_offset":7416,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":444,"record_offset":7472,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":444,"record_offset":4692,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":444,"record_offset":4664,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":16148,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":444,"record_offset":15780,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":12092,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":8576,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":9204,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":10584,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":4608,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":8796,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":9020,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":9176,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":8964,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":3624,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":2200,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":2648,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":444,"record_offset":7500,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":444,"record_offset":11964,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs
//     evidence={"module":444,"record_offset":13784,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25418,"source_line":193,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":444,"record_offset":13564,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":444,"record_offset":13536,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":444,"record_offset":14792,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":444,"record_offset":15096,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":444,"record_offset":8740,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25386,"source_line":221,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":13416,"parent_record_offset":13384,"ancestor_record_offset":1924,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":1924,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":13440,"parent_record_offset":13416,"ancestor_record_offset":1924,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":444,"record_offset":10404,"parent_record_offset":9240,"ancestor_record_offset":1924,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":444,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":1924,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":13384,"parent_record_offset":13340,"ancestor_record_offset":1924,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":14228,"parent_record_offset":13784,"ancestor_record_offset":1924,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":15132,"parent_record_offset":15096,"ancestor_record_offset":1924,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":444,"record_offset":7440,"parent_record_offset":7416,"ancestor_record_offset":1924,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":444,"record_offset":15812,"parent_record_offset":15780,"ancestor_record_offset":1924,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":444,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":1924,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":444,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":1924,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":1924,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":1924,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":1924,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":1924,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":1924,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":4104,"parent_record_offset":3932,"ancestor_record_offset":1924,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":444,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":1924,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":13852,"parent_record_offset":13784,"ancestor_record_offset":1924,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":1924,"inlinee":25421,"source_line":1820,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":1924,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":444,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":444,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":1924,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":444,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":1924,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":444,"record_offset":5196,"parent_record_offset":5160,"ancestor_record_offset":1924,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":1924,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":444,"record_offset":15008,"parent_record_offset":14792,"ancestor_record_offset":1924,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":444,"record_offset":12060,"parent_record_offset":11964,"ancestor_record_offset":1924,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":444,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":1924,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":444,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":1924,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":444,"record_offset":7196,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":444,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":1924,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":444,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":1924,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":444,"record_offset":14040,"parent_record_offset":14012,"ancestor_record_offset":1924,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":444,"record_offset":5004,"parent_record_offset":4976,"ancestor_record_offset":1924,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":444,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":1924,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":444,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":1924,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":444,"record_offset":8844,"parent_record_offset":8820,"ancestor_record_offset":1924,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":444,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":1924,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":444,"record_offset":3776,"parent_record_offset":3732,"ancestor_record_offset":1924,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":444,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":1924,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":444,"record_offset":12000,"parent_record_offset":11964,"ancestor_record_offset":1924,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":444,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":1924,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":444,"record_offset":5388,"parent_record_offset":4692,"ancestor_record_offset":1924,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":444,"record_offset":7372,"parent_record_offset":6304,"ancestor_record_offset":1924,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":444,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":1924,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":444,"record_offset":2596,"parent_record_offset":2568,"ancestor_record_offset":1924,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":444,"record_offset":4564,"parent_record_offset":3732,"ancestor_record_offset":1924,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":4168,"parent_record_offset":3880,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":444,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":1924,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":444,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1924,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":444,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":1924,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":444,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":1924,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":1924,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":444,"record_offset":15164,"parent_record_offset":15132,"ancestor_record_offset":1924,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":444,"record_offset":15196,"parent_record_offset":15164,"ancestor_record_offset":1924,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":444,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":1924,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":444,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":1924,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":444,"record_offset":3316,"parent_record_offset":3288,"ancestor_record_offset":1924,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":444,"record_offset":2964,"parent_record_offset":2848,"ancestor_record_offset":1924,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":444,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":1924,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":444,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":1924,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":444,"record_offset":2816,"parent_record_offset":2784,"ancestor_record_offset":1924,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":444,"record_offset":2784,"parent_record_offset":2748,"ancestor_record_offset":1924,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":444,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":1924,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":444,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1924,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":444,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1924,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":444,"record_offset":3288,"parent_record_offset":2748,"ancestor_record_offset":1924,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2288,"parent_record_offset":2256,"ancestor_record_offset":1924,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1924,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2448,"parent_record_offset":2416,"ancestor_record_offset":1924,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2544,"parent_record_offset":2448,"ancestor_record_offset":1924,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1924,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2232,"parent_record_offset":2200,"ancestor_record_offset":1924,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":444,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":1924,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":444,"record_offset":2848,"parent_record_offset":2816,"ancestor_record_offset":1924,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":444,"record_offset":3132,"parent_record_offset":2848,"ancestor_record_offset":1924,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":444,"record_offset":3104,"parent_record_offset":2848,"ancestor_record_offset":1924,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":444,"record_offset":2908,"parent_record_offset":2848,"ancestor_record_offset":1924,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":444,"record_offset":10432,"parent_record_offset":10404,"ancestor_record_offset":1924,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":444,"record_offset":18104,"parent_record_offset":18068,"ancestor_record_offset":1924,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":444,"record_offset":10456,"parent_record_offset":10432,"ancestor_record_offset":1924,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":444,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":1924,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":444,"record_offset":18160,"parent_record_offset":18068,"ancestor_record_offset":1924,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":444,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":1924,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$39::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":444,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":1924,"inlinee":25420,"source_line":180,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":444,"record_offset":6304,"parent_record_offset":6268,"ancestor_record_offset":1924,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":9240,"parent_record_offset":9204,"ancestor_record_offset":1924,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":8612,"parent_record_offset":8576,"ancestor_record_offset":1924,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":10624,"parent_record_offset":10584,"ancestor_record_offset":1924,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":10176,"parent_record_offset":10148,"ancestor_record_offset":1924,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":1924,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":8672,"parent_record_offset":8640,"ancestor_record_offset":1924,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":444,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":1924,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":444,"record_offset":8352,"parent_record_offset":8328,"ancestor_record_offset":1924,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":1924,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":1924,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":14012,"parent_record_offset":13988,"ancestor_record_offset":1924,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13936,"parent_record_offset":13784,"ancestor_record_offset":1924,"inlinee":25181,"source_line":237,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":14076,"parent_record_offset":13960,"ancestor_record_offset":1924,"inlinee":25373,"source_line":208,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":444,"record_offset":14112,"parent_record_offset":13784,"ancestor_record_offset":1924,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":444,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":1924,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":444,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":1924,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":9100,"parent_record_offset":9068,"ancestor_record_offset":1924,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":1924,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":1924,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":8820,"parent_record_offset":8796,"ancestor_record_offset":1924,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":444,"record_offset":8904,"parent_record_offset":8796,"ancestor_record_offset":1924,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":444,"record_offset":15404,"parent_record_offset":15376,"ancestor_record_offset":1924,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":444,"record_offset":15432,"parent_record_offset":15404,"ancestor_record_offset":1924,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":444,"record_offset":15644,"parent_record_offset":15404,"ancestor_record_offset":1924,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":444,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":1924,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":444,"record_offset":4948,"parent_record_offset":4912,"ancestor_record_offset":1924,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":444,"record_offset":5044,"parent_record_offset":4876,"ancestor_record_offset":1924,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":444,"record_offset":4876,"parent_record_offset":4824,"ancestor_record_offset":1924,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":6612,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":5076,"parent_record_offset":4824,"ancestor_record_offset":1924,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":7224,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":1924,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":5132,"parent_record_offset":4824,"ancestor_record_offset":1924,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":444,"record_offset":6696,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":444,"record_offset":6584,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":444,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":1924,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":1924,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":6392,"parent_record_offset":6364,"ancestor_record_offset":1924,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":7080,"parent_record_offset":6912,"ancestor_record_offset":1924,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":1924,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":1924,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":444,"record_offset":6424,"parent_record_offset":6392,"ancestor_record_offset":1924,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":7020,"parent_record_offset":6936,"ancestor_record_offset":1924,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":1924,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":5232,"parent_record_offset":4824,"ancestor_record_offset":1924,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":7316,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":4824,"parent_record_offset":4784,"ancestor_record_offset":1924,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":1924,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":1924,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6452,"parent_record_offset":6392,"ancestor_record_offset":1924,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":1924,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6556,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":4784,"parent_record_offset":4748,"ancestor_record_offset":1924,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":444,"record_offset":6912,"parent_record_offset":6500,"ancestor_record_offset":1924,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":444,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":1924,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":3660,"parent_record_offset":3624,"ancestor_record_offset":1924,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":16212,"parent_record_offset":16180,"ancestor_record_offset":1924,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":16728,"parent_record_offset":16468,"ancestor_record_offset":1924,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":444,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":1924,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":3880,"parent_record_offset":3732,"ancestor_record_offset":1924,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":444,"record_offset":4140,"parent_record_offset":3880,"ancestor_record_offset":1924,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":444,"record_offset":6336,"parent_record_offset":6304,"ancestor_record_offset":1924,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":444,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":1924,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":444,"record_offset":5508,"parent_record_offset":5476,"ancestor_record_offset":1924,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":444,"record_offset":9132,"parent_record_offset":9100,"ancestor_record_offset":1924,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":444,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":1924,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":444,"record_offset":3732,"parent_record_offset":3696,"ancestor_record_offset":1924,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":444,"record_offset":3696,"parent_record_offset":3660,"ancestor_record_offset":1924,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":444,"record_offset":15240,"parent_record_offset":15096,"ancestor_record_offset":1924,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":15064,"parent_record_offset":14792,"ancestor_record_offset":1924,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":15036,"parent_record_offset":14792,"ancestor_record_offset":1924,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":14840,"parent_record_offset":14792,"ancestor_record_offset":1924,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":14872,"parent_record_offset":14840,"ancestor_record_offset":1924,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":14972,"parent_record_offset":14944,"ancestor_record_offset":1924,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":444,"record_offset":14944,"parent_record_offset":14792,"ancestor_record_offset":1924,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":444,"record_offset":15376,"parent_record_offset":15096,"ancestor_record_offset":1924,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x634A
// 首条位置: {"module":444,"inlinee":25418}
// 编译实例 RVAs: 0x7f5e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1279740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377056368,"raw_hex":"e8fc861300","size":5,"soft_fail":false,"target_va":5378336448,"va":5377056703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cc30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11103356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377056368,"raw_hex":"e87c6ca900","size":5,"soft_fail":false,"target_va":5388160048,"va":5377056687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-431596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377056368,"raw_hex":"e8146af9ff","size":5,"soft_fail":false,"target_va":5376625152,"va":5377056743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4850576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377056368,"raw_hex":"e890034a00","size":5,"soft_fail":false,"target_va":5381907008,"va":5377056427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":14228,"parent_record_offset":13784,"ancestor_record_offset":13784,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":13852,"parent_record_offset":13784,"ancestor_record_offset":13784,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13936,"parent_record_offset":13784,"ancestor_record_offset":13784,"inlinee":25181,"source_line":237,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":444,"record_offset":14112,"parent_record_offset":13784,"ancestor_record_offset":13784,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":444,"record_offset":19260,"parent_record_offset":18200,"ancestor_record_offset":18200,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":14300,"parent_record_offset":14268,"ancestor_record_offset":13784,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":13784,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":444,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":13784,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":444,"record_offset":14268,"parent_record_offset":14228,"ancestor_record_offset":13784,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":444,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":13784,"inlinee":25421,"source_line":1820,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":19080,"parent_record_offset":18660,"ancestor_record_offset":18200,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":18988,"parent_record_offset":18932,"ancestor_record_offset":18200,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":18200,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":13784,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":444,"record_offset":18852,"parent_record_offset":18660,"ancestor_record_offset":18200,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":444,"record_offset":14040,"parent_record_offset":14012,"ancestor_record_offset":13784,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":444,"record_offset":19288,"parent_record_offset":19260,"ancestor_record_offset":18200,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":444,"record_offset":14388,"parent_record_offset":14228,"ancestor_record_offset":13784,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$39::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":444,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":13784,"inlinee":25420,"source_line":180,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":13784,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":13784,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":14012,"parent_record_offset":13988,"ancestor_record_offset":13784,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":444,"record_offset":14076,"parent_record_offset":13960,"ancestor_record_offset":13784,"inlinee":25373,"source_line":208,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":444,"record_offset":14136,"parent_record_offset":14112,"ancestor_record_offset":13784,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":444,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":13784,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":444,"record_offset":18732,"parent_record_offset":18704,"ancestor_record_offset":18200,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00444.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":444,"record_offset":18704,"parent_record_offset":18660,"ancestor_record_offset":18200,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00444.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_balcony_door.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_balcony_door.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_balcony_door.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
// 信息 ID: 8361929bb17787657f411acf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x7f2025 -> 0x7f5e70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=444, inline_record=13784; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors
// 信息 ID: d896c26ccedc299a01643179
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc172a4ba3b8f88c1aeeacf2: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > > @ 0x437c3c -> 0x7f3da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > > @ 0x47e255 -> 0x7f3da0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > @ 0x521cfb -> 0x7f3da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
