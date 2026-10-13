// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cb31031e010113ba1cbe55f4d710605dce2e36f1222b1b34696a81b382faad58
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 471
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
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":471,"record_offset":2696,"rva":9211248,"va":5377920368}
// 编译实例 RVAs: 0x8c8d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401aaf30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7468107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8b50b8eff","size":5,"soft_fail":false,"target_va":5370457904,"va":5377926006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4442724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e89c35bcff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377921759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4441627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8e539bcff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377921398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t196531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8b3ff0200","size":5,"soft_fail":false,"target_va":5378120480,"va":5377923944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t200794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e85a100300","size":5,"soft_fail":false,"target_va":5378126272,"va":5377925473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t204972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8ac200300","size":5,"soft_fail":false,"target_va":5378126944,"va":5377921967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t541311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e87f420800","size":5,"soft_fail":false,"target_va":5378462960,"va":5377921644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1554295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e877b71700","size":5,"soft_fail":false,"target_va":5379475248,"va":5377920948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3739866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8da103900","size":5,"soft_fail":false,"target_va":5381660768,"va":5377920897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3959333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8256a3c00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377928054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5361072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8b0cd5100","size":5,"soft_fail":false,"target_va":5383283184,"va":5377922107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5402641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e811705200","size":5,"soft_fail":false,"target_va":5383325024,"va":5377922378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5402761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e889705200","size":5,"soft_fail":false,"target_va":5383325056,"va":5377922290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11746747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8bb3db300","size":5,"soft_fail":false,"target_va":5389669280,"va":5377922528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414993d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12382318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e86ef0bc00","size":5,"soft_fail":false,"target_va":5390308304,"va":5377925981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14149a770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12387361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e82104bd00","size":5,"soft_fail":false,"target_va":5390313328,"va":5377925962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12418557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8fd7dbd00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377922014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12525585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e81120bf00","size":5,"soft_fail":false,"target_va":5390450688,"va":5377925098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fb00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38427360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8e05a4a02","size":5,"soft_fail":false,"target_va":5416352512,"va":5377925147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38457477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e885d04a02","size":5,"soft_fail":false,"target_va":5416381344,"va":5377923862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38694485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8556e4e02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377926918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38694818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8a26f4e02","size":5,"soft_fail":false,"target_va":5416621536,"va":5377926713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38694912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e800704e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377926683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38695199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e81f714e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377927004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38695462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e826724e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377926773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e82c1e0000","size":5,"soft_fail":false,"target_va":5377928256,"va":5377920527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38694308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8a46d4e02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377927063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t204481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8c11e0300","size":5,"soft_fail":false,"target_va":5378126416,"va":5377921930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t202732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8ec170300","size":5,"soft_fail":false,"target_va":5378126544,"va":5377923807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1553959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e827b61700","size":5,"soft_fail":false,"target_va":5379475184,"va":5377921220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3728877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8ede53800","size":5,"soft_fail":false,"target_va":5381650864,"va":5377921982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3732711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8e7f43800","size":5,"soft_fail":false,"target_va":5381654320,"va":5377921604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3309674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e86a803200","size":5,"soft_fail":false,"target_va":5381231744,"va":5377922065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutout::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t269066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e80a1b0400","size":5,"soft_fail":false,"target_va":5378193456,"va":5377924385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutKind::cottage_balcony_door_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t269367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8371c0400","size":5,"soft_fail":false,"target_va":5378193712,"va":5377924340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6042919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e827355c00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377920868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6051426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e862565c00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377920793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6038345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e849235c00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377926562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6042314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8ca325c00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377920497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38790763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e86be64f02","size":5,"soft_fail":false,"target_va":5416717696,"va":5377926928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38692406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e836664e02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377926949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t146895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8cf3d0200","size":5,"soft_fail":false,"target_va":5378070256,"va":5377923356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t150138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e87a4a0200","size":5,"soft_fail":false,"target_va":5378071840,"va":5377921697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e802b4ffff","size":5,"soft_fail":false,"target_va":5377906464,"va":5377925913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5373034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e86afc5100","size":5,"soft_fail":false,"target_va":5383297984,"va":5377924945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5360035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e8a3c95100","size":5,"soft_fail":false,"target_va":5383282224,"va":5377922184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5357853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377920368,"raw_hex":"e81dc15100","size":5,"soft_fail":false,"target_va":5383280256,"va":5377922398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":12948,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":471,"record_offset":7492,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":471,"record_offset":7548,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":471,"record_offset":5268,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":471,"record_offset":5240,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":15628,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":11820,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":8564,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":9196,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26677,"source_line":298,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":10460,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26686,"source_line":309,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":5184,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":471,"record_offset":13588,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":471,"record_offset":13176,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":8760,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":8984,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":9168,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":8928,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":4376,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":2972,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":3364,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":471,"record_offset":7576,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":471,"record_offset":11724,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs
//     evidence={"module":471,"record_offset":14368,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":27628,"source_line":233,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":471,"record_offset":14120,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":471,"record_offset":14092,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":471,"record_offset":8704,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":27587,"source_line":221,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::x
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":471,"record_offset":13844,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":27592,"source_line":151,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":13052,"parent_record_offset":13020,"ancestor_record_offset":2696,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":2696,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":2696,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":471,"record_offset":10308,"parent_record_offset":9228,"ancestor_record_offset":2696,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":471,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":2696,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":13020,"parent_record_offset":12976,"ancestor_record_offset":2696,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":14780,"parent_record_offset":14368,"ancestor_record_offset":2696,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":12976,"parent_record_offset":12948,"ancestor_record_offset":2696,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":471,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":2696,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4704,"parent_record_offset":4676,"ancestor_record_offset":2696,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4732,"parent_record_offset":4704,"ancestor_record_offset":2696,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":2696,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":2696,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4792,"parent_record_offset":4676,"ancestor_record_offset":2696,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":471,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":2696,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":471,"record_offset":14432,"parent_record_offset":14368,"ancestor_record_offset":2696,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":471,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":2696,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":471,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":2696,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":471,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":2696,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":471,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":2696,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":471,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":2696,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":2696,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":13316,"parent_record_offset":13232,"ancestor_record_offset":2696,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":13380,"parent_record_offset":13200,"ancestor_record_offset":2696,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":471,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":2696,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":471,"record_offset":7276,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":471,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":2696,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":471,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":2696,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":471,"record_offset":7008,"parent_record_offset":6980,"ancestor_record_offset":2696,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":471,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":2696,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":471,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":2696,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":471,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":2696,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":471,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":2696,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":471,"record_offset":4524,"parent_record_offset":4484,"ancestor_record_offset":2696,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":471,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":2696,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":11756,"parent_record_offset":11724,"ancestor_record_offset":2696,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":2696,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":2696,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":5772,"parent_record_offset":5268,"ancestor_record_offset":2696,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":7448,"parent_record_offset":6472,"ancestor_record_offset":2696,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":8632,"parent_record_offset":8596,"ancestor_record_offset":2696,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":471,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2696,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":471,"record_offset":5140,"parent_record_offset":4484,"ancestor_record_offset":2696,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":2696,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":4856,"parent_record_offset":4624,"ancestor_record_offset":2696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":471,"record_offset":5640,"parent_record_offset":5616,"ancestor_record_offset":2696,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":2696,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":2696,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":471,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":2696,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":13140,"parent_record_offset":12976,"ancestor_record_offset":2696,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":471,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":2696,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":471,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":2696,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":2696,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2696,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":4096,"parent_record_offset":4068,"ancestor_record_offset":2696,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":471,"record_offset":3688,"parent_record_offset":3572,"ancestor_record_offset":2696,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":471,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2696,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":471,"record_offset":3400,"parent_record_offset":3364,"ancestor_record_offset":2696,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":471,"record_offset":3540,"parent_record_offset":3508,"ancestor_record_offset":2696,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":471,"record_offset":3508,"parent_record_offset":3472,"ancestor_record_offset":2696,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":2696,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2696,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":2696,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":471,"record_offset":4012,"parent_record_offset":3472,"ancestor_record_offset":2696,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3076,"parent_record_offset":3044,"ancestor_record_offset":2696,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2696,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3204,"parent_record_offset":3172,"ancestor_record_offset":2696,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3260,"parent_record_offset":3204,"ancestor_record_offset":2696,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3172,"parent_record_offset":3140,"ancestor_record_offset":2696,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3016,"parent_record_offset":2972,"ancestor_record_offset":2696,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":471,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":2696,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":2696,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":3856,"parent_record_offset":3572,"ancestor_record_offset":2696,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":3828,"parent_record_offset":3572,"ancestor_record_offset":2696,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":471,"record_offset":3632,"parent_record_offset":3572,"ancestor_record_offset":2696,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":471,"record_offset":10336,"parent_record_offset":10308,"ancestor_record_offset":2696,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":471,"record_offset":17288,"parent_record_offset":17252,"ancestor_record_offset":2696,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":471,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":2696,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":471,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":2696,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":471,"record_offset":17344,"parent_record_offset":17252,"ancestor_record_offset":2696,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":471,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":2696,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":471,"record_offset":6472,"parent_record_offset":6436,"ancestor_record_offset":2696,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":9228,"parent_record_offset":9196,"ancestor_record_offset":2696,"inlinee":26679,"source_line":249,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":2696,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":2696,"inlinee":26687,"source_line":258,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":10084,"parent_record_offset":10056,"ancestor_record_offset":2696,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":2696,"inlinee":26703,"source_line":252,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":8664,"parent_record_offset":8632,"ancestor_record_offset":2696,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":471,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":2696,"inlinee":26705,"source_line":261,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":471,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":2696,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":14512,"parent_record_offset":14488,"ancestor_record_offset":2696,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":14540,"parent_record_offset":14512,"ancestor_record_offset":2696,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":14564,"parent_record_offset":14540,"ancestor_record_offset":2696,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":14488,"parent_record_offset":14368,"ancestor_record_offset":2696,"inlinee":27129,"source_line":237,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":14628,"parent_record_offset":14512,"ancestor_record_offset":2696,"inlinee":27581,"source_line":208,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":471,"record_offset":14664,"parent_record_offset":14368,"ancestor_record_offset":2696,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":471,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":2696,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":471,"record_offset":13520,"parent_record_offset":13200,"ancestor_record_offset":2696,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":9064,"parent_record_offset":9032,"ancestor_record_offset":2696,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":2696,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":2696,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":2696,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":471,"record_offset":8868,"parent_record_offset":8760,"ancestor_record_offset":2696,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":471,"record_offset":13260,"parent_record_offset":13232,"ancestor_record_offset":2696,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":471,"record_offset":13232,"parent_record_offset":13200,"ancestor_record_offset":2696,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":471,"record_offset":5464,"parent_record_offset":5436,"ancestor_record_offset":2696,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":471,"record_offset":5496,"parent_record_offset":5464,"ancestor_record_offset":2696,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":471,"record_offset":5436,"parent_record_offset":5396,"ancestor_record_offset":2696,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":471,"record_offset":6752,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":471,"record_offset":5560,"parent_record_offset":5396,"ancestor_record_offset":2696,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":471,"record_offset":7304,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":471,"record_offset":6836,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":471,"record_offset":6696,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":471,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":2696,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":471,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":2696,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":471,"record_offset":7160,"parent_record_offset":7052,"ancestor_record_offset":2696,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":471,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":2696,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":471,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":2696,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":471,"record_offset":6564,"parent_record_offset":6532,"ancestor_record_offset":2696,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":2696,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":5616,"parent_record_offset":5396,"ancestor_record_offset":2696,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":7396,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":5396,"parent_record_offset":5360,"ancestor_record_offset":2696,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":2696,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":2696,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":6592,"parent_record_offset":6532,"ancestor_record_offset":2696,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":2696,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":6724,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":5360,"parent_record_offset":5324,"ancestor_record_offset":2696,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":471,"record_offset":7052,"parent_record_offset":6640,"ancestor_record_offset":2696,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":471,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":2696,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":4412,"parent_record_offset":4376,"ancestor_record_offset":2696,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":15692,"parent_record_offset":15660,"ancestor_record_offset":2696,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":16144,"parent_record_offset":15940,"ancestor_record_offset":2696,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":471,"record_offset":4552,"parent_record_offset":4524,"ancestor_record_offset":2696,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":4624,"parent_record_offset":4484,"ancestor_record_offset":2696,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":471,"record_offset":4828,"parent_record_offset":4624,"ancestor_record_offset":2696,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":471,"record_offset":6504,"parent_record_offset":6472,"ancestor_record_offset":2696,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":471,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":2696,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":471,"record_offset":5892,"parent_record_offset":5860,"ancestor_record_offset":2696,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":471,"record_offset":9096,"parent_record_offset":9064,"ancestor_record_offset":2696,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":471,"record_offset":4580,"parent_record_offset":4552,"ancestor_record_offset":2696,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":4484,"parent_record_offset":4448,"ancestor_record_offset":2696,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":4448,"parent_record_offset":4412,"ancestor_record_offset":2696,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BEC
// 首条位置: {"module":471,"inlinee":27628}
// 编译实例 RVAs: 0x8c8a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t206348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377919584,"raw_hex":"e80c260300","size":5,"soft_fail":false,"target_va":5378126272,"va":5377919919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12530781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377919584,"raw_hex":"e85d34bf00","size":5,"soft_fail":false,"target_va":5390450688,"va":5377919902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fb00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38432555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377919584,"raw_hex":"e82b6f4a02","size":5,"soft_fail":false,"target_va":5416352512,"va":5377919952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5378336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377919584,"raw_hex":"e820115200","size":5,"soft_fail":false,"target_va":5383297984,"va":5377919643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":1532,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":471,"record_offset":592,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":6427,"source_line":1464,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":648,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":27129,"source_line":237,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":471,"record_offset":820,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":471,"record_offset":1444,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":1632,"parent_record_offset":1600,"ancestor_record_offset":412,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":412,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":471,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":412,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":1600,"parent_record_offset":1560,"ancestor_record_offset":412,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":1560,"parent_record_offset":1532,"ancestor_record_offset":412,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":1264,"parent_record_offset":844,"ancestor_record_offset":412,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":1172,"parent_record_offset":1116,"ancestor_record_offset":412,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":412,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":972,"parent_record_offset":888,"ancestor_record_offset":412,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":471,"record_offset":1036,"parent_record_offset":844,"ancestor_record_offset":412,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":471,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":412,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":471,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":412,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":471,"record_offset":616,"parent_record_offset":592,"ancestor_record_offset":412,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":1720,"parent_record_offset":1560,"ancestor_record_offset":412,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":412,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":412,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":412,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":471,"record_offset":784,"parent_record_offset":672,"ancestor_record_offset":412,"inlinee":27581,"source_line":208,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":471,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":412,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":471,"record_offset":1292,"parent_record_offset":844,"ancestor_record_offset":412,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":471,"record_offset":916,"parent_record_offset":888,"ancestor_record_offset":412,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":471,"record_offset":888,"parent_record_offset":844,"ancestor_record_offset":412,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00471.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: ab2f22c28ddfac51c1c0db30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x8c753b -> 0x8c8a60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors; module=471, inline_record=14368; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors
// 信息 ID: 658616d70abcd0c120e7d20f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc172a4ba3b8f88c1aeeacf2: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> @ 0x1eef78 -> 0x8c8d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > @ 0x2ac020 -> 0x8c8d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
