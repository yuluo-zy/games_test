// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/decoclutter_picker.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c7dde2a7ce693c65d9307718976fa0cb3238f61a454478f09a4a32bdd7c1b07e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decoclutter_picker.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1235, 1239, 2873, 4207
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
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4207,"record_offset":4312,"rva":43447040,"va":5412156160}
// 编译实例 RVAs: 0x296f300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-36480605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8a359d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412159304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=2
//     target_aliases=583 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42382776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8484a79fd","size":5,"soft_fail":false,"target_va":5369774336,"va":5412157107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38678498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e81ed0b1fd","size":5,"soft_fail":false,"target_va":5373479040,"va":5412157533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35980269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e813fcdafd","size":5,"soft_fail":false,"target_va":5376179280,"va":5412159544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407204d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35977917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e84305dbfd","size":5,"soft_fail":false,"target_va":5376181456,"va":5412159368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35922486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8cadddbfd","size":5,"soft_fail":false,"target_va":5376236864,"va":5412159345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d680
//     owner=unresolved_namespace; count=2
//     target_aliases=24 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33693743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8d1dffdfd","size":5,"soft_fail":false,"target_va":5378463360,"va":5412157098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e2d80
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33081720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8883607fe","size":5,"soft_fail":false,"target_va":5379075456,"va":5412157171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33081695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8a13607fe","size":5,"soft_fail":false,"target_va":5379075472,"va":5412157162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411767b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25136482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e89e7280fe","size":5,"soft_fail":false,"target_va":5387020208,"va":5412156685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411769c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25138650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8266a80fe","size":5,"soft_fail":false,"target_va":5387020736,"va":5412159381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141176b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25137549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8736e80fe","size":5,"soft_fail":false,"target_va":5387021088,"va":5412158632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25135207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8997780fe","size":5,"soft_fail":false,"target_va":5387022848,"va":5412158050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25132860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8c48080fe","size":5,"soft_fail":false,"target_va":5387023904,"va":5412156759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117ae20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25118641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e84fb880fe","size":5,"soft_fail":false,"target_va":5387038240,"va":5412156876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25116614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e83ac080fe","size":5,"soft_fail":false,"target_va":5387040112,"va":5412156721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25113550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e832cc80fe","size":5,"soft_fail":false,"target_va":5387043168,"va":5412156713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25113288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e838cd80fe","size":5,"soft_fail":false,"target_va":5387044016,"va":5412157299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25110817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8dfd680fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412156796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25110878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8a2d680fe","size":5,"soft_fail":false,"target_va":5387046592,"va":5412157465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118ebb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25037314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8fef581fe","size":5,"soft_fail":false,"target_va":5387119536,"va":5412156845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141196f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25003638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e88a7982fe","size":5,"soft_fail":false,"target_va":5387153184,"va":5412156817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24954715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8a53883fe","size":5,"soft_fail":false,"target_va":5387204688,"va":5412159398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419aff00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16513176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8680704ff","size":5,"soft_fail":false,"target_va":5395644160,"va":5412157331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14231f710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decoclutter_picker.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6620893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e823f99aff","size":5,"soft_fail":false,"target_va":5405538064,"va":5412158952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<ref$<str$> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8b1ecffff","size":5,"soft_fail":false,"target_va":5412154272,"va":5412159210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4462378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e82a174400","size":5,"soft_fail":false,"target_va":5416621536,"va":5412159153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4464472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8581f4400","size":5,"soft_fail":false,"target_va":5416621600,"va":5412157123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::DecoClutterCategory::icon
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33082051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e83d3507fe","size":5,"soft_fail":false,"target_va":5379075392,"va":5412157438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::DecoClutterCategory::label
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33081999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8713507fe","size":5,"soft_fail":false,"target_va":5379075424,"va":5412157418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32744657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e82f5b0cfe","size":5,"soft_fail":false,"target_va":5379414368,"va":5412159020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_registry::ClutterRegistry::picker_items
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32745574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e89a570cfe","size":5,"soft_fail":false,"target_va":5379411664,"va":5412157233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32674122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8b66e0dfe","size":5,"soft_fail":false,"target_va":5379482496,"va":5412156613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32658183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8f9ac0dfe","size":5,"soft_fail":false,"target_va":5379498400,"va":5412156578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions::title
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31687245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8b37d1cfe","size":5,"soft_fail":false,"target_va":5380471856,"va":5412159096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29024457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8371f45fe","size":5,"soft_fail":false,"target_va":5383134720,"va":5412159172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35947444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e84c7cdbfd","size":5,"soft_fail":false,"target_va":5376209536,"va":5412156975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::ScrollingListView::scroll_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35974197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8cb13dbfd","size":5,"soft_fail":false,"target_va":5376184320,"va":5412158512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::ScrollingListView::set_scroll
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35974362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e82613dbfd","size":5,"soft_fail":false,"target_va":5376184112,"va":5412158469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::ScrollingListView::take_home_focus_request
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35974219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8b513dbfd","size":5,"soft_fail":false,"target_va":5376184480,"va":5412158694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::text_hint_popup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35976526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412156160,"raw_hex":"e8b20adbfd","size":5,"soft_fail":false,"target_va":5376182752,"va":5412159273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4207,"record_offset":5992,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4207,"record_offset":5660,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4207,"record_offset":4580,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4207,"record_offset":6080,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4207,"record_offset":6988,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":11104,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":10912,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":11068,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":5936,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":11036,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":6620,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":11164,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4207,"record_offset":11224,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4207,"record_offset":10856,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::position
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4207,"record_offset":10640,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":6398,"source_line":377,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":4207,"record_offset":5964,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6884,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10540,"source_line":100,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6912,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::next_cycle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6476,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10538,"source_line":148,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::previous_cycle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6708,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":10539,"source_line":178,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4207,"record_offset":6252,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":10444,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::expect_child_with_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":11192,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":22031,"source_line":1489,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::LayoutStyle::size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":4207,"record_offset":6336,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":22141,"source_line":371,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::layout::horizontal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":4207,"record_offset":6280,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":22131,"source_line":302,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":11568,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21795,"source_line":398,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":11628,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21798,"source_line":357,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":11508,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":9296,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21802,"source_line":1339,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::mute_click
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":7080,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21706,"source_line":1332,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":8220,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21710,"source_line":1344,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::ScrollingListView::nav_home_row
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":10944,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21807,"source_line":830,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::TabSwitcherView::delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":6196,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21804,"source_line":752,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$54::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":9356,"parent_record_offset":4312,"ancestor_record_offset":4312,"inlinee":21691,"source_line":216,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4207,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":4312,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4207,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":4312,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4207,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":4312,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4207,"record_offset":11376,"parent_record_offset":11320,"ancestor_record_offset":4312,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4312,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":11348,"parent_record_offset":11320,"ancestor_record_offset":4312,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":4312,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":4312,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4312,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4207,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":4312,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4207,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":4312,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4207,"record_offset":4888,"parent_record_offset":4860,"ancestor_record_offset":4312,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4207,"record_offset":4860,"parent_record_offset":4728,"ancestor_record_offset":4312,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4207,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":4312,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4207,"record_offset":4728,"parent_record_offset":4684,"ancestor_record_offset":4312,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4207,"record_offset":5704,"parent_record_offset":5660,"ancestor_record_offset":4312,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4207,"record_offset":4684,"parent_record_offset":4624,"ancestor_record_offset":4312,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4207,"record_offset":4624,"parent_record_offset":4580,"ancestor_record_offset":4312,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4207,"record_offset":7912,"parent_record_offset":7888,"ancestor_record_offset":4312,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4207,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":4312,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4207,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":4312,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4207,"record_offset":7980,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4207,"record_offset":5752,"parent_record_offset":5704,"ancestor_record_offset":4312,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4207,"record_offset":5308,"parent_record_offset":5276,"ancestor_record_offset":4312,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4207,"record_offset":4804,"parent_record_offset":4728,"ancestor_record_offset":4312,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4207,"record_offset":11964,"parent_record_offset":11932,"ancestor_record_offset":4312,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4207,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":4312,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":4207,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":4312,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4207,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":4312,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":5796,"parent_record_offset":5752,"ancestor_record_offset":4312,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":5868,"parent_record_offset":5752,"ancestor_record_offset":4312,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":6528,"parent_record_offset":6476,"ancestor_record_offset":4312,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":8152,"parent_record_offset":7148,"ancestor_record_offset":4312,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":4312,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4207,"record_offset":5900,"parent_record_offset":5704,"ancestor_record_offset":4312,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4207,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":4312,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4207,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":4312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4207,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":4312,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4207,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":4312,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4207,"record_offset":7184,"parent_record_offset":7148,"ancestor_record_offset":4312,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4207,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":4312,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4207,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":4312,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4207,"record_offset":5212,"parent_record_offset":4728,"ancestor_record_offset":4312,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4207,"record_offset":5500,"parent_record_offset":4728,"ancestor_record_offset":4312,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4207,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":4312,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4207,"record_offset":5276,"parent_record_offset":5244,"ancestor_record_offset":4312,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4207,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":4312,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4207,"record_offset":5528,"parent_record_offset":5500,"ancestor_record_offset":4312,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4207,"record_offset":10676,"parent_record_offset":10640,"ancestor_record_offset":4312,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_meshes::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_meshes.rs
//     evidence={"module":4207,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":4312,"inlinee":15778,"source_line":630,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":4207,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":4312,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":4207,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":4312,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::first
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":4312,"inlinee":136676,"source_line":193,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":4312,"inlinee":136677,"source_line":208,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":4312,"inlinee":10247,"source_line":133,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::previous
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":4207,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":4312,"inlinee":10251,"source_line":163,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4207,"record_offset":7624,"parent_record_offset":7596,"ancestor_record_offset":4312,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4207,"record_offset":7656,"parent_record_offset":7624,"ancestor_record_offset":4312,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":4207,"record_offset":7596,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4207,"record_offset":7456,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4207,"record_offset":8008,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":4207,"record_offset":7540,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":4207,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":4207,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":4312,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4207,"record_offset":9516,"parent_record_offset":9480,"ancestor_record_offset":4312,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4207,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":4312,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4207,"record_offset":7864,"parent_record_offset":7756,"ancestor_record_offset":4312,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":4207,"record_offset":7268,"parent_record_offset":7236,"ancestor_record_offset":4312,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":4312,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":8100,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":4312,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":4312,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":9576,"parent_record_offset":9516,"ancestor_record_offset":4312,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7296,"parent_record_offset":7236,"ancestor_record_offset":4312,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":4312,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7428,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4207,"record_offset":7756,"parent_record_offset":7344,"ancestor_record_offset":4312,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4207,"record_offset":9480,"parent_record_offset":9448,"ancestor_record_offset":4312,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4207,"record_offset":7212,"parent_record_offset":7148,"ancestor_record_offset":4312,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4207,"record_offset":9448,"parent_record_offset":9388,"ancestor_record_offset":4312,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::decoclutter_picker::decoclutter_picker_menu::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/decoclutter_picker.rs
//     evidence={"module":4207,"record_offset":10740,"parent_record_offset":10640,"ancestor_record_offset":4312,"inlinee":136678,"source_line":178,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::decoclutter_picker::decoclutter_picker_menu::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/decoclutter_picker.rs
//     evidence={"module":4207,"record_offset":11132,"parent_record_offset":11104,"ancestor_record_offset":4312,"inlinee":136679,"source_line":280,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":8184,"parent_record_offset":7112,"ancestor_record_offset":4312,"inlinee":21709,"source_line":286,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":8252,"parent_record_offset":8220,"ancestor_record_offset":4312,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":9388,"parent_record_offset":9356,"ancestor_record_offset":4312,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4207,"record_offset":7148,"parent_record_offset":7112,"ancestor_record_offset":4312,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":7112,"parent_record_offset":7080,"ancestor_record_offset":4312,"inlinee":21707,"source_line":194,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4207,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":4312,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4207,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":4312,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::layout::LayoutStyle::horizontal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/layout.rs
//     evidence={"module":4207,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":4312,"inlinee":22114,"source_line":307,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":11536,"parent_record_offset":11508,"ancestor_record_offset":4312,"inlinee":21714,"source_line":234,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":4312,"inlinee":21722,"source_line":234,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::ScrollingListView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":4312,"inlinee":21738,"source_line":234,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::TabSwitcherView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":4207,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":4312,"inlinee":21725,"source_line":229,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":4207,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":4312,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111B2
// 首条位置: {"module":1235,"inlinee":70066}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1122D
// 首条位置: {"module":1239,"inlinee":70189}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::force_interactive
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":121956,"parent_record_offset":120920,"ancestor_record_offset":120920,"inlinee":22046,"source_line":309,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":120952,"parent_record_offset":120920,"ancestor_record_offset":120920,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1239,"record_offset":121712,"parent_record_offset":121688,"ancestor_record_offset":120920,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":121280,"parent_record_offset":121256,"ancestor_record_offset":120920,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1239,"record_offset":121364,"parent_record_offset":121340,"ancestor_record_offset":120920,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1239,"record_offset":121780,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":121512,"parent_record_offset":121484,"ancestor_record_offset":120920,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1239,"record_offset":121484,"parent_record_offset":121456,"ancestor_record_offset":120920,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1239,"record_offset":123116,"parent_record_offset":123084,"ancestor_record_offset":123056,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1239,"record_offset":121304,"parent_record_offset":121280,"ancestor_record_offset":120920,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1239,"record_offset":121628,"parent_record_offset":121604,"ancestor_record_offset":120920,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1239,"record_offset":120980,"parent_record_offset":120952,"ancestor_record_offset":120920,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1239,"record_offset":121736,"parent_record_offset":121712,"ancestor_record_offset":120920,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":121424,"parent_record_offset":121396,"ancestor_record_offset":120920,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":121456,"parent_record_offset":121424,"ancestor_record_offset":120920,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1239,"record_offset":121396,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":121256,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":121808,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1239,"record_offset":121340,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1239,"record_offset":121200,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1239,"record_offset":121688,"parent_record_offset":121664,"ancestor_record_offset":120920,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":121036,"parent_record_offset":121008,"ancestor_record_offset":120920,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":121664,"parent_record_offset":121556,"ancestor_record_offset":120920,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1239,"record_offset":121068,"parent_record_offset":121036,"ancestor_record_offset":120920,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121604,"parent_record_offset":121580,"ancestor_record_offset":120920,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121900,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121580,"parent_record_offset":121556,"ancestor_record_offset":120920,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121120,"parent_record_offset":121096,"ancestor_record_offset":120920,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121096,"parent_record_offset":121036,"ancestor_record_offset":120920,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121144,"parent_record_offset":121120,"ancestor_record_offset":120920,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121228,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1239,"record_offset":121556,"parent_record_offset":121144,"ancestor_record_offset":120920,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1239,"record_offset":121008,"parent_record_offset":120952,"ancestor_record_offset":120920,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1239,"record_offset":121980,"parent_record_offset":121956,"ancestor_record_offset":120920,"inlinee":21896,"source_line":609,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x215E6
// 首条位置: {"module":4207,"inlinee":136678}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":4207,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":10740,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4207,"record_offset":10812,"parent_record_offset":10788,"ancestor_record_offset":10740,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":4207,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":10740,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/04207.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B39A
// 首条位置: {"module":2873,"inlinee":111514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":71348,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2873,"record_offset":80624,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2873,"record_offset":80804,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2873,"record_offset":81172,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2873,"record_offset":80512,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":2873,"record_offset":73260,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":73628,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":80684,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":80744,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":76188,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2873,"record_offset":73176,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":2873,"record_offset":76104,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::has_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":79320,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21902,"source_line":354,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":80428,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21898,"source_line":345,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":71432,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":72172,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":74696,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21802,"source_line":1339,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":73664,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21710,"source_line":1344,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$54::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":77244,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21691,"source_line":216,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$55::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":76216,"parent_record_offset":71172,"ancestor_record_offset":71172,"inlinee":21713,"source_line":222,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":71740,"parent_record_offset":71716,"ancestor_record_offset":71172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":71764,"parent_record_offset":71740,"ancestor_record_offset":71172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":71788,"parent_record_offset":71764,"ancestor_record_offset":71172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2873,"record_offset":71716,"parent_record_offset":71660,"ancestor_record_offset":71172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2873,"record_offset":71396,"parent_record_offset":71372,"ancestor_record_offset":71172,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":81036,"parent_record_offset":81004,"ancestor_record_offset":71172,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":71688,"parent_record_offset":71660,"ancestor_record_offset":71172,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":71660,"parent_record_offset":71636,"ancestor_record_offset":71172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":81084,"parent_record_offset":81060,"ancestor_record_offset":71172,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":81060,"parent_record_offset":81036,"ancestor_record_offset":71172,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2873,"record_offset":71636,"parent_record_offset":71612,"ancestor_record_offset":71172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":81004,"parent_record_offset":80960,"ancestor_record_offset":71172,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":80928,"parent_record_offset":80892,"ancestor_record_offset":71172,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2873,"record_offset":80960,"parent_record_offset":80928,"ancestor_record_offset":71172,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2873,"record_offset":80652,"parent_record_offset":80624,"ancestor_record_offset":71172,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2873,"record_offset":80828,"parent_record_offset":80804,"ancestor_record_offset":71172,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2873,"record_offset":81280,"parent_record_offset":81256,"ancestor_record_offset":71172,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2873,"record_offset":81256,"parent_record_offset":81172,"ancestor_record_offset":71172,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2873,"record_offset":80892,"parent_record_offset":80828,"ancestor_record_offset":71172,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2873,"record_offset":75900,"parent_record_offset":74808,"ancestor_record_offset":71172,"inlinee":10219,"source_line":264,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2873,"record_offset":75924,"parent_record_offset":75900,"ancestor_record_offset":71172,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2873,"record_offset":75952,"parent_record_offset":75924,"ancestor_record_offset":71172,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2873,"record_offset":75980,"parent_record_offset":75952,"ancestor_record_offset":71172,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2873,"record_offset":72932,"parent_record_offset":72908,"ancestor_record_offset":71172,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2873,"record_offset":72500,"parent_record_offset":72476,"ancestor_record_offset":71172,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2873,"record_offset":72584,"parent_record_offset":72560,"ancestor_record_offset":71172,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2873,"record_offset":73000,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2873,"record_offset":73328,"parent_record_offset":73296,"ancestor_record_offset":71172,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":2873,"record_offset":73356,"parent_record_offset":73296,"ancestor_record_offset":71172,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":2873,"record_offset":73440,"parent_record_offset":73296,"ancestor_record_offset":71172,"inlinee":80042,"source_line":84,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":2873,"record_offset":73296,"parent_record_offset":73260,"ancestor_record_offset":71172,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2873,"record_offset":72732,"parent_record_offset":72704,"ancestor_record_offset":71172,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2873,"record_offset":72704,"parent_record_offset":72676,"ancestor_record_offset":71172,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":73588,"parent_record_offset":73440,"ancestor_record_offset":71172,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":76024,"parent_record_offset":74808,"ancestor_record_offset":71172,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":75836,"parent_record_offset":74808,"ancestor_record_offset":71172,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2873,"record_offset":71372,"parent_record_offset":71348,"ancestor_record_offset":71172,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2873,"record_offset":73548,"parent_record_offset":73524,"ancestor_record_offset":71172,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":72524,"parent_record_offset":72500,"ancestor_record_offset":71172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":71564,"parent_record_offset":71496,"ancestor_record_offset":71172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2873,"record_offset":73224,"parent_record_offset":73200,"ancestor_record_offset":71172,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2873,"record_offset":73524,"parent_record_offset":73500,"ancestor_record_offset":71172,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2873,"record_offset":72848,"parent_record_offset":72824,"ancestor_record_offset":71172,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":72204,"parent_record_offset":72172,"ancestor_record_offset":71172,"inlinee":4803,"source_line":478,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":77292,"parent_record_offset":77268,"ancestor_record_offset":71172,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":73500,"parent_record_offset":73464,"ancestor_record_offset":71172,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2873,"record_offset":73404,"parent_record_offset":73380,"ancestor_record_offset":71172,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2873,"record_offset":81124,"parent_record_offset":80960,"ancestor_record_offset":71172,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2873,"record_offset":73200,"parent_record_offset":73176,"ancestor_record_offset":71172,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2873,"record_offset":73380,"parent_record_offset":73356,"ancestor_record_offset":71172,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2873,"record_offset":73464,"parent_record_offset":73440,"ancestor_record_offset":71172,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":2873,"record_offset":80536,"parent_record_offset":80512,"ancestor_record_offset":71172,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2873,"record_offset":72956,"parent_record_offset":72932,"ancestor_record_offset":71172,"inlinee":10212,"source_line":30,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$1::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2873,"record_offset":80772,"parent_record_offset":80744,"ancestor_record_offset":71172,"inlinee":26844,"source_line":1301,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":72644,"parent_record_offset":72616,"ancestor_record_offset":71172,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":72676,"parent_record_offset":72644,"ancestor_record_offset":71172,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2873,"record_offset":72616,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":72476,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":73028,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2873,"record_offset":72560,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":2873,"record_offset":72420,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":2873,"record_offset":72908,"parent_record_offset":72884,"ancestor_record_offset":71172,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":77368,"parent_record_offset":77344,"ancestor_record_offset":71172,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":72256,"parent_record_offset":72232,"ancestor_record_offset":71172,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":72884,"parent_record_offset":72776,"ancestor_record_offset":71172,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":2873,"record_offset":72288,"parent_record_offset":72256,"ancestor_record_offset":71172,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72824,"parent_record_offset":72800,"ancestor_record_offset":71172,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":73120,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72800,"parent_record_offset":72776,"ancestor_record_offset":71172,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72340,"parent_record_offset":72316,"ancestor_record_offset":71172,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":77428,"parent_record_offset":77368,"ancestor_record_offset":71172,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72316,"parent_record_offset":72256,"ancestor_record_offset":71172,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72364,"parent_record_offset":72340,"ancestor_record_offset":71172,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72448,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2873,"record_offset":72776,"parent_record_offset":72364,"ancestor_record_offset":71172,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":77344,"parent_record_offset":77320,"ancestor_record_offset":71172,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":72232,"parent_record_offset":72172,"ancestor_record_offset":71172,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":2873,"record_offset":77320,"parent_record_offset":77268,"ancestor_record_offset":71172,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::decoclutter_picker::decoclutter_picker_menu::closure$5::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/decoclutter_picker.rs
//     evidence={"module":2873,"record_offset":80712,"parent_record_offset":80684,"ancestor_record_offset":71172,"inlinee":111516,"source_line":243,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":74772,"parent_record_offset":74736,"ancestor_record_offset":71172,"inlinee":10273,"source_line":1571,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::get_content_mut_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":74808,"parent_record_offset":74772,"ancestor_record_offset":71172,"inlinee":10210,"source_line":1579,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":73688,"parent_record_offset":73664,"ancestor_record_offset":71172,"inlinee":21711,"source_line":1525,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":77268,"parent_record_offset":77244,"ancestor_record_offset":71172,"inlinee":21642,"source_line":1646,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":76240,"parent_record_offset":76216,"ancestor_record_offset":71172,"inlinee":21643,"source_line":1650,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":79344,"parent_record_offset":79320,"ancestor_record_offset":71172,"inlinee":11244,"source_line":603,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":80452,"parent_record_offset":80428,"ancestor_record_offset":71172,"inlinee":21899,"source_line":597,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2873,"record_offset":80476,"parent_record_offset":80452,"ancestor_record_offset":71172,"inlinee":21900,"source_line":585,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$18::get_content_mut_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":75868,"parent_record_offset":75836,"ancestor_record_offset":71172,"inlinee":10214,"source_line":1588,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$1::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":71536,"parent_record_offset":71496,"ancestor_record_offset":71172,"inlinee":22087,"source_line":91,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":71496,"parent_record_offset":71464,"ancestor_record_offset":71172,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":72136,"parent_record_offset":71464,"ancestor_record_offset":71172,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2873,"record_offset":71464,"parent_record_offset":71432,"ancestor_record_offset":71172,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2873,"record_offset":74736,"parent_record_offset":74696,"ancestor_record_offset":71172,"inlinee":21722,"source_line":234,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":2873,"record_offset":76128,"parent_record_offset":76104,"ancestor_record_offset":71172,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B39C
// 首条位置: {"module":2873,"inlinee":111516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x215E7
// 首条位置: {"module":4207,"inlinee":136679}
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
// 完整结构及原始类型记录: decoclutter_picker.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decoclutter_picker.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decoclutter_picker.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiPointerCapture>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu
// 信息 ID: 09f1c7dff5797ed1a6740a96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 827f52a5243c415a2b9357bd: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> > @ 0x230c618 -> 0x296f300 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::DecoClutterPicker>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestDecoClutterPicker>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)> @ 0x23be68c -> 0x296f300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$1
// 信息 ID: 0dce6a3a5f863c4b95d6af3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// [闭包环境类型引用] toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$1 <- toolbar::mei::core::NodeContext::with_node_mut<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with_node_mut<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$1>; module=1235, inline_record=98960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$2
// 信息 ID: 1246ab7a9800bf3944bdfffd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// [闭包环境类型引用] toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$2,ref$<str$> >; module=1239, inline_record=120920; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$2,ref$<str$> >; module=1239, inline_record=122044; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$2,ref$<str$> >; module=1239, inline_record=123056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$3
// 信息 ID: 39e819b2618a87eea51cf6e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::decoclutter_picker::decoclutter_picker_menu; module=4207, inline_record=10740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$5
// 信息 ID: b282aee3667576034977b27a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// [闭包环境类型引用] toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$5 <- toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$5>; module=2873, inline_record=71172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$5::closure$0
// 信息 ID: 719333ef36e79c22ee44949d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::decoclutter_picker::decoclutter_picker_menu::closure_env$5>; module=2873, inline_record=80712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::decoclutter_picker::decoclutter_picker_menu::closure$6
// 信息 ID: 469d119fa2fdde1569097eee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::decoclutter_picker::decoclutter_picker_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::decoclutter_picker::decoclutter_picker_menu; module=4207, inline_record=11132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
