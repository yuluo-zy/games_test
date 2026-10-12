// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/collision/intersect_meshes.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 9ce443ac20f78343e306b9e3bcca96799df1e5ae494165a1f02cc9a9046480af
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: intersect_meshes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 500, 559, 596, 1585, 1596, 1611, 1665, 1734
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
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.7.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.0"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rapier3d; // namespace candidate
//   package identities: [{"name":"rapier3d","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050","version":"0.22.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x837A
// 首条位置: {"module":596,"inlinee":33658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::subscribe_mesh_mesh_intersections
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D38
// 首条位置: {"module":1596,"inlinee":32056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1611,"record_offset":1884528,"parent_record_offset":1884500,"ancestor_record_offset":1884500,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastWorld::mesh_intersect_collider_diff_publisher
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1611,"record_offset":1884584,"parent_record_offset":1884500,"ancestor_record_offset":1884500,"inlinee":27810,"source_line":363,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1611,"record_offset":1884552,"parent_record_offset":1884528,"ancestor_record_offset":1884500,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":37584,"rva":11107184,"va":5379816304}
// 编译实例 RVAs: 0xa97b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5143459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e85d84b1ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5379819790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4737946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e866b4b7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379819429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=22
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4737363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8adb6b7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379818862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408768f0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2239672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e848d3ddff","size":5,"soft_fail":false,"target_va":5377583344,"va":5379823011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2241072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8d0cdddff","size":5,"soft_fail":false,"target_va":5377584960,"va":5379826027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2238116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e85cd9ddff","size":5,"soft_fail":false,"target_va":5377588416,"va":5379826527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2231828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8ecf1ddff","size":5,"soft_fail":false,"target_va":5377594192,"va":5379826015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f390
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2204158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8025edeff","size":5,"soft_fail":false,"target_va":5377618832,"va":5379822985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093a440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1432349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8e324eaff","size":5,"soft_fail":false,"target_va":5378384960,"va":5379817304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093a980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1436663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e80914eaff","size":5,"soft_fail":false,"target_va":5378386304,"va":5379822962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093afc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1434869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e80b1beaff","size":5,"soft_fail":false,"target_va":5378387904,"va":5379822768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1238842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8c618edff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379817045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t804358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e806460c00","size":5,"soft_fail":false,"target_va":5380623504,"va":5379819141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t801085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e83d390c00","size":5,"soft_fail":false,"target_va":5380624688,"va":5379823598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b93940
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1029490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e872b50f00","size":5,"soft_fail":false,"target_va":5380847936,"va":5379818441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b94c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1034048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e840c70f00","size":5,"soft_fail":false,"target_va":5380852768,"va":5379818715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412929e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8366851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e803ab7f00","size":5,"soft_fail":false,"target_va":5388184032,"va":5379817176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e3d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8697363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e813b68400","size":5,"soft_fail":false,"target_va":5388516608,"va":5379819240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e4450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8696672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e860b38400","size":5,"soft_fail":false,"target_va":5388518480,"va":5379821803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f3850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8761719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e877b18500","size":5,"soft_fail":false,"target_va":5388580944,"va":5379819220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f9800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8781751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8b7ff8500","size":5,"soft_fail":false,"target_va":5388605440,"va":5379823684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412ffb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8808660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8d4688600","size":5,"soft_fail":false,"target_va":5388630800,"va":5379822135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14145c1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10235828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8b42f9c00","size":5,"soft_fail":false,"target_va":5390057888,"va":5379822055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416c1130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12749872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8308cc200","size":5,"soft_fail":false,"target_va":5392568624,"va":5379818747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31523047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8e700e101","size":5,"soft_fail":false,"target_va":5411346944,"va":5379823892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31852346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e83a07e601","size":5,"soft_fail":false,"target_va":5411669968,"va":5379817617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32030032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e850bde801","size":5,"soft_fail":false,"target_va":5411847152,"va":5379817115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32024892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e83ca9e801","size":5,"soft_fail":false,"target_va":5411847168,"va":5379822271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31716785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8b1f5e301","size":5,"soft_fail":false,"target_va":5411540992,"va":5379824202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5467095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e82994acff","size":5,"soft_fail":false,"target_va":5374355376,"va":5379822466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3026703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e80f2f2e00","size":5,"soft_fail":false,"target_va":5382848896,"va":5379822188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31717516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e88cf8e301","size":5,"soft_fail":false,"target_va":5411541584,"va":5379824063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31717639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e807f9e301","size":5,"soft_fail":false,"target_va":5411541808,"va":5379824164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31717590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8d6f8e301","size":5,"soft_fail":false,"target_va":5411541776,"va":5379824181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31658395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e89b11e301","size":5,"soft_fail":false,"target_va":5411482433,"va":5379824033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31718816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8a0fde301","size":5,"soft_fail":false,"target_va":5411542064,"va":5379823243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8e0ecffff","size":5,"soft_fail":false,"target_va":5379811584,"va":5379816475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31726810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8da1ce401","size":5,"soft_fail":false,"target_va":5411543360,"va":5379816545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31669185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8c13be301","size":5,"soft_fail":false,"target_va":5411492368,"va":5379823178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31671581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e81d45e301","size":5,"soft_fail":false,"target_va":5411494800,"va":5379823214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31667613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e89d35e301","size":5,"soft_fail":false,"target_va":5411491664,"va":5379824046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2004021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e835941e00","size":5,"soft_fail":false,"target_va":5381823600,"va":5379819574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2004956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8dc971e00","size":5,"soft_fail":false,"target_va":5381824016,"va":5379819055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1568537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e819ef1700","size":5,"soft_fail":false,"target_va":5381385536,"va":5379816994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1573726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e85e031800","size":5,"soft_fail":false,"target_va":5381390640,"va":5379816909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1567445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8d5ea1700","size":5,"soft_fail":false,"target_va":5381386480,"va":5379819030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1568263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e807ee1700","size":5,"soft_fail":false,"target_va":5381384704,"va":5379816436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rapier3d::geometry::collider::Collider::compute_aabb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     evidence={"assembly":"\tcallq\t1026927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e86fab0f00","size":5,"soft_fail":false,"target_va":5380845408,"va":5379818476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2839768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e8d8542b00","size":5,"soft_fail":false,"target_va":5382657856,"va":5379818083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2935674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e87acb2c00","size":5,"soft_fail":false,"target_va":5382753792,"va":5379818113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8998985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e849508900","size":5,"soft_fail":false,"target_va":5388825296,"va":5379826306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31713153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379816304,"raw_hex":"e881e7e301","size":5,"soft_fail":false,"target_va":5411537104,"va":5379823946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379816304,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379817135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53984,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":11307,"source_line":1214,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53776,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":11302,"source_line":2924,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":60352,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":51268,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":57684,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":53572,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":49924,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":61240,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":50216,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":559,"record_offset":40308,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":559,"record_offset":56296,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":52392,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":40608,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":49216,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":50036,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":559,"record_offset":40368,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":559,"record_offset":50372,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":24520,"source_line":117,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":559,"record_offset":50880,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":559,"record_offset":51296,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":41876,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":32025,"source_line":324,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::remove_invalid_polylines
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":50188,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":32029,"source_line":497,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::as_ivec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":56596,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":28189,"source_line":1017,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":56140,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":54856,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55968,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55364,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55156,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55880,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55068,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55548,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":559,"record_offset":56656,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4881,"source_line":2348,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":559,"record_offset":54228,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::all_edges
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":559,"record_offset":61072,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":32033,"source_line":538,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":39324,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":37900,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":38348,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":40732,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":25277,"source_line":1513,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":41148,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] tabloid::impl$3::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":559,"record_offset":40704,"parent_record_offset":37584,"ancestor_record_offset":37584,"inlinee":24756,"source_line":164,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":50496,"parent_record_offset":50472,"ancestor_record_offset":37584,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":50520,"parent_record_offset":50496,"ancestor_record_offset":37584,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":47236,"parent_record_offset":47204,"ancestor_record_offset":37584,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":50440,"parent_record_offset":50412,"ancestor_record_offset":37584,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":50472,"parent_record_offset":50440,"ancestor_record_offset":37584,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":47204,"parent_record_offset":47052,"ancestor_record_offset":37584,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":559,"record_offset":50412,"parent_record_offset":50372,"ancestor_record_offset":37584,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":54016,"parent_record_offset":53984,"ancestor_record_offset":37584,"inlinee":11308,"source_line":1270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53804,"parent_record_offset":53776,"ancestor_record_offset":37584,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":54044,"parent_record_offset":54016,"ancestor_record_offset":37584,"inlinee":11309,"source_line":1358,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53836,"parent_record_offset":53804,"ancestor_record_offset":37584,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53860,"parent_record_offset":53836,"ancestor_record_offset":37584,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":559,"record_offset":54268,"parent_record_offset":54228,"ancestor_record_offset":37584,"inlinee":11311,"source_line":92,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":559,"record_offset":53884,"parent_record_offset":53860,"ancestor_record_offset":37584,"inlinee":13999,"source_line":2832,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":57764,"parent_record_offset":57732,"ancestor_record_offset":37584,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47392,"parent_record_offset":47364,"ancestor_record_offset":37584,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":37584,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":57812,"parent_record_offset":57788,"ancestor_record_offset":37584,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":57788,"parent_record_offset":57764,"ancestor_record_offset":37584,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47340,"parent_record_offset":47316,"ancestor_record_offset":37584,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":56988,"parent_record_offset":56900,"ancestor_record_offset":37584,"inlinee":7968,"source_line":1945,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":56900,"parent_record_offset":56800,"ancestor_record_offset":37584,"inlinee":29461,"source_line":2113,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":50596,"parent_record_offset":50372,"ancestor_record_offset":37584,"inlinee":12282,"source_line":1709,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":49652,"parent_record_offset":49244,"ancestor_record_offset":37584,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":49676,"parent_record_offset":49652,"ancestor_record_offset":37584,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":57236,"parent_record_offset":56988,"ancestor_record_offset":37584,"inlinee":7971,"source_line":1977,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":50648,"parent_record_offset":50596,"ancestor_record_offset":37584,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":57204,"parent_record_offset":57172,"ancestor_record_offset":37584,"inlinee":7975,"source_line":1952,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$25::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":57024,"parent_record_offset":56988,"ancestor_record_offset":37584,"inlinee":29463,"source_line":2117,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":49788,"parent_record_offset":49764,"ancestor_record_offset":37584,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$69::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":56800,"parent_record_offset":56768,"ancestor_record_offset":37584,"inlinee":29460,"source_line":3746,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":57732,"parent_record_offset":57684,"ancestor_record_offset":37584,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":60388,"parent_record_offset":60352,"ancestor_record_offset":37584,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":56844,"parent_record_offset":56800,"ancestor_record_offset":37584,"inlinee":26151,"source_line":1176,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":44504,"parent_record_offset":44480,"ancestor_record_offset":37584,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":37584,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":59464,"parent_record_offset":59436,"ancestor_record_offset":37584,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":44480,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":58384,"parent_record_offset":58360,"ancestor_record_offset":37584,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":58752,"parent_record_offset":58728,"ancestor_record_offset":37584,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":559,"record_offset":40336,"parent_record_offset":40308,"ancestor_record_offset":37584,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\mod.rs
//     evidence={"module":559,"record_offset":57576,"parent_record_offset":57488,"ancestor_record_offset":37584,"inlinee":9414,"source_line":384,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":56932,"parent_record_offset":56900,"ancestor_record_offset":37584,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":56956,"parent_record_offset":56932,"ancestor_record_offset":37584,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":559,"record_offset":46420,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39660,"parent_record_offset":39632,"ancestor_record_offset":37584,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":45092,"parent_record_offset":45060,"ancestor_record_offset":37584,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39712,"parent_record_offset":39688,"ancestor_record_offset":37584,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39688,"parent_record_offset":39660,"ancestor_record_offset":37584,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39608,"parent_record_offset":39580,"ancestor_record_offset":37584,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39632,"parent_record_offset":39608,"ancestor_record_offset":37584,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39804,"parent_record_offset":39632,"ancestor_record_offset":37584,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":39916,"parent_record_offset":39892,"ancestor_record_offset":37584,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":40436,"parent_record_offset":40396,"ancestor_record_offset":37584,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":40992,"parent_record_offset":40964,"ancestor_record_offset":37584,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":51624,"parent_record_offset":51600,"ancestor_record_offset":37584,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":41052,"parent_record_offset":41020,"ancestor_record_offset":37584,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":50728,"parent_record_offset":50704,"ancestor_record_offset":37584,"inlinee":11098,"source_line":22,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":559,"record_offset":50704,"parent_record_offset":50596,"ancestor_record_offset":37584,"inlinee":11097,"source_line":268,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":559,"record_offset":51648,"parent_record_offset":51624,"ancestor_record_offset":37584,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":559,"record_offset":52184,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":559,"record_offset":54564,"parent_record_offset":54536,"ancestor_record_offset":37584,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":559,"record_offset":54620,"parent_record_offset":54592,"ancestor_record_offset":37584,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":559,"record_offset":56728,"parent_record_offset":56704,"ancestor_record_offset":37584,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":56680,"parent_record_offset":56656,"ancestor_record_offset":37584,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":54648,"parent_record_offset":54620,"ancestor_record_offset":37584,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":49284,"parent_record_offset":49244,"ancestor_record_offset":37584,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":559,"record_offset":56868,"parent_record_offset":56844,"ancestor_record_offset":37584,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":39736,"parent_record_offset":39712,"ancestor_record_offset":37584,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":47140,"parent_record_offset":47116,"ancestor_record_offset":37584,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":51192,"parent_record_offset":51164,"ancestor_record_offset":37584,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":47424,"parent_record_offset":47392,"ancestor_record_offset":37584,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":559,"record_offset":38048,"parent_record_offset":38016,"ancestor_record_offset":37584,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":45120,"parent_record_offset":45060,"ancestor_record_offset":37584,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":41484,"parent_record_offset":41456,"ancestor_record_offset":37584,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":41456,"parent_record_offset":41428,"ancestor_record_offset":37584,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":559,"record_offset":55096,"parent_record_offset":55068,"ancestor_record_offset":37584,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":559,"record_offset":55420,"parent_record_offset":55392,"ancestor_record_offset":37584,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":559,"record_offset":54912,"parent_record_offset":54880,"ancestor_record_offset":37584,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":39476,"parent_record_offset":39432,"ancestor_record_offset":37584,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":51100,"parent_record_offset":50880,"ancestor_record_offset":37584,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":53952,"parent_record_offset":53776,"ancestor_record_offset":37584,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":54592,"parent_record_offset":54564,"ancestor_record_offset":37584,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":46476,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":44812,"parent_record_offset":44784,"ancestor_record_offset":37584,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":38296,"parent_record_offset":38268,"ancestor_record_offset":37584,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":40264,"parent_record_offset":39432,"ancestor_record_offset":37584,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":49700,"parent_record_offset":49676,"ancestor_record_offset":37584,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":49724,"parent_record_offset":49700,"ancestor_record_offset":37584,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":39868,"parent_record_offset":39580,"ancestor_record_offset":37584,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":46612,"parent_record_offset":46588,"ancestor_record_offset":37584,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":44900,"parent_record_offset":44784,"ancestor_record_offset":37584,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":41580,"parent_record_offset":41556,"ancestor_record_offset":37584,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":57264,"parent_record_offset":57236,"ancestor_record_offset":37584,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":50148,"parent_record_offset":50124,"ancestor_record_offset":37584,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":50672,"parent_record_offset":50648,"ancestor_record_offset":37584,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":38760,"parent_record_offset":38736,"ancestor_record_offset":37584,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":50312,"parent_record_offset":50216,"ancestor_record_offset":37584,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":38736,"parent_record_offset":38712,"ancestor_record_offset":37584,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":38944,"parent_record_offset":38916,"ancestor_record_offset":37584,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44976,"parent_record_offset":44948,"ancestor_record_offset":37584,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":50548,"parent_record_offset":50520,"ancestor_record_offset":37584,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44924,"parent_record_offset":44900,"ancestor_record_offset":37584,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":39760,"parent_record_offset":39736,"ancestor_record_offset":37584,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":559,"record_offset":57172,"parent_record_offset":56988,"ancestor_record_offset":37584,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":40520,"parent_record_offset":40492,"ancestor_record_offset":37584,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":40548,"parent_record_offset":40520,"ancestor_record_offset":37584,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":46564,"parent_record_offset":46540,"ancestor_record_offset":37584,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":49948,"parent_record_offset":49924,"ancestor_record_offset":37584,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":51072,"parent_record_offset":50880,"ancestor_record_offset":37584,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":46588,"parent_record_offset":46564,"ancestor_record_offset":37584,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":38688,"parent_record_offset":38664,"ancestor_record_offset":37584,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":54468,"parent_record_offset":54444,"ancestor_record_offset":37584,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":38712,"parent_record_offset":38688,"ancestor_record_offset":37584,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":49972,"parent_record_offset":49948,"ancestor_record_offset":37584,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":50124,"parent_record_offset":50100,"ancestor_record_offset":37584,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":39016,"parent_record_offset":38988,"ancestor_record_offset":37584,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":46652,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":38664,"parent_record_offset":38548,"ancestor_record_offset":37584,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":54444,"parent_record_offset":54420,"ancestor_record_offset":37584,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":50100,"parent_record_offset":50036,"ancestor_record_offset":37584,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":46540,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":38016,"parent_record_offset":37988,"ancestor_record_offset":37584,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":38376,"parent_record_offset":38348,"ancestor_record_offset":37584,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":38516,"parent_record_offset":38484,"ancestor_record_offset":37584,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":38484,"parent_record_offset":38448,"ancestor_record_offset":37584,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":38424,"parent_record_offset":38400,"ancestor_record_offset":37584,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":38448,"parent_record_offset":38424,"ancestor_record_offset":37584,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":38400,"parent_record_offset":38376,"ancestor_record_offset":37584,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":38988,"parent_record_offset":38448,"ancestor_record_offset":37584,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":37988,"parent_record_offset":37956,"ancestor_record_offset":37584,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":37956,"parent_record_offset":37932,"ancestor_record_offset":37584,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":38148,"parent_record_offset":38116,"ancestor_record_offset":37584,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":38244,"parent_record_offset":38148,"ancestor_record_offset":37584,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":38116,"parent_record_offset":38092,"ancestor_record_offset":37584,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":40464,"parent_record_offset":40436,"ancestor_record_offset":37584,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":37932,"parent_record_offset":37900,"ancestor_record_offset":37584,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":38268,"parent_record_offset":38244,"ancestor_record_offset":37584,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":38548,"parent_record_offset":38516,"ancestor_record_offset":37584,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":38832,"parent_record_offset":38548,"ancestor_record_offset":37584,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":38804,"parent_record_offset":38548,"ancestor_record_offset":37584,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":38608,"parent_record_offset":38548,"ancestor_record_offset":37584,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":59180,"parent_record_offset":59116,"ancestor_record_offset":37584,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":50984,"parent_record_offset":50948,"ancestor_record_offset":37584,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":50816,"parent_record_offset":50596,"ancestor_record_offset":37584,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":49492,"parent_record_offset":49368,"ancestor_record_offset":37584,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":50760,"parent_record_offset":50596,"ancestor_record_offset":37584,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":59204,"parent_record_offset":59180,"ancestor_record_offset":37584,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":50840,"parent_record_offset":50816,"ancestor_record_offset":37584,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":50784,"parent_record_offset":50760,"ancestor_record_offset":37584,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":49516,"parent_record_offset":49492,"ancestor_record_offset":37584,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":49880,"parent_record_offset":49788,"ancestor_record_offset":37584,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":559,"record_offset":40396,"parent_record_offset":40368,"ancestor_record_offset":37584,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::instanced_wall::BrickSourceId::from_wall_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":559,"record_offset":56344,"parent_record_offset":56320,"ancestor_record_offset":37584,"inlinee":26442,"source_line":60,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::instanced_wall::impl$1::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":559,"record_offset":56320,"parent_record_offset":56296,"ancestor_record_offset":37584,"inlinee":26504,"source_line":97,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":559,"record_offset":51164,"parent_record_offset":51136,"ancestor_record_offset":37584,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":559,"record_offset":59088,"parent_record_offset":59060,"ancestor_record_offset":37584,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":559,"record_offset":52140,"parent_record_offset":52116,"ancestor_record_offset":37584,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":559,"record_offset":52440,"parent_record_offset":52416,"ancestor_record_offset":37584,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":52416,"parent_record_offset":52392,"ancestor_record_offset":37584,"inlinee":32031,"source_line":98,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":59272,"parent_record_offset":59248,"ancestor_record_offset":37584,"inlinee":32032,"source_line":102,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::polyline::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":44536,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":32027,"source_line":521,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$1::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":46444,"parent_record_offset":46420,"ancestor_record_offset":37584,"inlinee":27632,"source_line":174,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$96::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":46512,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":25281,"source_line":1433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":54996,"parent_record_offset":54880,"ancestor_record_offset":37584,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55244,"parent_record_offset":55216,"ancestor_record_offset":37584,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55216,"parent_record_offset":55184,"ancestor_record_offset":37584,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55184,"parent_record_offset":55156,"ancestor_record_offset":37584,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55392,"parent_record_offset":55364,"ancestor_record_offset":37584,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":55488,"parent_record_offset":55364,"ancestor_record_offset":37584,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":54880,"parent_record_offset":54856,"ancestor_record_offset":37584,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":559,"record_offset":56624,"parent_record_offset":56596,"ancestor_record_offset":37584,"inlinee":28191,"source_line":69,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":41388,"parent_record_offset":41348,"ancestor_record_offset":37584,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":41428,"parent_record_offset":41388,"ancestor_record_offset":37584,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":41524,"parent_record_offset":41348,"ancestor_record_offset":37584,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":41348,"parent_record_offset":41296,"ancestor_record_offset":37584,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":51600,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":40964,"parent_record_offset":40936,"ancestor_record_offset":37584,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":52212,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":41020,"parent_record_offset":40964,"ancestor_record_offset":37584,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":41668,"parent_record_offset":41296,"ancestor_record_offset":37584,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":51684,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":559,"record_offset":51572,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":559,"record_offset":52092,"parent_record_offset":52068,"ancestor_record_offset":37584,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":51356,"parent_record_offset":51332,"ancestor_record_offset":37584,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":51380,"parent_record_offset":51356,"ancestor_record_offset":37584,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":40828,"parent_record_offset":40804,"ancestor_record_offset":37584,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":40804,"parent_record_offset":40780,"ancestor_record_offset":37584,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":52068,"parent_record_offset":51900,"ancestor_record_offset":37584,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":41232,"parent_record_offset":41204,"ancestor_record_offset":37584,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":41204,"parent_record_offset":41176,"ancestor_record_offset":37584,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":51412,"parent_record_offset":51380,"ancestor_record_offset":37584,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":52008,"parent_record_offset":51924,"ancestor_record_offset":37584,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51952,"parent_record_offset":51924,"ancestor_record_offset":37584,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":41556,"parent_record_offset":41296,"ancestor_record_offset":37584,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":52304,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":40936,"parent_record_offset":40876,"ancestor_record_offset":37584,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":41296,"parent_record_offset":41260,"ancestor_record_offset":37584,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51924,"parent_record_offset":51900,"ancestor_record_offset":37584,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51464,"parent_record_offset":51440,"ancestor_record_offset":37584,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51440,"parent_record_offset":51380,"ancestor_record_offset":37584,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":37584,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47084,"parent_record_offset":47052,"ancestor_record_offset":37584,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":40908,"parent_record_offset":40876,"ancestor_record_offset":37584,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":46996,"parent_record_offset":46972,"ancestor_record_offset":37584,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51488,"parent_record_offset":51464,"ancestor_record_offset":37584,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47052,"parent_record_offset":46996,"ancestor_record_offset":37584,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":559,"record_offset":47024,"parent_record_offset":46996,"ancestor_record_offset":37584,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":40876,"parent_record_offset":40852,"ancestor_record_offset":37584,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51544,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47116,"parent_record_offset":47084,"ancestor_record_offset":37584,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":46972,"parent_record_offset":46948,"ancestor_record_offset":37584,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":41260,"parent_record_offset":41232,"ancestor_record_offset":37584,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":51900,"parent_record_offset":51488,"ancestor_record_offset":37584,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":559,"record_offset":40780,"parent_record_offset":40756,"ancestor_record_offset":37584,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":559,"record_offset":41176,"parent_record_offset":41148,"ancestor_record_offset":37584,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":559,"record_offset":61096,"parent_record_offset":61072,"ancestor_record_offset":37584,"inlinee":7627,"source_line":240,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::iter::Iter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map/iter.rs
//     evidence={"module":559,"record_offset":61120,"parent_record_offset":61096,"ancestor_record_offset":37584,"inlinee":7629,"source_line":47,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":559,"record_offset":54536,"parent_record_offset":54228,"ancestor_record_offset":37584,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":559,"record_offset":49340,"parent_record_offset":49312,"ancestor_record_offset":37584,"inlinee":5808,"source_line":2431,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":50948,"parent_record_offset":50880,"ancestor_record_offset":37584,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":559,"record_offset":59116,"parent_record_offset":59088,"ancestor_record_offset":37584,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":559,"record_offset":49404,"parent_record_offset":49368,"ancestor_record_offset":37584,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":559,"record_offset":49368,"parent_record_offset":49340,"ancestor_record_offset":37584,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":559,"record_offset":51040,"parent_record_offset":50948,"ancestor_record_offset":37584,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":559,"record_offset":49432,"parent_record_offset":49404,"ancestor_record_offset":37584,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/partitioning/qbvh/traversal.rs
//     evidence={"module":559,"record_offset":46844,"parent_record_offset":46784,"ancestor_record_offset":37584,"inlinee":13819,"source_line":31,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_visitor.rs
//     evidence={"module":559,"record_offset":46812,"parent_record_offset":46784,"ancestor_record_offset":37584,"inlinee":13818,"source_line":20,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":43472,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":39360,"parent_record_offset":39324,"ancestor_record_offset":37584,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":42028,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":47848,"parent_record_offset":47808,"ancestor_record_offset":37584,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":48416,"parent_record_offset":48144,"ancestor_record_offset":37584,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":39504,"parent_record_offset":39476,"ancestor_record_offset":37584,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":39580,"parent_record_offset":39432,"ancestor_record_offset":37584,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":39840,"parent_record_offset":39580,"ancestor_record_offset":37584,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":42496,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::pipeline::query_pipeline::QueryPipeline::colliders_with_aabb_intersecting_aabb
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/pipeline/query_pipeline/mod.rs
//     evidence={"module":559,"record_offset":46784,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":32028,"source_line":581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":51332,"parent_record_offset":51296,"ancestor_record_offset":37584,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":44564,"parent_record_offset":44536,"ancestor_record_offset":37584,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":40756,"parent_record_offset":40732,"ancestor_record_offset":37584,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":45296,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":25277,"source_line":1513,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":45712,"parent_record_offset":41876,"ancestor_record_offset":37584,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":559,"record_offset":55028,"parent_record_offset":54996,"ancestor_record_offset":37584,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44612,"parent_record_offset":44588,"ancestor_record_offset":37584,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":37584,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44724,"parent_record_offset":44696,"ancestor_record_offset":37584,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":45060,"parent_record_offset":44660,"ancestor_record_offset":37584,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44588,"parent_record_offset":44564,"ancestor_record_offset":37584,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":39532,"parent_record_offset":39504,"ancestor_record_offset":37584,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":44784,"parent_record_offset":44752,"ancestor_record_offset":37584,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":39432,"parent_record_offset":39396,"ancestor_record_offset":37584,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":39396,"parent_record_offset":39360,"ancestor_record_offset":37584,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":559,"record_offset":49244,"parent_record_offset":49216,"ancestor_record_offset":37584,"inlinee":24758,"source_line":147,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":559,"record_offset":52360,"parent_record_offset":51296,"ancestor_record_offset":37584,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D1F
// 首条位置: {"module":559,"inlinee":32031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":559,"record_offset":52440,"parent_record_offset":52416,"ancestor_record_offset":52416,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D20
// 首条位置: {"module":559,"inlinee":32032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":559,"record_offset":59296,"parent_record_offset":59272,"ancestor_record_offset":59272,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A95
// 首条位置: {"module":1585,"inlinee":72341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1585,"record_offset":147508,"parent_record_offset":147484,"ancestor_record_offset":147484,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":147556,"parent_record_offset":147532,"ancestor_record_offset":147484,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1585,"record_offset":147532,"parent_record_offset":147508,"ancestor_record_offset":147484,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":66780,"rva":11120544,"va":5379829664}
// 编译实例 RVAs: 0xa9afa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4749028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e81c89b7ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379830527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416bffe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12734387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e8b34fc200","size":5,"soft_fail":false,"target_va":5392564192,"va":5379829800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t805275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e89b490c00","size":5,"soft_fail":false,"target_va":5380635696,"va":5379830416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::shape::trimesh::TriMesh::triangle
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13732773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e8a58bd100","size":5,"soft_fail":false,"target_va":5393562672,"va":5379829894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::expand
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8016515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e883527a00","size":5,"soft_fail":false,"target_va":5387846592,"va":5379830072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::inclusive_contains_point
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8016711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e847537a00","size":5,"soft_fail":false,"target_va":5387846896,"va":5379830180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::aabb
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7994680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e838fd7900","size":5,"soft_fail":false,"target_va":5387824720,"va":5379830035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::intersect_triangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7994948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e844fe7900","size":5,"soft_fail":false,"target_va":5387824960,"va":5379830007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7994212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379829664,"raw_hex":"e864fb7900","size":5,"soft_fail":false,"target_va":5387824544,"va":5379830327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":66968,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67564,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67024,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":67756,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":67284,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::from_parry_triangle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":67372,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":32036,"source_line":308,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":67444,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":559,"record_offset":67500,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_simultaneous_visitor.rs
//     evidence={"module":559,"record_offset":66996,"parent_record_offset":66780,"ancestor_record_offset":66780,"inlinee":32035,"source_line":21,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":67944,"parent_record_offset":67920,"ancestor_record_offset":66780,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":67920,"parent_record_offset":67828,"ancestor_record_offset":66780,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67128,"parent_record_offset":67104,"ancestor_record_offset":66780,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67856,"parent_record_offset":67828,"ancestor_record_offset":66780,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67828,"parent_record_offset":67804,"ancestor_record_offset":66780,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67176,"parent_record_offset":67152,"ancestor_record_offset":66780,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67152,"parent_record_offset":67128,"ancestor_record_offset":66780,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":67804,"parent_record_offset":67780,"ancestor_record_offset":66780,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67604,"parent_record_offset":67564,"ancestor_record_offset":66780,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67104,"parent_record_offset":67080,"ancestor_record_offset":66780,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67080,"parent_record_offset":67048,"ancestor_record_offset":66780,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":67048,"parent_record_offset":67024,"ancestor_record_offset":66780,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":67888,"parent_record_offset":67856,"ancestor_record_offset":66780,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":559,"record_offset":67532,"parent_record_offset":67500,"ancestor_record_offset":66780,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":67780,"parent_record_offset":67756,"ancestor_record_offset":66780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":67244,"parent_record_offset":67220,"ancestor_record_offset":66780,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":67332,"parent_record_offset":67284,"ancestor_record_offset":66780,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":67724,"parent_record_offset":67564,"ancestor_record_offset":66780,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":67220,"parent_record_offset":67048,"ancestor_record_offset":66780,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12837
// 首条位置: {"module":1665,"inlinee":75831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21516,"parent_record_offset":21488,"ancestor_record_offset":21488,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1665,"record_offset":21580,"parent_record_offset":21556,"ancestor_record_offset":21488,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1665,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":21488,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1665,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":21488,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":21556,"parent_record_offset":21516,"ancestor_record_offset":21488,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":21668,"parent_record_offset":21516,"ancestor_record_offset":21488,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01665.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::from_parry_triangle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D24
// 首条位置: {"module":559,"inlinee":32036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D19
// 首条位置: {"module":559,"inlinee":32025}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":45268,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":44480,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$15::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":559,"record_offset":46420,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":9465,"source_line":363,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":46476,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":46876,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":46652,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":46540,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":559,"record_offset":44536,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":32027,"source_line":521,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$96::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":46512,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":25281,"source_line":1433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":43472,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":42028,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":42496,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::pipeline::query_pipeline::QueryPipeline::colliders_with_aabb_intersecting_aabb
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/pipeline/query_pipeline/mod.rs
//     evidence={"module":559,"record_offset":46784,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":32028,"source_line":581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":45296,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":25277,"source_line":1513,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":45712,"parent_record_offset":41876,"ancestor_record_offset":41876,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":47236,"parent_record_offset":47204,"ancestor_record_offset":41876,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":47204,"parent_record_offset":47052,"ancestor_record_offset":41876,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47392,"parent_record_offset":47364,"ancestor_record_offset":41876,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":41876,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":47340,"parent_record_offset":47316,"ancestor_record_offset":41876,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":44504,"parent_record_offset":44480,"ancestor_record_offset":41876,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":43860,"parent_record_offset":43832,"ancestor_record_offset":41876,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":45092,"parent_record_offset":45060,"ancestor_record_offset":41876,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":43912,"parent_record_offset":43888,"ancestor_record_offset":41876,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":43888,"parent_record_offset":43860,"ancestor_record_offset":41876,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":43800,"parent_record_offset":43768,"ancestor_record_offset":41876,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":43832,"parent_record_offset":43800,"ancestor_record_offset":41876,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":44004,"parent_record_offset":43832,"ancestor_record_offset":41876,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":44116,"parent_record_offset":44092,"ancestor_record_offset":41876,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":45556,"parent_record_offset":45528,"ancestor_record_offset":41876,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":45616,"parent_record_offset":45584,"ancestor_record_offset":41876,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":43936,"parent_record_offset":43912,"ancestor_record_offset":41876,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":47140,"parent_record_offset":47116,"ancestor_record_offset":41876,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":47424,"parent_record_offset":47392,"ancestor_record_offset":41876,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":559,"record_offset":42188,"parent_record_offset":42156,"ancestor_record_offset":41876,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":45120,"parent_record_offset":45060,"ancestor_record_offset":41876,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":46028,"parent_record_offset":46000,"ancestor_record_offset":41876,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":46000,"parent_record_offset":45972,"ancestor_record_offset":41876,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":43636,"parent_record_offset":43596,"ancestor_record_offset":41876,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":44812,"parent_record_offset":44784,"ancestor_record_offset":41876,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":42444,"parent_record_offset":42416,"ancestor_record_offset":41876,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":43736,"parent_record_offset":43596,"ancestor_record_offset":41876,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44068,"parent_record_offset":43768,"ancestor_record_offset":41876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":46612,"parent_record_offset":46588,"ancestor_record_offset":41876,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":44900,"parent_record_offset":44784,"ancestor_record_offset":41876,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":46280,"parent_record_offset":46256,"ancestor_record_offset":41876,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":41876,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":42884,"parent_record_offset":42860,"ancestor_record_offset":41876,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":43092,"parent_record_offset":43064,"ancestor_record_offset":41876,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44976,"parent_record_offset":44948,"ancestor_record_offset":41876,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44924,"parent_record_offset":44900,"ancestor_record_offset":41876,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":43960,"parent_record_offset":43936,"ancestor_record_offset":41876,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":46564,"parent_record_offset":46540,"ancestor_record_offset":41876,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":559,"record_offset":46588,"parent_record_offset":46564,"ancestor_record_offset":41876,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":42836,"parent_record_offset":42812,"ancestor_record_offset":41876,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":42860,"parent_record_offset":42836,"ancestor_record_offset":41876,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":43164,"parent_record_offset":43136,"ancestor_record_offset":41876,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":42812,"parent_record_offset":42696,"ancestor_record_offset":41876,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":42156,"parent_record_offset":42128,"ancestor_record_offset":41876,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":559,"record_offset":42524,"parent_record_offset":42496,"ancestor_record_offset":41876,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":42664,"parent_record_offset":42632,"ancestor_record_offset":41876,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":559,"record_offset":42632,"parent_record_offset":42596,"ancestor_record_offset":41876,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":42572,"parent_record_offset":42548,"ancestor_record_offset":41876,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":42596,"parent_record_offset":42572,"ancestor_record_offset":41876,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":42548,"parent_record_offset":42524,"ancestor_record_offset":41876,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":559,"record_offset":43136,"parent_record_offset":42596,"ancestor_record_offset":41876,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42128,"parent_record_offset":42096,"ancestor_record_offset":41876,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42096,"parent_record_offset":42068,"ancestor_record_offset":41876,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42296,"parent_record_offset":42260,"ancestor_record_offset":41876,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42392,"parent_record_offset":42296,"ancestor_record_offset":41876,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42260,"parent_record_offset":42232,"ancestor_record_offset":41876,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42068,"parent_record_offset":42028,"ancestor_record_offset":41876,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":559,"record_offset":42416,"parent_record_offset":42392,"ancestor_record_offset":41876,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":42696,"parent_record_offset":42664,"ancestor_record_offset":41876,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":42980,"parent_record_offset":42696,"ancestor_record_offset":41876,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":42952,"parent_record_offset":42696,"ancestor_record_offset":41876,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":559,"record_offset":42756,"parent_record_offset":42696,"ancestor_record_offset":41876,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$1::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":46444,"parent_record_offset":46420,"ancestor_record_offset":41876,"inlinee":27632,"source_line":174,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":45936,"parent_record_offset":45896,"ancestor_record_offset":41876,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":45972,"parent_record_offset":45936,"ancestor_record_offset":41876,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":46068,"parent_record_offset":45896,"ancestor_record_offset":41876,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":559,"record_offset":45896,"parent_record_offset":45844,"ancestor_record_offset":41876,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":45528,"parent_record_offset":45500,"ancestor_record_offset":41876,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":45584,"parent_record_offset":45528,"ancestor_record_offset":41876,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":46156,"parent_record_offset":45844,"ancestor_record_offset":41876,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":41876,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":45368,"parent_record_offset":45344,"ancestor_record_offset":41876,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":45784,"parent_record_offset":45760,"ancestor_record_offset":41876,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":559,"record_offset":45760,"parent_record_offset":45736,"ancestor_record_offset":41876,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":46256,"parent_record_offset":45844,"ancestor_record_offset":41876,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45500,"parent_record_offset":45440,"ancestor_record_offset":41876,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45844,"parent_record_offset":45808,"ancestor_record_offset":41876,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45416,"parent_record_offset":45392,"ancestor_record_offset":41876,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47084,"parent_record_offset":47052,"ancestor_record_offset":41876,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45472,"parent_record_offset":45440,"ancestor_record_offset":41876,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":46996,"parent_record_offset":46972,"ancestor_record_offset":41876,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47052,"parent_record_offset":46996,"ancestor_record_offset":41876,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":559,"record_offset":47024,"parent_record_offset":46996,"ancestor_record_offset":41876,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45440,"parent_record_offset":45416,"ancestor_record_offset":41876,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":47116,"parent_record_offset":47084,"ancestor_record_offset":41876,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":46972,"parent_record_offset":46948,"ancestor_record_offset":41876,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":559,"record_offset":45808,"parent_record_offset":45784,"ancestor_record_offset":41876,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":559,"record_offset":45344,"parent_record_offset":45320,"ancestor_record_offset":41876,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":559,"record_offset":45736,"parent_record_offset":45712,"ancestor_record_offset":41876,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/partitioning/qbvh/traversal.rs
//     evidence={"module":559,"record_offset":46844,"parent_record_offset":46784,"ancestor_record_offset":41876,"inlinee":13819,"source_line":31,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/query/visitors/bounding_volume_intersections_visitor.rs
//     evidence={"module":559,"record_offset":46812,"parent_record_offset":46784,"ancestor_record_offset":41876,"inlinee":13818,"source_line":20,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":43512,"parent_record_offset":43472,"ancestor_record_offset":41876,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":47848,"parent_record_offset":47808,"ancestor_record_offset":41876,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":48416,"parent_record_offset":48144,"ancestor_record_offset":41876,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":43664,"parent_record_offset":43636,"ancestor_record_offset":41876,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":43768,"parent_record_offset":43596,"ancestor_record_offset":41876,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":559,"record_offset":44040,"parent_record_offset":43768,"ancestor_record_offset":41876,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":44564,"parent_record_offset":44536,"ancestor_record_offset":41876,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":45320,"parent_record_offset":45296,"ancestor_record_offset":41876,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44612,"parent_record_offset":44588,"ancestor_record_offset":41876,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":41876,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44724,"parent_record_offset":44696,"ancestor_record_offset":41876,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":45060,"parent_record_offset":44660,"ancestor_record_offset":41876,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44588,"parent_record_offset":44564,"ancestor_record_offset":41876,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":43692,"parent_record_offset":43664,"ancestor_record_offset":41876,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":44784,"parent_record_offset":44752,"ancestor_record_offset":41876,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":43596,"parent_record_offset":43552,"ancestor_record_offset":41876,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":43552,"parent_record_offset":43512,"ancestor_record_offset":41876,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1306E
// 首条位置: {"module":1734,"inlinee":77934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1734,"record_offset":121364,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1734,"record_offset":121420,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1734,"record_offset":121392,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1734,"record_offset":121996,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1734,"record_offset":119780,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1734,"record_offset":121160,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1734,"record_offset":122192,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":121968,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":27839,"source_line":1569,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":121656,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":10926,"source_line":261,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::SubworldColliderHandle::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119644,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":27803,"source_line":210,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119672,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":25293,"source_line":185,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider::Collider::shape
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider.rs
//     evidence={"module":1734,"record_offset":121196,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":35133,"source_line":431,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1734,"record_offset":121448,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":10929,"source_line":863,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1734,"record_offset":120004,"parent_record_offset":119492,"ancestor_record_offset":119492,"inlinee":4470,"source_line":689,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1734,"record_offset":122392,"parent_record_offset":122368,"ancestor_record_offset":119492,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1734,"record_offset":122368,"parent_record_offset":122272,"ancestor_record_offset":119492,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122072,"parent_record_offset":122040,"ancestor_record_offset":119492,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122304,"parent_record_offset":122272,"ancestor_record_offset":119492,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122272,"parent_record_offset":122244,"ancestor_record_offset":119492,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122120,"parent_record_offset":122096,"ancestor_record_offset":119492,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122096,"parent_record_offset":122072,"ancestor_record_offset":119492,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1734,"record_offset":122244,"parent_record_offset":122216,"ancestor_record_offset":119492,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1734,"record_offset":121244,"parent_record_offset":121220,"ancestor_record_offset":119492,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1734,"record_offset":122040,"parent_record_offset":121996,"ancestor_record_offset":119492,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1734,"record_offset":119904,"parent_record_offset":119884,"ancestor_record_offset":119492,"inlinee":26774,"source_line":1836,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1734,"record_offset":119960,"parent_record_offset":119936,"ancestor_record_offset":119492,"inlinee":10268,"source_line":1836,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1734,"record_offset":120820,"parent_record_offset":120792,"ancestor_record_offset":119492,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1734,"record_offset":120320,"parent_record_offset":120296,"ancestor_record_offset":119492,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1734,"record_offset":120404,"parent_record_offset":120380,"ancestor_record_offset":119492,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1734,"record_offset":120344,"parent_record_offset":120320,"ancestor_record_offset":119492,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1734,"record_offset":120980,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1734,"record_offset":122336,"parent_record_offset":122304,"ancestor_record_offset":119492,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1734,"record_offset":120552,"parent_record_offset":120524,"ancestor_record_offset":119492,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1734,"record_offset":120524,"parent_record_offset":120496,"ancestor_record_offset":119492,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1734,"record_offset":121584,"parent_record_offset":121484,"ancestor_record_offset":119492,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1734,"record_offset":122216,"parent_record_offset":122192,"ancestor_record_offset":119492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1734,"record_offset":120672,"parent_record_offset":120648,"ancestor_record_offset":119492,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1734,"record_offset":122160,"parent_record_offset":121996,"ancestor_record_offset":119492,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1734,"record_offset":121520,"parent_record_offset":121484,"ancestor_record_offset":119492,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1734,"record_offset":121548,"parent_record_offset":121520,"ancestor_record_offset":119492,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$16::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":1734,"record_offset":119936,"parent_record_offset":119856,"ancestor_record_offset":119492,"inlinee":26776,"source_line":108,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$3::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119828,"parent_record_offset":119780,"ancestor_record_offset":119492,"inlinee":26768,"source_line":192,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$4::cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119856,"parent_record_offset":119828,"ancestor_record_offset":119492,"inlinee":26770,"source_line":198,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":120848,"parent_record_offset":120820,"ancestor_record_offset":119492,"inlinee":25293,"source_line":185,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$96::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119696,"parent_record_offset":119672,"ancestor_record_offset":119492,"inlinee":25281,"source_line":1433,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$98::cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1734,"record_offset":119884,"parent_record_offset":119856,"ancestor_record_offset":119492,"inlinee":26772,"source_line":1433,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1734,"record_offset":120464,"parent_record_offset":120436,"ancestor_record_offset":119492,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1734,"record_offset":120496,"parent_record_offset":120464,"ancestor_record_offset":119492,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1734,"record_offset":120436,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1734,"record_offset":120296,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1734,"record_offset":121008,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1734,"record_offset":120380,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1734,"record_offset":120268,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1734,"record_offset":120792,"parent_record_offset":120764,"ancestor_record_offset":119492,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1734,"record_offset":120052,"parent_record_offset":120028,"ancestor_record_offset":119492,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1734,"record_offset":120076,"parent_record_offset":120052,"ancestor_record_offset":119492,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1734,"record_offset":120764,"parent_record_offset":120596,"ancestor_record_offset":119492,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1734,"record_offset":120108,"parent_record_offset":120076,"ancestor_record_offset":119492,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120704,"parent_record_offset":120620,"ancestor_record_offset":119492,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120648,"parent_record_offset":120620,"ancestor_record_offset":119492,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":121100,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120620,"parent_record_offset":120596,"ancestor_record_offset":119492,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120160,"parent_record_offset":120136,"ancestor_record_offset":119492,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120136,"parent_record_offset":120076,"ancestor_record_offset":119492,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120184,"parent_record_offset":120160,"ancestor_record_offset":119492,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120240,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1734,"record_offset":120596,"parent_record_offset":120184,"ancestor_record_offset":119492,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1734,"record_offset":120028,"parent_record_offset":120004,"ancestor_record_offset":119492,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::shape::shared_shape::impl$1::as_ref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1734,"record_offset":121220,"parent_record_offset":121196,"ancestor_record_offset":119492,"inlinee":35135,"source_line":32,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::impl$33::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1734,"record_offset":119744,"parent_record_offset":119720,"ancestor_record_offset":119492,"inlinee":25297,"source_line":48,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::geometry::collider_components::impl$27::eq
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rapier3d","version":"0.22.0","source":"git+https://github.com/PounceLight/rapier.git?rev=7b5ece73c45a4d39bb983c48c283e773d7241050#7b5ece73c45a4d39bb983c48c283e773d7241050"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-40bda2dbc2e76d6e/7b5ece7/crates/rapier3d/../../src/geometry/collider_components.rs
//     evidence={"module":1734,"record_offset":119720,"parent_record_offset":119672,"ancestor_record_offset":119492,"inlinee":25295,"source_line":9,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::KeyData::from_ffi
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":1734,"record_offset":121680,"parent_record_offset":121656,"ancestor_record_offset":119492,"inlinee":10927,"source_line":304,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1734,"record_offset":121484,"parent_record_offset":121448,"ancestor_record_offset":119492,"inlinee":10931,"source_line":538,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1734,"record_offset":121616,"parent_record_offset":121584,"ancestor_record_offset":119492,"inlinee":10933,"source_line":542,"archive":"_pdb/B/modules/01734.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::should_intersect_collider_meshes
// 记录形式: compiled_procedure; instances=1
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":68232,"rva":11121600,"va":5379830720}
// 编译实例 RVAs: 0xa9b3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31711163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379830720,"raw_hex":"e8bbdfe301","size":5,"soft_fail":false,"target_va":5411542064,"va":5379830896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::begin_panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31707746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379830720,"raw_hex":"e862d2e301","size":5,"soft_fail":false,"target_va":5411538725,"va":5379830974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":559,"record_offset":68940,"parent_record_offset":68232,"ancestor_record_offset":68232,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":68912,"parent_record_offset":68232,"ancestor_record_offset":68232,"inlinee":27839,"source_line":1569,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":559,"record_offset":68604,"parent_record_offset":68232,"ancestor_record_offset":68232,"inlinee":10926,"source_line":261,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":559,"record_offset":68412,"parent_record_offset":68232,"ancestor_record_offset":68232,"inlinee":10929,"source_line":863,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":68536,"parent_record_offset":68444,"ancestor_record_offset":68232,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":559,"record_offset":68476,"parent_record_offset":68444,"ancestor_record_offset":68232,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":559,"record_offset":68504,"parent_record_offset":68476,"ancestor_record_offset":68232,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::KeyData::from_ffi
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":559,"record_offset":68628,"parent_record_offset":68604,"ancestor_record_offset":68232,"inlinee":10927,"source_line":304,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":559,"record_offset":68444,"parent_record_offset":68412,"ancestor_record_offset":68232,"inlinee":10931,"source_line":538,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":559,"record_offset":68568,"parent_record_offset":68536,"ancestor_record_offset":68232,"inlinee":10933,"source_line":542,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::remove_invalid_polylines
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D1D
// 首条位置: {"module":559,"inlinee":32029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 498
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A66
// 首条位置: {"module":1585,"inlinee":72294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72408,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":4906,"source_line":1342,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72436,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":11302,"source_line":2924,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72764,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":29372,"source_line":2972,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter_mut::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":1585,"record_offset":73352,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":24646,"source_line":157,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73132,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73044,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73504,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72828,"parent_record_offset":72796,"ancestor_record_offset":72344,"inlinee":14988,"source_line":1304,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72544,"parent_record_offset":72516,"ancestor_record_offset":72344,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72460,"parent_record_offset":72436,"ancestor_record_offset":72344,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72796,"parent_record_offset":72764,"ancestor_record_offset":72344,"inlinee":24644,"source_line":1237,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72644,"parent_record_offset":72460,"ancestor_record_offset":72344,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72856,"parent_record_offset":72828,"ancestor_record_offset":72344,"inlinee":11309,"source_line":1358,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72488,"parent_record_offset":72460,"ancestor_record_offset":72344,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1585,"record_offset":72516,"parent_record_offset":72488,"ancestor_record_offset":72344,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":72576,"parent_record_offset":72544,"ancestor_record_offset":72344,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":72668,"parent_record_offset":72644,"ancestor_record_offset":72344,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":72600,"parent_record_offset":72576,"ancestor_record_offset":72344,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":72716,"parent_record_offset":72692,"ancestor_record_offset":72344,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":72692,"parent_record_offset":72668,"ancestor_record_offset":72344,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":73072,"parent_record_offset":73044,"ancestor_record_offset":72344,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":73532,"parent_record_offset":73504,"ancestor_record_offset":72344,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":72944,"parent_record_offset":72920,"ancestor_record_offset":72344,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1585,"record_offset":73408,"parent_record_offset":73380,"ancestor_record_offset":72344,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1585,"record_offset":72896,"parent_record_offset":72796,"ancestor_record_offset":72344,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1585,"record_offset":72920,"parent_record_offset":72896,"ancestor_record_offset":72344,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1585,"record_offset":73380,"parent_record_offset":73352,"ancestor_record_offset":72344,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73268,"parent_record_offset":73232,"ancestor_record_offset":72344,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73232,"parent_record_offset":73204,"ancestor_record_offset":72344,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73204,"parent_record_offset":73164,"ancestor_record_offset":72344,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":73164,"parent_record_offset":73132,"ancestor_record_offset":72344,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1585,"record_offset":73304,"parent_record_offset":73268,"ancestor_record_offset":72344,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 521
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D1B
// 首条位置: {"module":559,"inlinee":32027}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":44564,"parent_record_offset":44536,"ancestor_record_offset":44536,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":45092,"parent_record_offset":45060,"ancestor_record_offset":44536,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":45152,"parent_record_offset":45120,"ancestor_record_offset":44536,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":44948,"parent_record_offset":44924,"ancestor_record_offset":44536,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":45120,"parent_record_offset":45060,"ancestor_record_offset":44536,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":44696,"parent_record_offset":44660,"ancestor_record_offset":44536,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":44812,"parent_record_offset":44784,"ancestor_record_offset":44536,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":44900,"parent_record_offset":44784,"ancestor_record_offset":44536,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44976,"parent_record_offset":44948,"ancestor_record_offset":44536,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":44924,"parent_record_offset":44900,"ancestor_record_offset":44536,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":45004,"parent_record_offset":44948,"ancestor_record_offset":44536,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44612,"parent_record_offset":44588,"ancestor_record_offset":44536,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":44536,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":44724,"parent_record_offset":44696,"ancestor_record_offset":44536,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":45060,"parent_record_offset":44660,"ancestor_record_offset":44536,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":44588,"parent_record_offset":44564,"ancestor_record_offset":44536,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":44752,"parent_record_offset":44724,"ancestor_record_offset":44536,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":44784,"parent_record_offset":44752,"ancestor_record_offset":44536,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":44660,"parent_record_offset":44636,"ancestor_record_offset":44536,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":44636,"parent_record_offset":44612,"ancestor_record_offset":44536,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines
// 记录形式: compiled_procedure; instances=1
// 源码行: 527
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":44128,"rva":9381968,"va":5378091088}
// 编译实例 RVAs: 0x8f2850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3010657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e89f0fd2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378092156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fb420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10518665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e88980a000","size":5,"soft_fail":false,"target_va":5388612640,"va":5378093970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428bc1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33330635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8cb95fc01","size":5,"soft_fail":false,"target_va":5411422656,"va":5378092016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33573759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e87f4b0002","size":5,"soft_fail":false,"target_va":5411669968,"va":5378096204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33755375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8ef100302","size":5,"soft_fail":false,"target_va":5411847200,"va":5378091820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t2521669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8457a2600","size":5,"soft_fail":false,"target_va":5380614224,"va":5378092550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3737885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8e3f6c6ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5378094024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33254963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8336efb01","size":5,"soft_fail":false,"target_va":5411347040,"va":5378092072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33445309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8bd55fe01","size":5,"soft_fail":false,"target_va":5411541584,"va":5378096270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t33445210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e85a55fe01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378096561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=23
//     evidence={"assembly":"\tcallq\t33446243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e86359fe01","size":5,"soft_fail":false,"target_va":5411542296,"va":5378096048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t33445228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e86c55fe01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378096671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e848abffff","size":5,"soft_fail":false,"target_va":5378069536,"va":5378091219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33452066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e82270fe01","size":5,"soft_fail":false,"target_va":5411543360,"va":5378091289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e88f190000","size":5,"soft_fail":false,"target_va":5378098800,"va":5378092252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3293783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e857423200","size":5,"soft_fail":false,"target_va":5381385536,"va":5378091748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3298972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e89c563200","size":5,"soft_fail":false,"target_va":5381390640,"va":5378091663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3290351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e8ef343200","size":5,"soft_fail":false,"target_va":5381386480,"va":5378096124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3293515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e84b413200","size":5,"soft_fail":false,"target_va":5381384704,"va":5378091184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10727692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e80cb1a300","size":5,"soft_fail":false,"target_va":5388825296,"va":5378097599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33440841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378091088,"raw_hex":"e84944fe01","size":5,"soft_fail":false,"target_va":5411537104,"va":5378096258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49312,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":11299,"source_line":1653,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::back_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":55840,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":28184,"source_line":1677,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":48964,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4909,"source_line":1608,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":50488,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49668,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_front
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":51012,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":11295,"source_line":1799,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":500,"record_offset":54232,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":14547,"source_line":47,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":47432,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":55160,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":47508,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":48488,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":48276,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":48708,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":500,"record_offset":65172,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":500,"record_offset":60700,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":500,"record_offset":54928,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":500,"record_offset":52900,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":500,"record_offset":65144,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":48020,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":48372,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":500,"record_offset":47908,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":500,"record_offset":48460,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":28180,"source_line":817,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":500,"record_offset":48828,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":28182,"source_line":822,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":500,"record_offset":48432,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":28178,"source_line":813,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":49580,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":49492,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":45856,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":44424,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":44880,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":46840,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":47280,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":20717,"source_line":774,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":48224,"parent_record_offset":48200,"ancestor_record_offset":44128,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":500,"record_offset":48200,"parent_record_offset":48116,"ancestor_record_offset":44128,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":54388,"parent_record_offset":54260,"ancestor_record_offset":44128,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":51308,"parent_record_offset":51012,"ancestor_record_offset":44128,"inlinee":9874,"source_line":187,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49080,"parent_record_offset":49052,"ancestor_record_offset":44128,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":48988,"parent_record_offset":48964,"ancestor_record_offset":44128,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":50512,"parent_record_offset":50488,"ancestor_record_offset":44128,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49700,"parent_record_offset":49668,"ancestor_record_offset":44128,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":60756,"parent_record_offset":60728,"ancestor_record_offset":44128,"inlinee":26751,"source_line":1727,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":54260,"parent_record_offset":54232,"ancestor_record_offset":44128,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49180,"parent_record_offset":48988,"ancestor_record_offset":44128,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49024,"parent_record_offset":48988,"ancestor_record_offset":44128,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":49052,"parent_record_offset":49024,"ancestor_record_offset":44128,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":51168,"parent_record_offset":51012,"ancestor_record_offset":44128,"inlinee":11296,"source_line":223,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":54952,"parent_record_offset":54928,"ancestor_record_offset":44128,"inlinee":26146,"source_line":2940,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":55060,"parent_record_offset":55028,"ancestor_record_offset":44128,"inlinee":26150,"source_line":3051,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$3::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":500,"record_offset":60728,"parent_record_offset":60700,"ancestor_record_offset":44128,"inlinee":28186,"source_line":181,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":500,"record_offset":55004,"parent_record_offset":54952,"ancestor_record_offset":44128,"inlinee":26148,"source_line":13,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":500,"record_offset":53808,"parent_record_offset":53784,"ancestor_record_offset":44128,"inlinee":13999,"source_line":2832,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":49112,"parent_record_offset":49080,"ancestor_record_offset":44128,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":49216,"parent_record_offset":49180,"ancestor_record_offset":44128,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":47616,"parent_record_offset":47588,"ancestor_record_offset":44128,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":49136,"parent_record_offset":49112,"ancestor_record_offset":44128,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":48144,"parent_record_offset":48116,"ancestor_record_offset":44128,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":48116,"parent_record_offset":48092,"ancestor_record_offset":44128,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":47676,"parent_record_offset":47644,"ancestor_record_offset":44128,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":49264,"parent_record_offset":49240,"ancestor_record_offset":44128,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":49240,"parent_record_offset":49216,"ancestor_record_offset":44128,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":47644,"parent_record_offset":47616,"ancestor_record_offset":44128,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":44128,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":55208,"parent_record_offset":55160,"ancestor_record_offset":44128,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":47472,"parent_record_offset":47432,"ancestor_record_offset":44128,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":47540,"parent_record_offset":47508,"ancestor_record_offset":44128,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":55084,"parent_record_offset":55060,"ancestor_record_offset":44128,"inlinee":26151,"source_line":1176,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":47588,"parent_record_offset":47540,"ancestor_record_offset":44128,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":55028,"parent_record_offset":55004,"ancestor_record_offset":44128,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":500,"record_offset":47872,"parent_record_offset":47540,"ancestor_record_offset":44128,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46192,"parent_record_offset":46164,"ancestor_record_offset":44128,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46244,"parent_record_offset":46220,"ancestor_record_offset":44128,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46220,"parent_record_offset":46192,"ancestor_record_offset":44128,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46140,"parent_record_offset":46112,"ancestor_record_offset":44128,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46164,"parent_record_offset":46140,"ancestor_record_offset":44128,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46336,"parent_record_offset":46164,"ancestor_record_offset":44128,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":500,"record_offset":46448,"parent_record_offset":46424,"ancestor_record_offset":44128,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":500,"record_offset":47388,"parent_record_offset":47348,"ancestor_record_offset":44128,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":500,"record_offset":47180,"parent_record_offset":47156,"ancestor_record_offset":44128,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":500,"record_offset":47784,"parent_record_offset":47760,"ancestor_record_offset":44128,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":500,"record_offset":47760,"parent_record_offset":47540,"ancestor_record_offset":44128,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":500,"record_offset":54976,"parent_record_offset":54952,"ancestor_record_offset":44128,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":500,"record_offset":55108,"parent_record_offset":55084,"ancestor_record_offset":44128,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":500,"record_offset":46268,"parent_record_offset":46244,"ancestor_record_offset":44128,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":48168,"parent_record_offset":48144,"ancestor_record_offset":44128,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":49412,"parent_record_offset":49388,"ancestor_record_offset":44128,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":61156,"parent_record_offset":61128,"ancestor_record_offset":44128,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":500,"record_offset":44580,"parent_record_offset":44548,"ancestor_record_offset":44128,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":500,"record_offset":49520,"parent_record_offset":49492,"ancestor_record_offset":44128,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":500,"record_offset":49608,"parent_record_offset":49580,"ancestor_record_offset":44128,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":46008,"parent_record_offset":45964,"ancestor_record_offset":44128,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":500,"record_offset":44828,"parent_record_offset":44800,"ancestor_record_offset":44128,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":500,"record_offset":46796,"parent_record_offset":45964,"ancestor_record_offset":44128,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":46400,"parent_record_offset":46112,"ancestor_record_offset":44128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":47720,"parent_record_offset":47540,"ancestor_record_offset":44128,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":47156,"parent_record_offset":47132,"ancestor_record_offset":44128,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":45292,"parent_record_offset":45268,"ancestor_record_offset":44128,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":45268,"parent_record_offset":45244,"ancestor_record_offset":44128,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":45476,"parent_record_offset":45448,"ancestor_record_offset":44128,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":52928,"parent_record_offset":52900,"ancestor_record_offset":44128,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":46292,"parent_record_offset":46268,"ancestor_record_offset":44128,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":47932,"parent_record_offset":47908,"ancestor_record_offset":44128,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":48516,"parent_record_offset":48488,"ancestor_record_offset":44128,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":48304,"parent_record_offset":48276,"ancestor_record_offset":44128,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":48544,"parent_record_offset":48516,"ancestor_record_offset":44128,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":48332,"parent_record_offset":48304,"ancestor_record_offset":44128,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":500,"record_offset":47956,"parent_record_offset":47932,"ancestor_record_offset":44128,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":45220,"parent_record_offset":45196,"ancestor_record_offset":44128,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":45244,"parent_record_offset":45220,"ancestor_record_offset":44128,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":45548,"parent_record_offset":45520,"ancestor_record_offset":44128,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":45196,"parent_record_offset":45080,"ancestor_record_offset":44128,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":44548,"parent_record_offset":44520,"ancestor_record_offset":44128,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":44908,"parent_record_offset":44880,"ancestor_record_offset":44128,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":500,"record_offset":45048,"parent_record_offset":45016,"ancestor_record_offset":44128,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":500,"record_offset":45016,"parent_record_offset":44980,"ancestor_record_offset":44128,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":44956,"parent_record_offset":44932,"ancestor_record_offset":44128,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":44980,"parent_record_offset":44956,"ancestor_record_offset":44128,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":44932,"parent_record_offset":44908,"ancestor_record_offset":44128,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":45520,"parent_record_offset":44980,"ancestor_record_offset":44128,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44520,"parent_record_offset":44488,"ancestor_record_offset":44128,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44488,"parent_record_offset":44460,"ancestor_record_offset":44128,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44680,"parent_record_offset":44648,"ancestor_record_offset":44128,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44776,"parent_record_offset":44680,"ancestor_record_offset":44128,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44648,"parent_record_offset":44624,"ancestor_record_offset":44128,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44460,"parent_record_offset":44424,"ancestor_record_offset":44128,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":44800,"parent_record_offset":44776,"ancestor_record_offset":44128,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":45080,"parent_record_offset":45048,"ancestor_record_offset":44128,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":45364,"parent_record_offset":45080,"ancestor_record_offset":44128,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":45336,"parent_record_offset":45080,"ancestor_record_offset":44128,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":45140,"parent_record_offset":45080,"ancestor_record_offset":44128,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/intersect_meshes.rs
//     evidence={"module":500,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":44128,"inlinee":28182,"source_line":822,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":500,"record_offset":47132,"parent_record_offset":47072,"ancestor_record_offset":44128,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":46864,"parent_record_offset":46840,"ancestor_record_offset":44128,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":47312,"parent_record_offset":47280,"ancestor_record_offset":44128,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":46888,"parent_record_offset":46864,"ancestor_record_offset":44128,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":46988,"parent_record_offset":46964,"ancestor_record_offset":44128,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47348,"parent_record_offset":47312,"ancestor_record_offset":44128,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47012,"parent_record_offset":46988,"ancestor_record_offset":44128,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47072,"parent_record_offset":47012,"ancestor_record_offset":44128,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47044,"parent_record_offset":47012,"ancestor_record_offset":44128,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47104,"parent_record_offset":47072,"ancestor_record_offset":44128,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":47220,"parent_record_offset":47012,"ancestor_record_offset":44128,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":46964,"parent_record_offset":46940,"ancestor_record_offset":44128,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":500,"record_offset":46940,"parent_record_offset":46916,"ancestor_record_offset":44128,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":45892,"parent_record_offset":45856,"ancestor_record_offset":44128,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":64168,"parent_record_offset":64136,"ancestor_record_offset":44128,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":64668,"parent_record_offset":64408,"ancestor_record_offset":44128,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":500,"record_offset":46036,"parent_record_offset":46008,"ancestor_record_offset":44128,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":46112,"parent_record_offset":45964,"ancestor_record_offset":44128,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":46372,"parent_record_offset":46112,"ancestor_record_offset":44128,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":500,"record_offset":46064,"parent_record_offset":46036,"ancestor_record_offset":44128,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":500,"record_offset":45964,"parent_record_offset":45928,"ancestor_record_offset":44128,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":500,"record_offset":45928,"parent_record_offset":45892,"ancestor_record_offset":44128,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 544
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":67076,"rva":9389680,"va":5378098800}
// 编译实例 RVAs: 0x8f4670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e74b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10431792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098800,"raw_hex":"e8302d9f00","size":5,"soft_fail":false,"target_va":5388530864,"va":5378099067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141529160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12797116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098800,"raw_hex":"e8bc44c300","size":5,"soft_fail":false,"target_va":5390897504,"va":5378100383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: intersect_meshes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33749183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098800,"raw_hex":"e8bff80202","size":5,"soft_fail":false,"target_va":5411848096,"va":5378098908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":68920,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":500,"record_offset":68976,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::as_ivec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":67524,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":28189,"source_line":1017,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::round
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":67380,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":28187,"source_line":653,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":68556,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":25456,"source_line":1525,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":500,"record_offset":67296,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":500,"record_offset":70712,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":28191,"source_line":69,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":67584,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":500,"record_offset":70740,"parent_record_offset":67076,"ancestor_record_offset":67076,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":69092,"parent_record_offset":69028,"ancestor_record_offset":67076,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":68308,"parent_record_offset":68284,"ancestor_record_offset":67076,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":500,"record_offset":67876,"parent_record_offset":67852,"ancestor_record_offset":67076,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":500,"record_offset":67960,"parent_record_offset":67936,"ancestor_record_offset":67076,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":500,"record_offset":67900,"parent_record_offset":67876,"ancestor_record_offset":67076,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":500,"record_offset":68380,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":500,"record_offset":69028,"parent_record_offset":68976,"ancestor_record_offset":67076,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":500,"record_offset":68108,"parent_record_offset":68080,"ancestor_record_offset":67076,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":500,"record_offset":68080,"parent_record_offset":68052,"ancestor_record_offset":67076,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":500,"record_offset":67324,"parent_record_offset":67296,"ancestor_record_offset":67076,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":500,"record_offset":68588,"parent_record_offset":68556,"ancestor_record_offset":67076,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":500,"record_offset":69144,"parent_record_offset":69028,"ancestor_record_offset":67076,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":68224,"parent_record_offset":68200,"ancestor_record_offset":67076,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::round
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":500,"record_offset":67408,"parent_record_offset":67380,"ancestor_record_offset":67076,"inlinee":28188,"source_line":192,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::IVec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":500,"record_offset":67552,"parent_record_offset":67524,"ancestor_record_offset":67076,"inlinee":28191,"source_line":69,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::i32::ivec3::impl$139::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/i32/ivec3.rs
//     evidence={"module":500,"record_offset":68332,"parent_record_offset":68308,"ancestor_record_offset":67076,"inlinee":28193,"source_line":20,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":68020,"parent_record_offset":67992,"ancestor_record_offset":67076,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":68052,"parent_record_offset":68020,"ancestor_record_offset":67076,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":500,"record_offset":67992,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":500,"record_offset":67852,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":500,"record_offset":68408,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":500,"record_offset":67936,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":500,"record_offset":67824,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":500,"record_offset":68284,"parent_record_offset":68260,"ancestor_record_offset":67076,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":67608,"parent_record_offset":67584,"ancestor_record_offset":67076,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":67632,"parent_record_offset":67608,"ancestor_record_offset":67076,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":68260,"parent_record_offset":68152,"ancestor_record_offset":67076,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":500,"record_offset":67664,"parent_record_offset":67632,"ancestor_record_offset":67076,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":68200,"parent_record_offset":68176,"ancestor_record_offset":67076,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":68500,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":68176,"parent_record_offset":68152,"ancestor_record_offset":67076,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":67716,"parent_record_offset":67692,"ancestor_record_offset":67076,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":67692,"parent_record_offset":67632,"ancestor_record_offset":67076,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":70444,"parent_record_offset":69492,"ancestor_record_offset":67076,"inlinee":10143,"source_line":1263,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":70472,"parent_record_offset":70444,"ancestor_record_offset":67076,"inlinee":10144,"source_line":1257,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":67740,"parent_record_offset":67716,"ancestor_record_offset":67076,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":67796,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":500,"record_offset":68152,"parent_record_offset":67740,"ancestor_record_offset":67076,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":500,"record_offset":67432,"parent_record_offset":67408,"ancestor_record_offset":67076,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 813
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E12
// 首条位置: {"module":500,"inlinee":28178}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 817
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E14
// 首条位置: {"module":500,"inlinee":28180}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 822
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E16
// 首条位置: {"module":500,"inlinee":28182}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::intersect_meshes::on_exit_game_state
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 852
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D3C
// 首条位置: {"module":1596,"inlinee":32060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1611,"record_offset":2435756,"parent_record_offset":2435732,"ancestor_record_offset":2435732,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":1611,"record_offset":2435812,"parent_record_offset":2435732,"ancestor_record_offset":2435732,"inlinee":10653,"source_line":286,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1611,"record_offset":2435780,"parent_record_offset":2435756,"ancestor_record_offset":2435732,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::clear
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"indexmap","version":"2.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.7.1/src/map.rs
//     evidence={"module":1611,"record_offset":2435840,"parent_record_offset":2435812,"ancestor_record_offset":2435732,"inlinee":10619,"source_line":278,"archive":"_pdb/B/modules/01611.jsonl.gz"}
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
// 完整结构及原始类型记录: intersect_meshes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::IntersectionOutput
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::collision::intersect_meshes::IntersectionOutput", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::collision::intersect_meshes::IntersectionOutput,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::IntersectionOutput>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::IntersectionOutput"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::Bucket", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "core::ptr::drop_in_place", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState>", "constructor": "indexmap::map::IndexMap", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<indexmap::map::IndexMap<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["indexmap::map::IndexMap<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::remove_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>", "constructor": "petgraph::graphmap::GraphMap::remove_node", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::add_edge<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>", "constructor": "petgraph::graphmap::GraphMap::add_edge", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::GraphMap::add_node<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>", "constructor": "petgraph::graphmap::GraphMap::add_node", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::impl$19::default<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>", "constructor": "petgraph::graphmap::impl$19::default", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::insert_full<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::insert_full", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::swap_remove_full<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle> >", "constructor": "indexmap::map::core::IndexMapCore::swap_remove_full", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::pop<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::pop", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::IndexMapCore::clear<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::IndexMapCore::clear", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["indexmap::Bucket<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>", "constructor": "indexmap::map::core::get_hash::closure_env$0", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<usize,allocator_api2::stable::alloc::global::Global,indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["usize", "allocator_api2::stable::alloc::global::Global", "indexmap::map::core::get_hash::closure_env$0<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "constructor": "petgraph::graphmap::AllEdges", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >", "core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >", "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$0::spec_extend", "arguments": ["country_core::systems::collision::BirdNestCandidateLocation", "core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "petgraph::graphmap::impl$6::add_edge::closure_env$1<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>", "constructor": "petgraph::graphmap::impl$6::add_edge::closure_env$1", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "enum2$<petgraph::Undirected>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::core::entry::Entry::or_insert_with<country_core::systems::collision::world::SubworldColliderHandle,alloc::vec::Vec<tuple$<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,petgraph::graphmap::impl$6::add_edge::closure_env$1<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState> >", "constructor": "indexmap::map::core::entry::Entry::or_insert_with", "arguments": ["country_core::systems::collision::world::SubworldColliderHandle", "alloc::vec::Vec<tuple$<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>", "petgraph::graphmap::impl$6::add_edge::closure_env$1<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::swap_remove<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle> >", "constructor": "indexmap::map::IndexMap::swap_remove", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "std::hash::random::RandomState", "tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "indexmap::map::IndexMap::hash<tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,std::hash::random::RandomState,tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle> >", "constructor": "indexmap::map::IndexMap::hash", "arguments": ["tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>", "country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts", "std::hash::random::RandomState", "tuple$<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::MeshIntersectionGraph
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 65
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> > >,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> > >", "bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>", "tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$12::init_state<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_ecs::system::system_param::impl$12::init_state", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::systems::collision::intersect_meshes::MeshIntersectionGraph"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$10::into_configs<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >", "constructor": "bevy_ecs::schedule::config::impl$10::into_configs", "arguments": ["tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> >", "void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 35
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "constructor": "bevy_ecs::system::commands::command::insert_resource::closure_env$0", "arguments": ["country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber> >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber> >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber> >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::insert_resource<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "constructor": "bevy_ecs::system::commands::Commands::insert_resource", "arguments": ["country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$12::init_state<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "constructor": "bevy_ecs::system::system_param::impl$12::init_state", "arguments": ["country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "constructor": "tuple$", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::ScheduleConfigTupleMarker", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)>", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::Polyline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::Polyline", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>", "constructor": "alloc::vec::Vec::retain_mut", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::Polyline", "alloc::alloc::Global", "country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylineMergeScratch"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::PolylinePoint
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint", "core::array::iter::IntoIter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::iter::Iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint>", "constructor": "alloc::collections::vec_deque::iter::Iter", "arguments": ["country_core::systems::collision::intersect_meshes::polyline::PolylinePoint"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::collections::vec_deque::iter::Iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint>,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::collections::vec_deque::iter::Iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint>", "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<alloc::collections::vec_deque::iter::Iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint>,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::map::Map<alloc::collections::vec_deque::iter::Iter<country_core::systems::collision::intersect_meshes::polyline::PolylinePoint>,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::collision::intersect_meshes::polyline::VertexId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>", "constructor": "tuple$", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<glam::i32::ivec3::IVec3,country_core::systems::collision::intersect_meshes::polyline::VertexId,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["glam::i32::ivec3::IVec3", "country_core::systems::collision::intersect_meshes::polyline::VertexId", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: intersect_meshes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: intersect_meshes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 236; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections
// 信息 ID: 054fd578c703354757c90b51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4a67c61cfb7a77b85b6e6789: void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > > @ 0x14cb995 -> 0xa97b70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshMeshIntersectionSubscriber>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>)> > @ 0x17cef93 -> 0xa97b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$0
// 信息 ID: 7541f1072f4e9401b6abd8f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=52416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$1
// 信息 ID: 1d0b72e60d22bd92850f8034
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=59272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure$3
// 信息 ID: 062372303940e63b4f2f2268
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::systems::collision::BirdNestCandidateLocation,core::iter::adapters::flatten::FlatMap<petgraph::graphmap::AllEdges<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts,enum2$<petgraph::Undirected> >,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::systems::collision::BirdNestCandidateLocation> >,country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections::closure_env$3>,alloc::alloc::Global>; module=1585, inline_record=147484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::from_parry_triangle
// 信息 ID: 79561985afd4b20773c5f372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh; module=559, inline_record=67372; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh; module=559, inline_record=67408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::impl$3::default
// 信息 ID: b30e6d8dc5834185ab8f7297
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::impl$0::build; module=596, inline_record=107860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh
// 信息 ID: 0c17f1a31385bc192bac2371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x17d164b -> 0xa9afa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure$0
// 信息 ID: 03c77b5cc86aa7b14ea30a7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::intersect_mesh_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0 <- parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_bvtt<u32,u32,parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor<u32,u32,country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_bvtt<u32,u32,parry3d::query::visitors::bounding_volume_intersections_simultaneous_visitor::BoundingVolumeIntersectionsSimultaneousVisitor<u32,u32,country_core::systems::collision::intersect_meshes::intersect_mesh_mesh::closure_env$0> >; module=1665, inline_record=21488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders
// 信息 ID: b72a4338cc67c3ef49348ff6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=41876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure$0
// 信息 ID: 5a0c30ba2f72af0575bd5226
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0 <- parry3d::partitioning::qbvh::qbvh::Qbvh::traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut$<country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0>; module=1734, inline_record=119492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::on_exit_game_state
// 信息 ID: 9dc8739cc9c1479e03122950
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4d0dfad491692889285b6ba6: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> > >; module=1611, inline_record=2435732; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)> >; module=1734, inline_record=8424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize
// 信息 ID: 41aece55aae8ba692470a06e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$11::eq
// 信息 ID: 3d6f8b8fd4eecb3464bdfd5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$2::default
// 信息 ID: e5facbdbb844601c02205704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=44536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::impl$6::eq
// 信息 ID: 4e6d28759d8de288eb217eb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=48432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=50460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=51680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=53276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=53304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=55812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=57012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines; module=500, inline_record=58116; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines
// 信息 ID: 75a67e3b83d8e66982f7a7fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x17d167c -> 0x8f2850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines::closure$0
// 信息 ID: ac3af4f4d71f489f15259eb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines @ 0x8f2cdc -> 0x8f4670 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines @ 0x8f2ced -> 0x8f4670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::remove_invalid_polylines
// 信息 ID: 51720154ed8a1eefccd905cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=50188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure$0
// 信息 ID: 041a5dee2722dc405ce5bfce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::intersect_meshes::remove_invalid_polylines", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0 <- alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; module=1585, inline_record=72344; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain_mut<country_core::systems::collision::intersect_meshes::polyline::Polyline,alloc::alloc::Global,country_core::systems::collision::intersect_meshes::remove_invalid_polylines::closure_env$0>; module=1585, inline_record=74092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::should_intersect_collider_meshes
// 信息 ID: 3c5f2335042881901e9d23c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<rapier3d::geometry::collider_components::ColliderHandle> >,country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders::closure_env$0> @ 0x17d15c6 -> 0xa9b3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::intersect_meshes::subscribe_mesh_mesh_intersections
// 信息 ID: 47fac3e0c9996fea4dc0c4be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d1b1f6201f2a09a88654812c: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> > >; module=1611, inline_record=1884500; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> >; module=1734, inline_record=10444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
