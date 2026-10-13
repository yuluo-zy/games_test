// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_walls.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a2cb41ab790df2ffac76e840c36e07adbd8d4198b0e46e897ca39dbeb5bfc6af
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: construct_walls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 371, 1226, 1247, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":371,"record_offset":4032,"rva":5151088,"va":5373860208}
// 编译实例 RVAs: 0x4e9970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4249180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8a429bfff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373862695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4049838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e85234c2ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5373863385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da730
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2168707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e87de8deff","size":5,"soft_fail":false,"target_va":5371701040,"va":5373869742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1113769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e85701efff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373863460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1116447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8e1f6eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373866154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t197736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e868040300","size":5,"soft_fail":false,"target_va":5374061056,"va":5373863315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405feee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1129610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e88a3c1100","size":5,"soft_fail":false,"target_va":5374996192,"va":5373866577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1348181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e855921400","size":5,"soft_fail":false,"target_va":5375213232,"va":5373865046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2011047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8a7af1e00","size":5,"soft_fail":false,"target_va":5375881440,"va":5373870388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3074184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e888e82e00","size":5,"soft_fail":false,"target_va":5376939904,"va":5373865715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb310
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4068763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89b153e00","size":5,"soft_fail":false,"target_va":5377930000,"va":5373861232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fa40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4539983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e84f464500","size":5,"soft_fail":false,"target_va":5378406976,"va":5373866988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91b30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8021023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e81f647a00","size":5,"soft_fail":false,"target_va":5381888816,"va":5373867788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8048026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89acd7a00","size":5,"soft_fail":false,"target_va":5381915200,"va":5373867169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caa1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8121734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e886ed7b00","size":5,"soft_fail":false,"target_va":5381988800,"va":5373867061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8136625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8b1277c00","size":5,"soft_fail":false,"target_va":5382000496,"va":5373863866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb1700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8156831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89f767c00","size":5,"soft_fail":false,"target_va":5382018816,"va":5373861980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8168486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e826a47c00","size":5,"soft_fail":false,"target_va":5382032368,"va":5373863877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ce2260
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8351786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e82a707f00","size":5,"soft_fail":false,"target_va":5382218336,"va":5373866545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420c18b0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29188223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e87f60bd01","size":5,"soft_fail":false,"target_va":5403056304,"va":5373868076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29726105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89995c501","size":5,"soft_fail":false,"target_va":5403593344,"va":5373867234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29109144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8982bbc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373868995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29058176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e88064bb01","size":5,"soft_fail":false,"target_va":5402919776,"va":5373861595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2556851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e84dfcd8ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5373861902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::downgrade::panic_cold_display::h190c4236508ec4a3 (.llvm.11040669212312333322)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29185637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e86556bd01","size":5,"soft_fail":false,"target_va":5403054496,"va":5373868854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8353116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e85c757f00","size":5,"soft_fail":false,"target_va":5382219824,"va":5373866703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8212307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8534f7d00","size":5,"soft_fail":false,"target_va":5382080048,"va":5373867736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29109342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e85e2cbc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373869341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29110004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8f42ebc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373868871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29185651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e87356bd01","size":5,"soft_fail":false,"target_va":5403054624,"va":5373868968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29109780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8142ebc01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373869159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e883d8ffff","size":5,"soft_fail":false,"target_va":5373858640,"va":5373868744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::walls::wall_holes::WallHoleStorage> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e84dd4ffff","size":5,"soft_fail":false,"target_va":5373858832,"va":5373870014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29119283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e83353bc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373860840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e84efaffff","size":5,"soft_fail":false,"target_va":5373858976,"va":5373860429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29120145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89156bc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373860506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::InstancedWall::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1125827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8c32d1100","size":5,"soft_fail":false,"target_va":5374993408,"va":5373867576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::InstancedWall::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1126652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8fc301100","size":5,"soft_fail":false,"target_va":5374993456,"va":5373866799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1900268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8ecfe1c00","size":5,"soft_fail":false,"target_va":5375763616,"va":5373863343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1900298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e80aff1c00","size":5,"soft_fail":false,"target_va":5375763568,"va":5373863265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1900200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8a8fe1c00","size":5,"soft_fail":false,"target_va":5375763488,"va":5373863283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8068757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8951e7b00","size":5,"soft_fail":false,"target_va":5381932784,"va":5373864022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8068869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8051f7b00","size":5,"soft_fail":false,"target_va":5381932864,"va":5373863990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8067157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e855187b00","size":5,"soft_fail":false,"target_va":5381932832,"va":5373865670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1573452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e84c021800","size":5,"soft_fail":false,"target_va":5375434976,"va":5373861519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1572989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e87d001800","size":5,"soft_fail":false,"target_va":5375436032,"va":5373863038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::WallEntities::wall_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1568404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e894ee1700","size":5,"soft_fail":false,"target_va":5375434752,"va":5373866343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1303994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8bae51300","size":5,"soft_fail":false,"target_va":5375168608,"va":5373864609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_wip_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1303916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e86ce51300","size":5,"soft_fail":false,"target_va":5375168704,"va":5373864783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::clone_and_reopen
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1304144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e850e61300","size":5,"soft_fail":false,"target_va":5375168800,"va":5373864651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::finalize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1307804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89cf41300","size":5,"soft_fail":false,"target_va":5375169664,"va":5373861855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::new_finalized
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1300412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8bcd71300","size":5,"soft_fail":false,"target_va":5375168512,"va":5373868095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8339816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e868417f00","size":5,"soft_fail":false,"target_va":5382206304,"va":5373866483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1860957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e85d651c00","size":5,"soft_fail":false,"target_va":5375724992,"va":5373864030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2495901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e89d152600","size":5,"soft_fail":false,"target_va":5376356912,"va":5373861006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2499787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8cb242600","size":5,"soft_fail":false,"target_va":5376360560,"va":5373860768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2495737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8f9142600","size":5,"soft_fail":false,"target_va":5376356144,"va":5373860402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8004631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e817247a00","size":5,"soft_fail":false,"target_va":5381865424,"va":5373860788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::half_timbering
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8271315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8d3357e00","size":5,"soft_fail":false,"target_va":5382136512,"va":5373865192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::roof_outline
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8265939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8d3207e00","size":5,"soft_fail":false,"target_va":5382131200,"va":5373865256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8315603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e8d3e27e00","size":5,"soft_fail":false,"target_va":5382181040,"va":5373865432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::add_wall_corners
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8256529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e811fc7d00","size":5,"soft_fail":false,"target_va":5382121472,"va":5373864938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_skirts::add_skirts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8308226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e802c67e00","size":5,"soft_fail":false,"target_va":5382173584,"va":5373865353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8323082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e80a007f00","size":5,"soft_fail":false,"target_va":5382189152,"va":5373866065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4091772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373860208,"raw_hex":"e87c6f3e00","size":5,"soft_fail":false,"target_va":5377955840,"va":5373864063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373860208,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373867272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10336,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":12918,"source_line":2673,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":12532,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":9492,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10712,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":8164,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":371,"record_offset":11400,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":9072,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":21996,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":6968,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":12560,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":371,"record_offset":12820,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":371,"record_offset":21872,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":7568,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":7512,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":19628,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":19536,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":14912,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":19944,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":371,"record_offset":23584,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":371,"record_offset":21900,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":371,"record_offset":10244,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21341,"source_line":2912,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":371,"record_offset":22108,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":371,"record_offset":22384,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":14712,"source_line":117,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":371,"record_offset":19480,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":371,"record_offset":10216,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_trigger_volume::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_trigger_volume.rs
//     evidence={"module":371,"record_offset":21236,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21531,"source_line":115,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":5920,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":4332,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":4908,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":17044,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":15004,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":11516,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":22988,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$37::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":9132,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":17308,"source_line":2193,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":9620,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$5
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":10768,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21524,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::create_wall
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":22892,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21535,"source_line":381,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":371,"record_offset":10684,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21523,"source_line":51,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":371,"record_offset":14664,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":371,"record_offset":19564,"parent_record_offset":4032,"ancestor_record_offset":4032,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":4032,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":4032,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":20200,"parent_record_offset":20176,"ancestor_record_offset":4032,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":22452,"parent_record_offset":22424,"ancestor_record_offset":4032,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":14076,"parent_record_offset":13984,"ancestor_record_offset":4032,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":371,"record_offset":20176,"parent_record_offset":20076,"ancestor_record_offset":4032,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":371,"record_offset":22424,"parent_record_offset":22384,"ancestor_record_offset":4032,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":20108,"parent_record_offset":20076,"ancestor_record_offset":4032,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":8284,"parent_record_offset":8252,"ancestor_record_offset":4032,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":4032,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":13984,"parent_record_offset":13956,"ancestor_record_offset":4032,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":13956,"parent_record_offset":13928,"ancestor_record_offset":4032,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":14196,"parent_record_offset":13956,"ancestor_record_offset":4032,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":20076,"parent_record_offset":20048,"ancestor_record_offset":4032,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":371,"record_offset":22608,"parent_record_offset":22384,"ancestor_record_offset":4032,"inlinee":11196,"source_line":1613,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":371,"record_offset":22660,"parent_record_offset":22608,"ancestor_record_offset":4032,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":8252,"parent_record_offset":8212,"ancestor_record_offset":4032,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":11228,"parent_record_offset":10768,"ancestor_record_offset":4032,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10364,"parent_record_offset":10336,"ancestor_record_offset":4032,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":4032,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":14260,"parent_record_offset":14236,"ancestor_record_offset":4032,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":8976,"parent_record_offset":8212,"ancestor_record_offset":4032,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10936,"parent_record_offset":10892,"ancestor_record_offset":4032,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":9004,"parent_record_offset":8212,"ancestor_record_offset":4032,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":8316,"parent_record_offset":8212,"ancestor_record_offset":4032,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":13900,"parent_record_offset":13864,"ancestor_record_offset":4032,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":13928,"parent_record_offset":13900,"ancestor_record_offset":4032,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":4032,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":11340,"parent_record_offset":10768,"ancestor_record_offset":4032,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":14476,"parent_record_offset":14432,"ancestor_record_offset":4032,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10452,"parent_record_offset":10364,"ancestor_record_offset":4032,"inlinee":8528,"source_line":2674,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":371,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":4032,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":371,"record_offset":23088,"parent_record_offset":23064,"ancestor_record_offset":4032,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":371,"record_offset":14584,"parent_record_offset":14476,"ancestor_record_offset":4032,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":371,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":4032,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":371,"record_offset":14236,"parent_record_offset":13864,"ancestor_record_offset":4032,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":371,"record_offset":13832,"parent_record_offset":13800,"ancestor_record_offset":4032,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":371,"record_offset":13864,"parent_record_offset":13832,"ancestor_record_offset":4032,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":371,"record_offset":13032,"parent_record_offset":12888,"ancestor_record_offset":4032,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":9100,"parent_record_offset":9072,"ancestor_record_offset":4032,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":11104,"parent_record_offset":10768,"ancestor_record_offset":4032,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":4032,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":371,"record_offset":12888,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":4032,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10824,"parent_record_offset":10768,"ancestor_record_offset":4032,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10892,"parent_record_offset":10852,"ancestor_record_offset":4032,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":4032,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":7032,"parent_record_offset":6992,"ancestor_record_offset":4032,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":8424,"parent_record_offset":8392,"ancestor_record_offset":4032,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":4032,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":371,"record_offset":13676,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":371,"record_offset":13308,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":371,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":4032,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":371,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":4032,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":371,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":4032,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":371,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":4032,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":371,"record_offset":13384,"parent_record_offset":13360,"ancestor_record_offset":4032,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":371,"record_offset":13456,"parent_record_offset":13432,"ancestor_record_offset":4032,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":371,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":4032,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":371,"record_offset":13612,"parent_record_offset":13332,"ancestor_record_offset":4032,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":371,"record_offset":13360,"parent_record_offset":13332,"ancestor_record_offset":4032,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":371,"record_offset":13280,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":371,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":4032,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":371,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":4032,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":371,"record_offset":14020,"parent_record_offset":13984,"ancestor_record_offset":4032,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":371,"record_offset":14044,"parent_record_offset":14020,"ancestor_record_offset":4032,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":371,"record_offset":13096,"parent_record_offset":13064,"ancestor_record_offset":4032,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6320,"parent_record_offset":6292,"ancestor_record_offset":4032,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":4032,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":4032,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":4032,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6292,"parent_record_offset":6260,"ancestor_record_offset":4032,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6464,"parent_record_offset":6292,"ancestor_record_offset":4032,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":371,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":4032,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":371,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":4032,"inlinee":8530,"source_line":1709,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":371,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":4032,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":371,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":4032,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":371,"record_offset":11196,"parent_record_offset":11160,"ancestor_record_offset":4032,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":371,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":4032,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":371,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":4032,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":371,"record_offset":9404,"parent_record_offset":9372,"ancestor_record_offset":4032,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":371,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":4032,"inlinee":8944,"source_line":21,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":371,"record_offset":19592,"parent_record_offset":19564,"ancestor_record_offset":4032,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":371,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":4032,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":371,"record_offset":22716,"parent_record_offset":22608,"ancestor_record_offset":4032,"inlinee":8943,"source_line":216,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":371,"record_offset":10620,"parent_record_offset":10364,"ancestor_record_offset":4032,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":371,"record_offset":8516,"parent_record_offset":8488,"ancestor_record_offset":4032,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":371,"record_offset":8488,"parent_record_offset":8452,"ancestor_record_offset":4032,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":371,"record_offset":8688,"parent_record_offset":8660,"ancestor_record_offset":4032,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":371,"record_offset":8344,"parent_record_offset":8212,"ancestor_record_offset":4032,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":371,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":4032,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":371,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":4032,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":371,"record_offset":14388,"parent_record_offset":14332,"ancestor_record_offset":4032,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":371,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":4032,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":14780,"parent_record_offset":14664,"ancestor_record_offset":4032,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":4032,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":8852,"parent_record_offset":8392,"ancestor_record_offset":4032,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":4032,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":14432,"parent_record_offset":14388,"ancestor_record_offset":4032,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":371,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":4032,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":371,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":4032,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":371,"record_offset":10296,"parent_record_offset":10268,"ancestor_record_offset":4032,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":371,"record_offset":7104,"parent_record_offset":7056,"ancestor_record_offset":4032,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":371,"record_offset":20140,"parent_record_offset":20108,"ancestor_record_offset":4032,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":371,"record_offset":17964,"parent_record_offset":17936,"ancestor_record_offset":4032,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":371,"record_offset":4544,"parent_record_offset":4448,"ancestor_record_offset":4032,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":371,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":4032,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":371,"record_offset":12976,"parent_record_offset":12888,"ancestor_record_offset":4032,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19876,"parent_record_offset":19824,"ancestor_record_offset":4032,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19904,"parent_record_offset":19824,"ancestor_record_offset":4032,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":4032,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":371,"record_offset":8572,"parent_record_offset":8516,"ancestor_record_offset":4032,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":371,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":4032,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":8544,"parent_record_offset":8516,"ancestor_record_offset":4032,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":6156,"parent_record_offset":6128,"ancestor_record_offset":4032,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":4032,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":5504,"parent_record_offset":5044,"ancestor_record_offset":4032,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":6204,"parent_record_offset":6028,"ancestor_record_offset":4032,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":8660,"parent_record_offset":8488,"ancestor_record_offset":4032,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":371,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":4032,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":6528,"parent_record_offset":6232,"ancestor_record_offset":4032,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":371,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":4032,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":371,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":4032,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":371,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":4032,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":371,"record_offset":18172,"parent_record_offset":18148,"ancestor_record_offset":4032,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":4032,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":4032,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4032,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":11456,"parent_record_offset":11400,"ancestor_record_offset":4032,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4032,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":371,"record_offset":5464,"parent_record_offset":5432,"ancestor_record_offset":4032,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":4032,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":4032,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":4032,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":371,"record_offset":6928,"parent_record_offset":5992,"ancestor_record_offset":4032,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":371,"record_offset":14864,"parent_record_offset":14836,"ancestor_record_offset":4032,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":371,"record_offset":13252,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":371,"record_offset":22260,"parent_record_offset":22232,"ancestor_record_offset":4032,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":371,"record_offset":22288,"parent_record_offset":22260,"ancestor_record_offset":4032,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":371,"record_offset":4480,"parent_record_offset":4448,"ancestor_record_offset":4032,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":371,"record_offset":7356,"parent_record_offset":7056,"ancestor_record_offset":4032,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":371,"record_offset":4508,"parent_record_offset":4480,"ancestor_record_offset":4032,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":371,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":4032,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":371,"record_offset":7204,"parent_record_offset":7168,"ancestor_record_offset":4032,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":371,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":4032,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":4032,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4032,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":4032,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":4032,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":4032,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":371,"record_offset":5208,"parent_record_offset":5100,"ancestor_record_offset":4032,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":371,"record_offset":10268,"parent_record_offset":10244,"ancestor_record_offset":4032,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":371,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":4032,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":371,"record_offset":4940,"parent_record_offset":4908,"ancestor_record_offset":4032,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":371,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":4032,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":371,"record_offset":5044,"parent_record_offset":5012,"ancestor_record_offset":4032,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":371,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":4032,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":371,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":4032,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":371,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4032,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":371,"record_offset":5536,"parent_record_offset":5012,"ancestor_record_offset":4032,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4644,"parent_record_offset":4612,"ancestor_record_offset":4032,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4804,"parent_record_offset":4644,"ancestor_record_offset":4032,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":4032,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":22204,"parent_record_offset":22176,"ancestor_record_offset":4032,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4364,"parent_record_offset":4332,"ancestor_record_offset":4032,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4032,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4420,"parent_record_offset":4388,"ancestor_record_offset":4032,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":371,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":4032,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":371,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":4032,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":371,"record_offset":5348,"parent_record_offset":5100,"ancestor_record_offset":4032,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":371,"record_offset":5320,"parent_record_offset":5100,"ancestor_record_offset":4032,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":371,"record_offset":5152,"parent_record_offset":5100,"ancestor_record_offset":4032,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":371,"record_offset":22828,"parent_record_offset":22608,"ancestor_record_offset":4032,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":371,"record_offset":22772,"parent_record_offset":22608,"ancestor_record_offset":4032,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":371,"record_offset":22852,"parent_record_offset":22828,"ancestor_record_offset":4032,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":371,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":4032,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":371,"record_offset":22136,"parent_record_offset":22108,"ancestor_record_offset":4032,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::RtMesh::set_albedo
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":371,"record_offset":11160,"parent_record_offset":10768,"ancestor_record_offset":4032,"inlinee":21533,"source_line":67,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":371,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":4032,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":371,"record_offset":13064,"parent_record_offset":12844,"ancestor_record_offset":4032,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":371,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":4032,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":371,"record_offset":13172,"parent_record_offset":13148,"ancestor_record_offset":4032,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":371,"record_offset":13124,"parent_record_offset":13096,"ancestor_record_offset":4032,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":371,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":4032,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":4032,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":4032,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":4032,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":15928,"parent_record_offset":15036,"ancestor_record_offset":4032,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":371,"record_offset":15036,"parent_record_offset":15004,"ancestor_record_offset":4032,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":12268,"parent_record_offset":12044,"ancestor_record_offset":4032,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":4032,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":371,"record_offset":11596,"parent_record_offset":11564,"ancestor_record_offset":4032,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":4032,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12156,"parent_record_offset":12044,"ancestor_record_offset":4032,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":4032,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":10004,"parent_record_offset":9696,"ancestor_record_offset":4032,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":18148,"parent_record_offset":17208,"ancestor_record_offset":4032,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12476,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9288,"parent_record_offset":9228,"ancestor_record_offset":4032,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":4032,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12072,"parent_record_offset":12044,"ancestor_record_offset":4032,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":4032,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":11624,"parent_record_offset":11564,"ancestor_record_offset":4032,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17208,"parent_record_offset":17184,"ancestor_record_offset":4032,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9204,"parent_record_offset":9180,"ancestor_record_offset":4032,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":4032,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":18008,"parent_record_offset":17208,"ancestor_record_offset":4032,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9260,"parent_record_offset":9228,"ancestor_record_offset":4032,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":11672,"parent_record_offset":11648,"ancestor_record_offset":4032,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17280,"parent_record_offset":17248,"ancestor_record_offset":4032,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17456,"parent_record_offset":17280,"ancestor_record_offset":4032,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17620,"parent_record_offset":17280,"ancestor_record_offset":4032,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17652,"parent_record_offset":17620,"ancestor_record_offset":4032,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":4032,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17248,"parent_record_offset":17208,"ancestor_record_offset":4032,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":15200,"parent_record_offset":15124,"ancestor_record_offset":4032,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":4032,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17912,"parent_record_offset":17248,"ancestor_record_offset":4032,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":371,"record_offset":9776,"parent_record_offset":9744,"ancestor_record_offset":4032,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":371,"record_offset":9808,"parent_record_offset":9776,"ancestor_record_offset":4032,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":371,"record_offset":12012,"parent_record_offset":11892,"ancestor_record_offset":4032,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":371,"record_offset":9744,"parent_record_offset":9696,"ancestor_record_offset":4032,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":11724,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":9668,"parent_record_offset":9644,"ancestor_record_offset":4032,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":12044,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":17676,"parent_record_offset":17652,"ancestor_record_offset":4032,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":371,"record_offset":18120,"parent_record_offset":17208,"ancestor_record_offset":4032,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":371,"record_offset":11752,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":371,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":4032,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":371,"record_offset":11836,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":371,"record_offset":12384,"parent_record_offset":11672,"ancestor_record_offset":4032,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":371,"record_offset":9372,"parent_record_offset":9316,"ancestor_record_offset":4032,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":371,"record_offset":17100,"parent_record_offset":17076,"ancestor_record_offset":4032,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":371,"record_offset":17184,"parent_record_offset":17160,"ancestor_record_offset":4032,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":371,"record_offset":15736,"parent_record_offset":15516,"ancestor_record_offset":4032,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":371,"record_offset":19824,"parent_record_offset":19760,"ancestor_record_offset":4032,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":371,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":4032,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":5956,"parent_record_offset":5920,"ancestor_record_offset":4032,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":371,"record_offset":6068,"parent_record_offset":6028,"ancestor_record_offset":4032,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":6232,"parent_record_offset":6028,"ancestor_record_offset":4032,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":371,"record_offset":6500,"parent_record_offset":6232,"ancestor_record_offset":4032,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":4032,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":17076,"parent_record_offset":17044,"ancestor_record_offset":4032,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":371,"record_offset":17160,"parent_record_offset":17044,"ancestor_record_offset":4032,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":371,"record_offset":6096,"parent_record_offset":6068,"ancestor_record_offset":4032,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":371,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":4032,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":371,"record_offset":6028,"parent_record_offset":5992,"ancestor_record_offset":4032,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":371,"record_offset":5992,"parent_record_offset":5956,"ancestor_record_offset":4032,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":4032,"inlinee":21521,"source_line":72,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":19656,"parent_record_offset":19628,"ancestor_record_offset":4032,"inlinee":21528,"source_line":279,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":19760,"parent_record_offset":19732,"ancestor_record_offset":4032,"inlinee":21529,"source_line":298,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$6
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":371,"record_offset":21936,"parent_record_offset":21900,"ancestor_record_offset":4032,"inlinee":21536,"source_line":346,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":371,"record_offset":14836,"parent_record_offset":14664,"ancestor_record_offset":4032,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5411
// 首条位置: {"module":371,"inlinee":21521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":371,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":8900,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5418
// 首条位置: {"module":371,"inlinee":21528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":371,"record_offset":19692,"parent_record_offset":19656,"ancestor_record_offset":19656,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5419
// 首条位置: {"module":371,"inlinee":21529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":371,"record_offset":19792,"parent_record_offset":19760,"ancestor_record_offset":19760,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":371,"record_offset":19824,"parent_record_offset":19760,"ancestor_record_offset":19760,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19876,"parent_record_offset":19824,"ancestor_record_offset":19760,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19904,"parent_record_offset":19824,"ancestor_record_offset":19760,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":371,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":19760,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5414
// 首条位置: {"module":371,"inlinee":21524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":11228,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":11340,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":11104,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10824,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":371,"record_offset":11256,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtMesh::set_albedo
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":371,"record_offset":11160,"parent_record_offset":10768,"ancestor_record_offset":10768,"inlinee":21533,"source_line":67,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":371,"record_offset":11000,"parent_record_offset":10976,"ancestor_record_offset":10768,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10976,"parent_record_offset":10936,"ancestor_record_offset":10768,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":371,"record_offset":10936,"parent_record_offset":10892,"ancestor_record_offset":10768,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":11068,"parent_record_offset":10852,"ancestor_record_offset":10768,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":371,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":21264,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10892,"parent_record_offset":10852,"ancestor_record_offset":10768,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":371,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":10768,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":371,"record_offset":11196,"parent_record_offset":11160,"ancestor_record_offset":10768,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":371,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":10768,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":371,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":10768,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":371,"record_offset":11032,"parent_record_offset":10936,"ancestor_record_offset":10768,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":371,"record_offset":11280,"parent_record_offset":11256,"ancestor_record_offset":10768,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8CE
// 首条位置: {"module":1226,"inlinee":59598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5420
// 首条位置: {"module":371,"inlinee":21536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":371,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":21936,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::create_wall
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x541F
// 首条位置: {"module":371,"inlinee":21535}
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
// 完整结构及原始类型记录: construct_walls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: construct_walls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: construct_walls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls
// 信息 ID: 05ddab696dae060997f4ddef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7440bee7bb657de7199723da: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x1289f -> 0x4e9970 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0xbfc36 -> 0x4e9970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$1
// 信息 ID: e29bd5966c0688385a1cdff3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=8900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$3
// 信息 ID: 2223f6c0d6a083e6667d72b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=19656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$4
// 信息 ID: 80c89563c8c061097c8c7d3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=19760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$5
// 信息 ID: 904bf28cbfae86f112934f24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=10768; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=21264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$5::closure$0
// 信息 ID: 39a0db631a01aaf4965de65a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$5::closure_env$0>,alloc::alloc::Global>; module=1226, inline_record=18180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$6
// 信息 ID: 6ffc2f6141dff0f99d0da141
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=21936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::create_wall
// 信息 ID: 9eb2f39a9e94f2bda16d3e03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=22892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
