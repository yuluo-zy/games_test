// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0f423fb4bb8751416879a11a6d17b1eafdac38cdc6def4c24a333a21fdded2e3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_wall_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 469, 1545, 1559, 1600, 1608
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
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
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":4084,"rva":9181504,"va":5377890624}
// 编译实例 RVAs: 0x8c1940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2221229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8531bdeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377899912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2218558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8c225deff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377897257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9177710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e892f573ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377899897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4413013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8aba9bcff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377892048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4412015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e891adbcff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377891786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t228135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8277b0300","size":5,"soft_fail":false,"target_va":5378120480,"va":5377892340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t234702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8ce940300","size":5,"soft_fail":false,"target_va":5378126944,"va":5377892237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t232319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e87f8b0300","size":5,"soft_fail":false,"target_va":5378127504,"va":5377895180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t570308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8c4b30800","size":5,"soft_fail":false,"target_va":5378462960,"va":5377892647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1584020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8942b1800","size":5,"soft_fail":false,"target_va":5379475248,"va":5377891223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3769580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8ec843900","size":5,"soft_fail":false,"target_va":5381660768,"va":5377891183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3984182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e836cb3c00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377903205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5389379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8433c5200","size":5,"soft_fail":false,"target_va":5383283184,"va":5377893800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de62a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5386427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8bb305200","size":5,"soft_fail":false,"target_va":5383283360,"va":5377896928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5430900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e874de5200","size":5,"soft_fail":false,"target_va":5383325024,"va":5377894119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5431019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8ebde5200","size":5,"soft_fail":false,"target_va":5383325056,"va":5377894032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11776204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8ccb0b300","size":5,"soft_fail":false,"target_va":5389669280,"va":5377893071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12448291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e823f2bd00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377892280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a6d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12466932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8f43abe00","size":5,"soft_fail":false,"target_va":5390364000,"va":5377897063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bbcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12549319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8c77cbf00","size":5,"soft_fail":false,"target_va":5390449840,"va":5377900516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bc000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12549743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e86f7ebf00","size":5,"soft_fail":false,"target_va":5390450688,"va":5377900940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fb00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38451752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e828ba4a02","size":5,"soft_fail":false,"target_va":5416352512,"va":5377900755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38718624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8a0cc4e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377902140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t38487622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e846464b02","size":5,"soft_fail":false,"target_va":5416382400,"va":5377894773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38719370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e88acf4e02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377902033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38719643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e89bd04e02","size":5,"soft_fail":false,"target_va":5416621536,"va":5377901888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38719737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8f9d04e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377901858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38720084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e854d24e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377902119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38720287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e81fd34e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377901948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e84d370000","size":5,"soft_fail":false,"target_va":5377904944,"va":5377890782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38719046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e846ce4e02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377902325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3304836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8846d3200","size":5,"soft_fail":false,"target_va":5381197456,"va":5377892615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3762944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8006b3900","size":5,"soft_fail":false,"target_va":5381655760,"va":5377892811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t230619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8db840300","size":5,"soft_fail":false,"target_va":5378126544,"va":5377895920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1583588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8e4291800","size":5,"soft_fail":false,"target_va":5379475184,"va":5377891591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3758607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e80f5a3900","size":5,"soft_fail":false,"target_va":5381650864,"va":5377892252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3762340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8a4683900","size":5,"soft_fail":false,"target_va":5381654320,"va":5377891975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2478704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e870d22500","size":5,"soft_fail":false,"target_va":5380376608,"va":5377897899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_transition_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3332708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e864da3200","size":5,"soft_fail":false,"target_va":5381233488,"va":5377900775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3337982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8feee3200","size":5,"soft_fail":false,"target_va":5381231744,"va":5377893757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3010017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8e1ed2d00","size":5,"soft_fail":false,"target_va":5380902448,"va":5377892426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutout::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t296945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8f1870400","size":5,"soft_fail":false,"target_va":5378193456,"va":5377896506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutKind::cottage_rank
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t297100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e88c880400","size":5,"soft_fail":false,"target_va":5378193568,"va":5377896463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6072645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e845a95c00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377891142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6081152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e880ca5c00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377891067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6063170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e842845c00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377901737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6072059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8fba65c00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377890752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38815648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8a0475002","size":5,"soft_fail":false,"target_va":5416717696,"va":5377902043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38717291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e86bc74e02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377902064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t175653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e825ae0200","size":5,"soft_fail":false,"target_va":5378070256,"va":5377894598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t179144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8c8bb0200","size":5,"soft_fail":false,"target_va":5378071840,"va":5377892691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::add_lintels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e89ddcffff","size":5,"soft_fail":false,"target_va":5377888160,"va":5377897214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e815deffff","size":5,"soft_fail":false,"target_va":5377890240,"va":5377898918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5429688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8b8d95200","size":5,"soft_fail":false,"target_va":5383324048,"va":5377894355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5401353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8096b5200","size":5,"soft_fail":false,"target_va":5383297984,"va":5377896626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5388304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e810385200","size":5,"soft_fail":false,"target_va":5383282224,"va":5377893915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5386112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890624,"raw_hex":"e8802f5200","size":5,"soft_fail":false,"target_va":5383280256,"va":5377894139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":16880,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":21892,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":469,"record_offset":21976,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":469,"record_offset":11652,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":469,"record_offset":10172,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":469,"record_offset":11708,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":469,"record_offset":23696,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":469,"record_offset":6736,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":469,"record_offset":6708,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":19096,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":19040,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":18840,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":19180,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":9944,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":12512,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26677,"source_line":298,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":13780,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26686,"source_line":309,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":6652,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":469,"record_offset":10084,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":469,"record_offset":11736,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":18692,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27129,"source_line":237,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":22448,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":6593,"source_line":237,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":469,"record_offset":11768,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15196,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15420,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15604,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15364,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":15716,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":15660,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":16640,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":12424,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":21308,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":5844,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":4436,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":4832,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":8956,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":469,"record_offset":23128,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27606,"source_line":258,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":469,"record_offset":16728,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::is_bay_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":469,"record_offset":18868,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27137,"source_line":389,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::subdivide_wall_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/mod.rs
//     evidence={"module":469,"record_offset":18896,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27605,"source_line":12,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":16852,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":17156,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27050,"source_line":181,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":16824,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":469,"record_offset":17436,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":469,"record_offset":15084,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":469,"record_offset":17688,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27051,"source_line":156,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":469,"record_offset":15112,"parent_record_offset":4084,"ancestor_record_offset":4084,"inlinee":27587,"source_line":221,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":4084,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":4084,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":4084,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":4084,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":4084,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":19412,"parent_record_offset":19388,"ancestor_record_offset":4084,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":23228,"parent_record_offset":23196,"ancestor_record_offset":4084,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":19340,"parent_record_offset":19252,"ancestor_record_offset":4084,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":469,"record_offset":23196,"parent_record_offset":23160,"ancestor_record_offset":4084,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":469,"record_offset":23160,"parent_record_offset":23128,"ancestor_record_offset":4084,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":469,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":4084,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":4084,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":19280,"parent_record_offset":19252,"ancestor_record_offset":4084,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":19252,"parent_record_offset":19228,"ancestor_record_offset":4084,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":4084,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":4084,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":19228,"parent_record_offset":19204,"ancestor_record_offset":4084,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":469,"record_offset":27072,"parent_record_offset":27036,"ancestor_record_offset":4084,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":469,"record_offset":13628,"parent_record_offset":12548,"ancestor_record_offset":4084,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":469,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":4084,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":16944,"parent_record_offset":16904,"ancestor_record_offset":4084,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":4084,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10848,"parent_record_offset":10816,"ancestor_record_offset":4084,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":4084,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":19120,"parent_record_offset":19096,"ancestor_record_offset":4084,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":4084,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":469,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":4084,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":469,"record_offset":22800,"parent_record_offset":22776,"ancestor_record_offset":4084,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":469,"record_offset":10604,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":469,"record_offset":11676,"parent_record_offset":11652,"ancestor_record_offset":4084,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10368,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":4084,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10508,"parent_record_offset":10476,"ancestor_record_offset":4084,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":4084,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":4084,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11024,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":4084,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11396,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":4084,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":4084,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":4084,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":4084,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":4084,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":4084,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":4084,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":10792,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":4084,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":4084,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":469,"record_offset":10640,"parent_record_offset":10604,"ancestor_record_offset":4084,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6172,"parent_record_offset":6144,"ancestor_record_offset":4084,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":4084,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6120,"parent_record_offset":6092,"ancestor_record_offset":4084,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":4084,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6260,"parent_record_offset":6144,"ancestor_record_offset":4084,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":469,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":4084,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":4084,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":469,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":4084,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":4084,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":4084,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":4084,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":12212,"parent_record_offset":11792,"ancestor_record_offset":4084,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":12120,"parent_record_offset":12064,"ancestor_record_offset":4084,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":11888,"parent_record_offset":11864,"ancestor_record_offset":4084,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":11920,"parent_record_offset":11836,"ancestor_record_offset":4084,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":11984,"parent_record_offset":11792,"ancestor_record_offset":4084,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":469,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":4084,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":469,"record_offset":8740,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":469,"record_offset":19064,"parent_record_offset":19040,"ancestor_record_offset":4084,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":469,"record_offset":19144,"parent_record_offset":19120,"ancestor_record_offset":4084,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":469,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":4084,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":469,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":4084,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":469,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4084,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":19308,"parent_record_offset":19280,"ancestor_record_offset":4084,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":4084,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":24180,"parent_record_offset":24044,"ancestor_record_offset":4084,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":469,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":4084,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":469,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":4084,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":15328,"parent_record_offset":15304,"ancestor_record_offset":4084,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":15244,"parent_record_offset":15220,"ancestor_record_offset":4084,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":12452,"parent_record_offset":12424,"ancestor_record_offset":4084,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":4084,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":469,"record_offset":5992,"parent_record_offset":5952,"ancestor_record_offset":4084,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":469,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":4084,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10244,"parent_record_offset":10204,"ancestor_record_offset":4084,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":4084,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10476,"parent_record_offset":10392,"ancestor_record_offset":4084,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":4084,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":4084,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11588,"parent_record_offset":10204,"ancestor_record_offset":4084,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":7248,"parent_record_offset":6736,"ancestor_record_offset":4084,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10576,"parent_record_offset":10320,"ancestor_record_offset":4084,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":8912,"parent_record_offset":7940,"ancestor_record_offset":4084,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10012,"parent_record_offset":9976,"ancestor_record_offset":4084,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":4084,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":6608,"parent_record_offset":5952,"ancestor_record_offset":4084,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":4084,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":6324,"parent_record_offset":6092,"ancestor_record_offset":4084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":4084,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":4084,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":27104,"parent_record_offset":27072,"ancestor_record_offset":4084,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":4084,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":22040,"parent_record_offset":21976,"ancestor_record_offset":4084,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4084,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":4084,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":22064,"parent_record_offset":22040,"ancestor_record_offset":4084,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":17064,"parent_record_offset":16904,"ancestor_record_offset":4084,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":469,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":4084,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":4084,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":4084,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":4084,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":4084,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4084,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4084,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":4084,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":4084,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":5156,"parent_record_offset":5040,"ancestor_record_offset":4084,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":4084,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":4868,"parent_record_offset":4832,"ancestor_record_offset":4084,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":5008,"parent_record_offset":4976,"ancestor_record_offset":4084,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":4976,"parent_record_offset":4940,"ancestor_record_offset":4084,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4084,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4084,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":4084,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":5480,"parent_record_offset":4940,"ancestor_record_offset":4084,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4544,"parent_record_offset":4512,"ancestor_record_offset":4084,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4512,"parent_record_offset":4484,"ancestor_record_offset":4084,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4672,"parent_record_offset":4640,"ancestor_record_offset":4084,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4728,"parent_record_offset":4672,"ancestor_record_offset":4084,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4640,"parent_record_offset":4608,"ancestor_record_offset":4084,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4484,"parent_record_offset":4436,"ancestor_record_offset":4084,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4084,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":4084,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":5324,"parent_record_offset":5040,"ancestor_record_offset":4084,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":5296,"parent_record_offset":5040,"ancestor_record_offset":4084,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":5100,"parent_record_offset":5040,"ancestor_record_offset":4084,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":4084,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":26260,"parent_record_offset":26224,"ancestor_record_offset":4084,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":4084,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":4084,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":26316,"parent_record_offset":26224,"ancestor_record_offset":4084,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":469,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":4084,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":469,"record_offset":7940,"parent_record_offset":7908,"ancestor_record_offset":4084,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":12548,"parent_record_offset":12512,"ancestor_record_offset":4084,"inlinee":26679,"source_line":249,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":9976,"parent_record_offset":9944,"ancestor_record_offset":4084,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":13820,"parent_record_offset":13780,"ancestor_record_offset":4084,"inlinee":26687,"source_line":258,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":13404,"parent_record_offset":13376,"ancestor_record_offset":4084,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":4084,"inlinee":26703,"source_line":252,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":10044,"parent_record_offset":10012,"ancestor_record_offset":4084,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":4084,"inlinee":26705,"source_line":261,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":469,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":4084,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":469,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":4084,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":469,"record_offset":10112,"parent_record_offset":10084,"ancestor_record_offset":4084,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":18720,"parent_record_offset":18692,"ancestor_record_offset":4084,"inlinee":26747,"source_line":143,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":22472,"parent_record_offset":22448,"ancestor_record_offset":4084,"inlinee":6594,"source_line":161,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":4084,"inlinee":26748,"source_line":122,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":4084,"inlinee":6595,"source_line":128,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":469,"record_offset":22576,"parent_record_offset":22472,"ancestor_record_offset":4084,"inlinee":6597,"source_line":214,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":4084,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":4084,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":4084,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":469,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":4084,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":469,"record_offset":12240,"parent_record_offset":11792,"ancestor_record_offset":4084,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":469,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":4084,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":469,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":4084,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15500,"parent_record_offset":15468,"ancestor_record_offset":4084,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15468,"parent_record_offset":15444,"ancestor_record_offset":4084,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15444,"parent_record_offset":15420,"ancestor_record_offset":4084,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":4084,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":469,"record_offset":15304,"parent_record_offset":15196,"ancestor_record_offset":4084,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":469,"record_offset":11864,"parent_record_offset":11836,"ancestor_record_offset":4084,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":469,"record_offset":11836,"parent_record_offset":11792,"ancestor_record_offset":4084,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":469,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":4084,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":469,"record_offset":6972,"parent_record_offset":6940,"ancestor_record_offset":4084,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":469,"record_offset":6912,"parent_record_offset":6872,"ancestor_record_offset":4084,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":469,"record_offset":8212,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":469,"record_offset":7036,"parent_record_offset":6872,"ancestor_record_offset":4084,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":469,"record_offset":8768,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":469,"record_offset":8296,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":469,"record_offset":8156,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":469,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":4084,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":469,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":4084,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":469,"record_offset":8624,"parent_record_offset":8516,"ancestor_record_offset":4084,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":469,"record_offset":6800,"parent_record_offset":6768,"ancestor_record_offset":4084,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":469,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":4084,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":469,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":4084,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":4084,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":7092,"parent_record_offset":6872,"ancestor_record_offset":4084,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8860,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":6872,"parent_record_offset":6836,"ancestor_record_offset":4084,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":4084,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":4084,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8052,"parent_record_offset":7992,"ancestor_record_offset":4084,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":4084,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8184,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":6836,"parent_record_offset":6800,"ancestor_record_offset":4084,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":469,"record_offset":8516,"parent_record_offset":8100,"ancestor_record_offset":4084,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":469,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":4084,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11500,"parent_record_offset":11468,"ancestor_record_offset":4084,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":4084,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":5880,"parent_record_offset":5844,"ancestor_record_offset":4084,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":25096,"parent_record_offset":25064,"ancestor_record_offset":4084,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":25548,"parent_record_offset":25344,"ancestor_record_offset":4084,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":469,"record_offset":6020,"parent_record_offset":5992,"ancestor_record_offset":4084,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":6092,"parent_record_offset":5952,"ancestor_record_offset":4084,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":6296,"parent_record_offset":6092,"ancestor_record_offset":4084,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":7968,"parent_record_offset":7940,"ancestor_record_offset":4084,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":6768,"parent_record_offset":6736,"ancestor_record_offset":4084,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":469,"record_offset":7368,"parent_record_offset":7336,"ancestor_record_offset":4084,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":469,"record_offset":15532,"parent_record_offset":15500,"ancestor_record_offset":4084,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":469,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":4084,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":5952,"parent_record_offset":5916,"ancestor_record_offset":4084,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":5916,"parent_record_offset":5880,"ancestor_record_offset":4084,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
//     evidence={"module":469,"record_offset":10204,"parent_record_offset":10172,"ancestor_record_offset":4084,"inlinee":27601,"source_line":75,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
//     evidence={"module":469,"record_offset":10272,"parent_record_offset":10244,"ancestor_record_offset":4084,"inlinee":27603,"source_line":78,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":469,"record_offset":18944,"parent_record_offset":18896,"ancestor_record_offset":4084,"inlinee":27078,"source_line":247,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BD1
// 首条位置: {"module":469,"inlinee":27601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10244,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11588,"parent_record_offset":10204,"ancestor_record_offset":10204,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":10204,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10204,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10204,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":10204,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10848,"parent_record_offset":10816,"ancestor_record_offset":10204,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10204,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":469,"record_offset":10604,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10368,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":10204,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10508,"parent_record_offset":10476,"ancestor_record_offset":10204,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":10204,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":10204,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11024,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10204,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11396,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10204,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10204,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":10204,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":10204,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10204,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10204,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10204,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":10792,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":10204,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":10204,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":469,"record_offset":10640,"parent_record_offset":10604,"ancestor_record_offset":10204,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":469,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":10204,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":10204,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":10204,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10476,"parent_record_offset":10392,"ancestor_record_offset":10204,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11256,"parent_record_offset":11120,"ancestor_record_offset":10204,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10576,"parent_record_offset":10320,"ancestor_record_offset":10204,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":10204,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":10204,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":10204,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":10204,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":10204,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10204,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":10204,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11500,"parent_record_offset":11468,"ancestor_record_offset":10204,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":10204,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
//     evidence={"module":469,"record_offset":10272,"parent_record_offset":10244,"ancestor_record_offset":10204,"inlinee":27603,"source_line":78,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BD3
// 首条位置: {"module":469,"inlinee":27603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":10272,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":10272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":10272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10848,"parent_record_offset":10816,"ancestor_record_offset":10272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":469,"record_offset":10604,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10368,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":10272,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10508,"parent_record_offset":10476,"ancestor_record_offset":10272,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":469,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":10272,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":10272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11024,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11396,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":469,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":10272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":10272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":469,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":10272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":469,"record_offset":10792,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":469,"record_offset":10320,"parent_record_offset":10296,"ancestor_record_offset":10272,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":469,"record_offset":10640,"parent_record_offset":10604,"ancestor_record_offset":10272,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":469,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":10272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":10272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":10272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10476,"parent_record_offset":10392,"ancestor_record_offset":10272,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":11256,"parent_record_offset":11120,"ancestor_record_offset":10272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":10576,"parent_record_offset":10320,"ancestor_record_offset":10272,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":10272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":10272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":10272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":10272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":10272,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":10272,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":469,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":10272,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11500,"parent_record_offset":11468,"ancestor_record_offset":10272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":469,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":10272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A1B
// 首条位置: {"module":1559,"inlinee":80411}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 349
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x139AA
// 首条位置: {"module":1545,"inlinee":80298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1545,"record_offset":85856,"parent_record_offset":85308,"ancestor_record_offset":85308,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85624,"parent_record_offset":85308,"ancestor_record_offset":85308,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85348,"parent_record_offset":85308,"ancestor_record_offset":85308,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85532,"parent_record_offset":85308,"ancestor_record_offset":85308,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85440,"parent_record_offset":85308,"ancestor_record_offset":85308,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1545,"record_offset":85780,"parent_record_offset":85756,"ancestor_record_offset":85308,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1545,"record_offset":85380,"parent_record_offset":85348,"ancestor_record_offset":85308,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1545,"record_offset":85820,"parent_record_offset":85652,"ancestor_record_offset":85308,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1545,"record_offset":85472,"parent_record_offset":85440,"ancestor_record_offset":85308,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1545,"record_offset":85564,"parent_record_offset":85532,"ancestor_record_offset":85308,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85720,"parent_record_offset":85684,"ancestor_record_offset":85308,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85684,"parent_record_offset":85652,"ancestor_record_offset":85308,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1545,"record_offset":85652,"parent_record_offset":85624,"ancestor_record_offset":85308,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1545,"record_offset":85756,"parent_record_offset":85720,"ancestor_record_offset":85308,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 记录形式: compiled_procedure; instances=1
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":3220,"rva":9181120,"va":5377890240}
// 编译实例 RVAs: 0x8c17c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a49f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12464446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e83e31be00","size":5,"soft_fail":false,"target_va":5390354928,"va":5377890477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38731091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e853fd4e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377890504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38731610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e85aff4e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377890593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38731690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e8aaff4e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377890545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1375398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e8a6fc1400","size":5,"soft_fail":false,"target_va":5379265744,"va":5377890341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3574756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e8e48b3600","size":5,"soft_fail":false,"target_va":5381465136,"va":5377890375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3575042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377890240,"raw_hex":"e8028d3600","size":5,"soft_fail":false,"target_va":5381465408,"va":5377890361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":3996,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":3460,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":3388,"parent_record_offset":3220,"ancestor_record_offset":3220,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3220,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3220,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":3928,"parent_record_offset":3508,"ancestor_record_offset":3220,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":469,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3220,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":469,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":3220,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":469,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":3220,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":469,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":3220,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":469,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":3220,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":469,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":3220,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":3220,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3220,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3220,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3220,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 502
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13B4F
// 首条位置: {"module":1600,"inlinee":80719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1600,"record_offset":40712,"parent_record_offset":40672,"ancestor_record_offset":40672,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$31::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1600,"record_offset":41684,"parent_record_offset":40672,"ancestor_record_offset":40672,"inlinee":11818,"source_line":1154,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1600,"record_offset":41596,"parent_record_offset":40672,"ancestor_record_offset":40672,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1600,"record_offset":41508,"parent_record_offset":40712,"ancestor_record_offset":40672,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1600,"record_offset":41448,"parent_record_offset":40740,"ancestor_record_offset":40672,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1600,"record_offset":41356,"parent_record_offset":41300,"ancestor_record_offset":40672,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1600,"record_offset":40788,"parent_record_offset":40740,"ancestor_record_offset":40672,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1600,"record_offset":40852,"parent_record_offset":40816,"ancestor_record_offset":40672,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1600,"record_offset":40972,"parent_record_offset":40816,"ancestor_record_offset":40672,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1600,"record_offset":41624,"parent_record_offset":41596,"ancestor_record_offset":40672,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1600,"record_offset":41732,"parent_record_offset":41708,"ancestor_record_offset":40672,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1600,"record_offset":40740,"parent_record_offset":40712,"ancestor_record_offset":40672,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1600,"record_offset":41708,"parent_record_offset":41684,"ancestor_record_offset":40672,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1600,"record_offset":40816,"parent_record_offset":40740,"ancestor_record_offset":40672,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1600,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":40672,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1600,"record_offset":41532,"parent_record_offset":41508,"ancestor_record_offset":40672,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1600,"record_offset":41096,"parent_record_offset":41064,"ancestor_record_offset":40672,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1600,"record_offset":41064,"parent_record_offset":40740,"ancestor_record_offset":40672,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 记录形式: compiled_procedure; instances=1
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":552,"rva":9179040,"va":5377888160}
// 编译实例 RVAs: 0x8c0fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fbce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t240737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e861ac0300","size":5,"soft_fail":false,"target_va":5378129120,"va":5377888378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12452155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e83b01be00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377888416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38492868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e8c45a4b02","size":5,"soft_fail":false,"target_va":5416381344,"va":5377888471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38732007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e8e7004f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377890228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t238217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e889a20300","size":5,"soft_fail":false,"target_va":5378126544,"va":5377888322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3762467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e823693900","size":5,"soft_fail":false,"target_va":5381650864,"va":5377888392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5393846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888160,"raw_hex":"e8b64d5200","size":5,"soft_fail":false,"target_va":5383282208,"va":5377888357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":936,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":469,"record_offset":1152,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":740,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":469,"record_offset":1272,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":3128,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":469,"record_offset":1896,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":2068,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27176,"source_line":100,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":2012,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26695,"source_line":133,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":2040,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":469,"record_offset":1984,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":1028,"parent_record_offset":996,"ancestor_record_offset":552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":469,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":996,"parent_record_offset":960,"ancestor_record_offset":552,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":552,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":1716,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":1624,"parent_record_offset":1568,"ancestor_record_offset":552,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":552,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":1424,"parent_record_offset":1340,"ancestor_record_offset":552,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":469,"record_offset":1488,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":3156,"parent_record_offset":3128,"ancestor_record_offset":552,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":469,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":552,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":469,"record_offset":808,"parent_record_offset":772,"ancestor_record_offset":552,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":1224,"parent_record_offset":1188,"ancestor_record_offset":552,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":1116,"parent_record_offset":960,"ancestor_record_offset":552,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":1188,"parent_record_offset":1152,"ancestor_record_offset":552,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":872,"parent_record_offset":772,"ancestor_record_offset":552,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":552,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":469,"record_offset":840,"parent_record_offset":808,"ancestor_record_offset":552,"inlinee":27106,"source_line":244,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":469,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":552,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":469,"record_offset":1744,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":469,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":552,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":469,"record_offset":1340,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":469,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":552,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00469.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_wall_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_wall_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_wall_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 信息 ID: 8b5159a96cd4c25b0be52b81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c32fe -> 0x8c0fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8cc90e -> 0x8c0fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 信息 ID: 7b67b1b1bc0093e110b94031
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c39a6 -> 0x8c17c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x8c3ea5 -> 0x8c17c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x8ccb2c -> 0x8c17c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 信息 ID: 208042f896e8046e502c0699
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$1::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; module=1600, inline_record=40672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 信息 ID: 87a89b771b3d7083b86b7b0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dfc66390619dc72eca7997e2: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0xee49f -> 0x8c1940 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x2cc29f -> 0x8c1940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 信息 ID: f0cfc9e7e2ebaa84c9211632
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=469, inline_record=10204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0::closure$0
// 信息 ID: b69ce12c22562ba9bfa8e0fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=469, inline_record=10272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$2
// 信息 ID: 446abd87649bdf4a1b2cebf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2> > > >; module=1559, inline_record=1468; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2> > > >; module=1559, inline_record=2148; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1608, inline_record=16200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$3
// 信息 ID: 245537401fc127638cc25811
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$3> >; module=1545, inline_record=85308; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$2>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$3> >; module=1545, inline_record=86900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
