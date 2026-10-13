// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_gates/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 680b4ce04dd0f13c987407ef716918bc155a6d52c3b11a8aa64d0b09bb330c4a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1241, 1263, 1264, 2715
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1241,"record_offset":260,"rva":13371616,"va":5382080736}
// 编译实例 RVAs: 0xcc08e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13362703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8f11934ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382083370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12278409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e877a544ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382091956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9331541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8ab9c71ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382081232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=43
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9332286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8c29971ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382081993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140596a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7522848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8e0358dff","size":5,"soft_fail":false,"target_va":5374569056,"va":5382091899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6216387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e83d25a1ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5382097822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406df9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6183907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e81da4a1ff","size":5,"soft_fail":false,"target_va":5375916480,"va":5382100382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ecd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6126723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e87d83a2ff","size":5,"soft_fail":false,"target_va":5375970592,"va":5382097310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5148017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e88f72b1ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5382087916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5138823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e87996b1ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382082786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-185057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e81f2dfdff","size":5,"soft_fail":false,"target_va":5381913760,"va":5382098812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-186604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e81427fdff","size":5,"soft_fail":false,"target_va":5381914320,"va":5382100919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-167104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e84073fdff","size":5,"soft_fail":false,"target_va":5381918976,"va":5382086075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8e7b8feff","size":5,"soft_fail":false,"target_va":5382000496,"va":5382084228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb3eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e85227ffff","size":5,"soft_fail":false,"target_va":5382028976,"va":5382084441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-60905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e81712ffff","size":5,"soft_fail":false,"target_va":5382031392,"va":5382092292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e85835ffff","size":5,"soft_fail":false,"target_va":5382032368,"va":5382084243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8e240ffff","size":5,"soft_fail":false,"target_va":5382033008,"va":5382081929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t184386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e842d00200","size":5,"soft_fail":false,"target_va":5382265392,"va":5382081001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cedab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t175963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e85baf0200","size":5,"soft_fail":false,"target_va":5382265520,"va":5382089552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177d0d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11249162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e80aa6ab00","size":5,"soft_fail":false,"target_va":5393338576,"va":5382089409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177d260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11247411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8339fab00","size":5,"soft_fail":false,"target_va":5393338976,"va":5382091560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177db40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11249829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8a5a8ab00","size":5,"soft_fail":false,"target_va":5393341248,"va":5382091414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21511466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e82a3d4801","size":5,"soft_fail":false,"target_va":5403593360,"va":5382081889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21508084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8f42f4801","size":5,"soft_fail":false,"target_va":5403593392,"va":5382085303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21511426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8023d4801","size":5,"soft_fail":false,"target_va":5403594288,"va":5382082857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t20881501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e85da03e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382096638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20833499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8dbe43d01","size":5,"soft_fail":false,"target_va":5402918528,"va":5382085024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-10775528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e818945bff","size":5,"soft_fail":false,"target_va":5371306160,"va":5382081683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4569533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e84346baff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382096648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8aff1ffff","size":5,"soft_fail":false,"target_va":5382079824,"va":5382083484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20883076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e884a63e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382095799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t20958595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e883cd3f01","size":5,"soft_fail":false,"target_va":5403054560,"va":5382095960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20883549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e85da83e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382095598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=33
//     evidence={"assembly":"\tcallq\t20898173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e87de13e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382081166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20883377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8b1a73e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382095562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20883190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8f6a63e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382096933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20883187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8f3a63e01","size":5,"soft_fail":false,"target_va":5402980304,"va":5382097112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7525758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8822a8dff","size":5,"soft_fail":false,"target_va":5374566080,"va":5382091833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6399457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e81f5a9eff","size":5,"soft_fail":false,"target_va":5375692416,"va":5382091868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMesh::new_with_registry
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t153198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e86e560200","size":5,"soft_fail":false,"target_va":5382245120,"va":5382091917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-212712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e818c1fcff","size":5,"soft_fail":false,"target_va":5381870304,"va":5382083011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::mesh_from_pts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6951243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e8b5ee95ff","size":5,"soft_fail":false,"target_va":5375140096,"va":5382091334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t13198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e88e330000","size":5,"soft_fail":false,"target_va":5382102272,"va":5382089069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t103275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e86b930100","size":5,"soft_fail":false,"target_va":5382189152,"va":5382085872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5140426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382080736,"raw_hex":"e83690b1ff","size":5,"soft_fail":false,"target_va":5376942944,"va":5382083365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":16924,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":572,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":6620,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":5360,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1241,"record_offset":32848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":5848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":17076,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1241,"record_offset":17500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":27724,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":27688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":4124,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":17284,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":5456,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":31192,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1241,"record_offset":27660,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5572,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5792,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5964,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":30944,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":32760,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":31928,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":30828,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":31556,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":6140,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":5820,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":26416,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59805,"source_line":301,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_arch_heights
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":4344,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59792,"source_line":505,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_bottom_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":600,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59788,"source_line":448,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":17620,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59798,"source_line":335,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh_gap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":21636,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59801,"source_line":397,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":17592,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59797,"source_line":292,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::offset_door_bottom_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":16244,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59795,"source_line":493,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_brick_steps
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":6744,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59794,"source_line":528,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_ladder
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":6168,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59793,"source_line":610,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":29240,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":5332,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":16896,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":31304,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1904,"parent_record_offset":1808,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":3572,"parent_record_offset":3472,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":260,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":260,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":1776,"parent_record_offset":1744,"ancestor_record_offset":260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":2024,"parent_record_offset":1776,"ancestor_record_offset":260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":260,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":260,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":260,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":10908,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":28336,"parent_record_offset":28304,"ancestor_record_offset":260,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9380,"parent_record_offset":9352,"ancestor_record_offset":260,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9556,"parent_record_offset":9380,"ancestor_record_offset":260,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":7028,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2396,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9432,"parent_record_offset":9380,"ancestor_record_offset":260,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9352,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9084,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":1712,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2884,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2068,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":11664,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":8788,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":28792,"parent_record_offset":28760,"ancestor_record_offset":260,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1241,"record_offset":33064,"parent_record_offset":33040,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1241,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1241,"record_offset":9816,"parent_record_offset":9380,"ancestor_record_offset":260,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1241,"record_offset":28304,"parent_record_offset":27856,"ancestor_record_offset":260,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1241,"record_offset":27812,"parent_record_offset":27768,"ancestor_record_offset":260,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1241,"record_offset":27856,"parent_record_offset":27812,"ancestor_record_offset":260,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":1848,"parent_record_offset":1808,"ancestor_record_offset":260,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":260,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7252,"parent_record_offset":7212,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1241,"record_offset":6560,"parent_record_offset":6168,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":19584,"parent_record_offset":19140,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":19080,"parent_record_offset":18928,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":2340,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":2256,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1241,"record_offset":7728,"parent_record_offset":7584,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1241,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1241,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":260,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1241,"record_offset":2596,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":260,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":18268,"parent_record_offset":18224,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1416,"parent_record_offset":1196,"ancestor_record_offset":260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":28432,"parent_record_offset":28400,"ancestor_record_offset":260,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":2180,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":28464,"parent_record_offset":28432,"ancestor_record_offset":260,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":260,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":28656,"parent_record_offset":28576,"ancestor_record_offset":260,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":7212,"parent_record_offset":7172,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":7172,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1660,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":15618,"source_line":109,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1048,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":7452,"parent_record_offset":7336,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":260,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":28368,"parent_record_offset":28336,"ancestor_record_offset":260,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":28760,"parent_record_offset":28724,"ancestor_record_offset":260,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":22180,"parent_record_offset":21636,"ancestor_record_offset":260,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1112,"parent_record_offset":736,"ancestor_record_offset":260,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":19340,"parent_record_offset":19140,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":28680,"parent_record_offset":28656,"ancestor_record_offset":260,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":3536,"parent_record_offset":3504,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":5728,"parent_record_offset":5596,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":260,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":260,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$51::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1241,"record_offset":2284,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":34395,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1241,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":18420,"parent_record_offset":18268,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":19396,"parent_record_offset":19140,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1241,"record_offset":3312,"parent_record_offset":2968,"ancestor_record_offset":260,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":3420,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":956,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":2500,"parent_record_offset":2396,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1241,"record_offset":3384,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1241,"record_offset":7536,"parent_record_offset":7508,"ancestor_record_offset":260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":260,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":9112,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":2532,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":16656,"parent_record_offset":16244,"ancestor_record_offset":260,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":2152,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19212,"parent_record_offset":19140,"ancestor_record_offset":260,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19276,"parent_record_offset":19236,"ancestor_record_offset":260,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2560,"parent_record_offset":2532,"ancestor_record_offset":260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":10992,"parent_record_offset":10964,"ancestor_record_offset":260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":988,"parent_record_offset":912,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":260,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":8900,"parent_record_offset":8816,"ancestor_record_offset":260,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":912,"parent_record_offset":872,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":8816,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1241,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":260,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1241,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":260,"inlinee":21456,"source_line":40,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1241,"record_offset":11136,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":21588,"source_line":201,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5628,"parent_record_offset":5596,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16472,"parent_record_offset":16244,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16628,"parent_record_offset":16244,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":8960,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16416,"parent_record_offset":16244,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1584,"parent_record_offset":1496,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":30116,"parent_record_offset":30084,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":30084,"parent_record_offset":30060,"ancestor_record_offset":260,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":30060,"parent_record_offset":30036,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":6320,"parent_record_offset":6168,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":16740,"parent_record_offset":16244,"ancestor_record_offset":260,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24296,"parent_record_offset":22864,"ancestor_record_offset":260,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":6232,"parent_record_offset":6168,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1241,"record_offset":844,"parent_record_offset":736,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":1236,"parent_record_offset":1196,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1241,"record_offset":18496,"parent_record_offset":18468,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":28896,"parent_record_offset":28724,"ancestor_record_offset":260,"inlinee":59807,"source_line":204,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":260,"inlinee":59800,"source_line":347,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh_gap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":22864,"parent_record_offset":22800,"ancestor_record_offset":260,"inlinee":59803,"source_line":410,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":736,"parent_record_offset":600,"ancestor_record_offset":260,"inlinee":59789,"source_line":519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":260,"inlinee":59791,"source_line":524,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1241,"record_offset":7144,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":21523,"source_line":51,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":19140,"parent_record_offset":17620,"ancestor_record_offset":260,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":29272,"parent_record_offset":29240,"ancestor_record_offset":260,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7508,"parent_record_offset":7336,"ancestor_record_offset":260,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":4412,"parent_record_offset":4344,"ancestor_record_offset":260,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7936,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":24897,"source_line":669,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7336,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20392,"parent_record_offset":20348,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20508,"parent_record_offset":20348,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":7584,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":19432,"parent_record_offset":19140,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20348,"parent_record_offset":17620,"ancestor_record_offset":260,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":260,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1241,"record_offset":30036,"parent_record_offset":29648,"ancestor_record_offset":260,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE99F
// 首条位置: {"module":1241,"inlinee":59807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":29008,"parent_record_offset":28896,"ancestor_record_offset":28896,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":28928,"parent_record_offset":28896,"ancestor_record_offset":28896,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":28964,"parent_record_offset":28928,"ancestor_record_offset":28896,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":29036,"parent_record_offset":29008,"ancestor_record_offset":28896,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE995
// 首条位置: {"module":1241,"inlinee":59797}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE99D
// 首条位置: {"module":1241,"inlinee":59805}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":26552,"parent_record_offset":26416,"ancestor_record_offset":26416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":26468,"parent_record_offset":26416,"ancestor_record_offset":26416,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":26716,"parent_record_offset":26692,"ancestor_record_offset":26416,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":26692,"parent_record_offset":26600,"ancestor_record_offset":26416,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":26628,"parent_record_offset":26600,"ancestor_record_offset":26416,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":26600,"parent_record_offset":26576,"ancestor_record_offset":26416,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":26656,"parent_record_offset":26628,"ancestor_record_offset":26416,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":26576,"parent_record_offset":26552,"ancestor_record_offset":26416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":26516,"parent_record_offset":26492,"ancestor_record_offset":26416,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":26492,"parent_record_offset":26468,"ancestor_record_offset":26416,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
// 记录形式: compiled_procedure; instances=1
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1241,"record_offset":35712,"rva":13393152,"va":5382102272}
// 编译实例 RVAs: 0xcc5d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=24
//     evidence={"assembly":"\tcallq\t-10796167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382102272,"raw_hex":"e879435bff","size":5,"soft_fail":false,"target_va":5371306160,"va":5382102322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":35872,"parent_record_offset":35712,"ancestor_record_offset":35712,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":36080,"parent_record_offset":35712,"ancestor_record_offset":35712,"inlinee":23355,"source_line":112,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":36000,"parent_record_offset":35936,"ancestor_record_offset":35712,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":35936,"parent_record_offset":35872,"ancestor_record_offset":35712,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":36040,"parent_record_offset":35872,"ancestor_record_offset":35712,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE996
// 首条位置: {"module":1241,"inlinee":59798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":17828,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":17924,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":18008,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":18164,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":17952,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":21540,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":21484,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1241,"record_offset":18192,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":18660,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":19140,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":18928,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20348,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":19460,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":17620,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":17620,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":18376,"parent_record_offset":18340,"ancestor_record_offset":17620,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":19584,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":19080,"parent_record_offset":18928,"ancestor_record_offset":17620,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1241,"record_offset":19108,"parent_record_offset":18928,"ancestor_record_offset":17620,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":18268,"parent_record_offset":18224,"ancestor_record_offset":17620,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":18340,"parent_record_offset":18304,"ancestor_record_offset":17620,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":18304,"parent_record_offset":18268,"ancestor_record_offset":17620,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":19340,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":17620,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":18548,"parent_record_offset":18496,"ancestor_record_offset":17620,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":18604,"parent_record_offset":18496,"ancestor_record_offset":17620,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":17976,"parent_record_offset":17952,"ancestor_record_offset":17620,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1241,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":17620,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":18992,"parent_record_offset":18968,"ancestor_record_offset":17620,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":18968,"parent_record_offset":18928,"ancestor_record_offset":17620,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":18420,"parent_record_offset":18268,"ancestor_record_offset":17620,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":19396,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":17620,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19212,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":19276,"parent_record_offset":19236,"ancestor_record_offset":17620,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":17856,"parent_record_offset":17828,"ancestor_record_offset":17620,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":17884,"parent_record_offset":17856,"ancestor_record_offset":17620,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":18088,"parent_record_offset":18056,"ancestor_record_offset":17620,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":17620,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":17620,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":21388,"parent_record_offset":20996,"ancestor_record_offset":17620,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":21208,"parent_record_offset":20996,"ancestor_record_offset":17620,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":21300,"parent_record_offset":20996,"ancestor_record_offset":17620,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":18224,"parent_record_offset":18192,"ancestor_record_offset":17620,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1241,"record_offset":18496,"parent_record_offset":18468,"ancestor_record_offset":17620,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":18120,"parent_record_offset":18088,"ancestor_record_offset":17620,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":18468,"parent_record_offset":18444,"ancestor_record_offset":17620,"inlinee":59800,"source_line":347,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20392,"parent_record_offset":20348,"ancestor_record_offset":17620,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":20508,"parent_record_offset":20348,"ancestor_record_offset":17620,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":19432,"parent_record_offset":19140,"ancestor_record_offset":17620,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":17620,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE998
// 首条位置: {"module":1241,"inlinee":59800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1241,"record_offset":18496,"parent_record_offset":18468,"ancestor_record_offset":18468,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":18548,"parent_record_offset":18496,"ancestor_record_offset":18468,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":18604,"parent_record_offset":18496,"ancestor_record_offset":18468,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":18520,"parent_record_offset":18496,"ancestor_record_offset":18468,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE999
// 首条位置: {"module":1241,"inlinee":59801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":21816,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":22180,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":21912,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":21996,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":22152,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":21940,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":26264,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":26152,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1241,"record_offset":22236,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":22504,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":24612,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":24404,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":25580,"parent_record_offset":21636,"ancestor_record_offset":21636,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":23496,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":21636,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":21636,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":23620,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":23100,"parent_record_offset":22952,"ancestor_record_offset":21636,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1241,"record_offset":23128,"parent_record_offset":22952,"ancestor_record_offset":21636,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1241,"record_offset":22464,"parent_record_offset":22440,"ancestor_record_offset":21636,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1241,"record_offset":22320,"parent_record_offset":22296,"ancestor_record_offset":21636,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21636,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":22296,"parent_record_offset":22260,"ancestor_record_offset":21636,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":22440,"parent_record_offset":22260,"ancestor_record_offset":21636,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":23376,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":21636,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24056,"parent_record_offset":24028,"ancestor_record_offset":21636,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":21636,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":21636,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":21964,"parent_record_offset":21940,"ancestor_record_offset":21636,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1241,"record_offset":22800,"parent_record_offset":22736,"ancestor_record_offset":21636,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":21636,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":22988,"parent_record_offset":22952,"ancestor_record_offset":21636,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":22408,"parent_record_offset":22296,"ancestor_record_offset":21636,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":23432,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":22700,"parent_record_offset":22668,"ancestor_record_offset":21636,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23272,"parent_record_offset":23248,"ancestor_record_offset":21636,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23248,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23312,"parent_record_offset":23272,"ancestor_record_offset":21636,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":21844,"parent_record_offset":21816,"ancestor_record_offset":21636,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":21872,"parent_record_offset":21844,"ancestor_record_offset":21636,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":22372,"parent_record_offset":22348,"ancestor_record_offset":21636,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":22348,"parent_record_offset":22320,"ancestor_record_offset":21636,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":22076,"parent_record_offset":22044,"ancestor_record_offset":21636,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":22044,"parent_record_offset":22020,"ancestor_record_offset":21636,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":21636,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24116,"parent_record_offset":23656,"ancestor_record_offset":21636,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":23940,"parent_record_offset":23656,"ancestor_record_offset":21636,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24296,"parent_record_offset":22864,"ancestor_record_offset":21636,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24028,"parent_record_offset":23656,"ancestor_record_offset":21636,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":21636,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":22108,"parent_record_offset":22076,"ancestor_record_offset":21636,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh_gap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":22864,"parent_record_offset":22800,"ancestor_record_offset":21636,"inlinee":59803,"source_line":410,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23160,"parent_record_offset":22864,"ancestor_record_offset":21636,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23716,"parent_record_offset":23656,"ancestor_record_offset":21636,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23840,"parent_record_offset":23656,"ancestor_record_offset":21636,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":22952,"parent_record_offset":22864,"ancestor_record_offset":21636,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":23468,"parent_record_offset":23160,"ancestor_record_offset":21636,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23656,"parent_record_offset":22864,"ancestor_record_offset":21636,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23336,"parent_record_offset":23312,"ancestor_record_offset":21636,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 410
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE99B
// 首条位置: {"module":1241,"inlinee":59803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24296,"parent_record_offset":22864,"ancestor_record_offset":22864,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24208,"parent_record_offset":22864,"ancestor_record_offset":22864,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23160,"parent_record_offset":22864,"ancestor_record_offset":22864,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":22952,"parent_record_offset":22864,"ancestor_record_offset":22864,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23656,"parent_record_offset":22864,"ancestor_record_offset":22864,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":23496,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":22864,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":22864,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":23620,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":23100,"parent_record_offset":22952,"ancestor_record_offset":22864,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1241,"record_offset":23128,"parent_record_offset":22952,"ancestor_record_offset":22864,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":23376,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":22864,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24056,"parent_record_offset":24028,"ancestor_record_offset":22864,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":22864,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":24324,"parent_record_offset":24296,"ancestor_record_offset":22864,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":23968,"parent_record_offset":23940,"ancestor_record_offset":22864,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":22864,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":22988,"parent_record_offset":22952,"ancestor_record_offset":22864,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1241,"record_offset":23432,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23272,"parent_record_offset":23248,"ancestor_record_offset":22864,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23248,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":23312,"parent_record_offset":23272,"ancestor_record_offset":22864,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":23524,"parent_record_offset":23496,"ancestor_record_offset":22864,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":23552,"parent_record_offset":23524,"ancestor_record_offset":22864,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24116,"parent_record_offset":23656,"ancestor_record_offset":22864,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":23940,"parent_record_offset":23656,"ancestor_record_offset":22864,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":24028,"parent_record_offset":23656,"ancestor_record_offset":22864,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23716,"parent_record_offset":23656,"ancestor_record_offset":22864,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23840,"parent_record_offset":23656,"ancestor_record_offset":22864,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":23468,"parent_record_offset":23160,"ancestor_record_offset":22864,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":23336,"parent_record_offset":23312,"ancestor_record_offset":22864,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_bottom_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 448
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE98C
// 首条位置: {"module":1241,"inlinee":59788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2396,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":1712,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2884,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2068,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":2340,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":2256,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1241,"record_offset":2596,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":2180,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$51::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1241,"record_offset":2284,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":34395,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":3420,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1241,"record_offset":3384,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":2532,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":2152,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":2688,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":736,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":59789,"source_line":519,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":600,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":600,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":600,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":1904,"parent_record_offset":1808,"ancestor_record_offset":600,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":3572,"parent_record_offset":3472,"ancestor_record_offset":600,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":600,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":2468,"parent_record_offset":2436,"ancestor_record_offset":600,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":600,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":1776,"parent_record_offset":1744,"ancestor_record_offset":600,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":2024,"parent_record_offset":1776,"ancestor_record_offset":600,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":600,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":600,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":600,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1241,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":600,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":2436,"parent_record_offset":2396,"ancestor_record_offset":600,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":600,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":1848,"parent_record_offset":1808,"ancestor_record_offset":600,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":600,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1241,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":600,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1241,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":600,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":600,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1416,"parent_record_offset":1196,"ancestor_record_offset":600,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":600,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":600,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1660,"parent_record_offset":1416,"ancestor_record_offset":600,"inlinee":15618,"source_line":109,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1048,"parent_record_offset":872,"ancestor_record_offset":600,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":600,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1112,"parent_record_offset":736,"ancestor_record_offset":600,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":3536,"parent_record_offset":3504,"ancestor_record_offset":600,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":600,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":600,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1241,"record_offset":3312,"parent_record_offset":2968,"ancestor_record_offset":600,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":600,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":600,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":600,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":956,"parent_record_offset":912,"ancestor_record_offset":600,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":600,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":2500,"parent_record_offset":2396,"ancestor_record_offset":600,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":600,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":600,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2560,"parent_record_offset":2532,"ancestor_record_offset":600,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":600,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":600,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":988,"parent_record_offset":912,"ancestor_record_offset":600,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":912,"parent_record_offset":872,"ancestor_record_offset":600,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":600,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1584,"parent_record_offset":1496,"ancestor_record_offset":600,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":600,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1241,"record_offset":844,"parent_record_offset":736,"ancestor_record_offset":600,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":1236,"parent_record_offset":1196,"ancestor_record_offset":600,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":600,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":600,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":600,"inlinee":59791,"source_line":524,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::offset_door_bottom_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE993
// 首条位置: {"module":1241,"inlinee":59795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":16292,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":16656,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":16388,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":16828,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16472,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16628,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16416,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":16740,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":16768,"parent_record_offset":16740,"ancestor_record_offset":16244,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":16440,"parent_record_offset":16416,"ancestor_record_offset":16244,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":16244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":16860,"parent_record_offset":16828,"ancestor_record_offset":16244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":16320,"parent_record_offset":16292,"ancestor_record_offset":16244,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":16348,"parent_record_offset":16320,"ancestor_record_offset":16244,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":16244,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16552,"parent_record_offset":16520,"ancestor_record_offset":16244,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":16244,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":16496,"parent_record_offset":16472,"ancestor_record_offset":16244,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":16584,"parent_record_offset":16552,"ancestor_record_offset":16244,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_arch_heights
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 505
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE990
// 首条位置: {"module":1241,"inlinee":59792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":4788,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":4440,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":5000,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1241,"record_offset":4916,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":5096,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":4412,"parent_record_offset":4344,"ancestor_record_offset":4344,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":4344,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":4344,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":4344,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":4624,"parent_record_offset":4532,"ancestor_record_offset":4344,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":5252,"parent_record_offset":5148,"ancestor_record_offset":4344,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":5180,"parent_record_offset":5148,"ancestor_record_offset":4344,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":4344,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":4532,"parent_record_offset":4504,"ancestor_record_offset":4344,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":4504,"parent_record_offset":4472,"ancestor_record_offset":4344,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":4744,"parent_record_offset":4504,"ancestor_record_offset":4344,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":5148,"parent_record_offset":5120,"ancestor_record_offset":4344,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":4828,"parent_record_offset":4788,"ancestor_record_offset":4344,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":4472,"parent_record_offset":4440,"ancestor_record_offset":4344,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":4568,"parent_record_offset":4532,"ancestor_record_offset":4344,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1241,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":4344,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4344,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1241,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4344,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":4344,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":4344,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":4700,"parent_record_offset":4672,"ancestor_record_offset":4344,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":4884,"parent_record_offset":4788,"ancestor_record_offset":4344,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":4344,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":4344,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE98D
// 首条位置: {"module":1241,"inlinee":59789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1112,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1241,"record_offset":844,"parent_record_offset":736,"ancestor_record_offset":736,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1241,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":736,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1168,"parent_record_offset":1140,"ancestor_record_offset":736,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1241,"record_offset":1416,"parent_record_offset":1196,"ancestor_record_offset":736,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":736,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1241,"record_offset":1660,"parent_record_offset":1416,"ancestor_record_offset":736,"inlinee":15618,"source_line":109,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1048,"parent_record_offset":872,"ancestor_record_offset":736,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":736,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":736,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":736,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":736,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":956,"parent_record_offset":912,"ancestor_record_offset":736,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":736,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":988,"parent_record_offset":912,"ancestor_record_offset":736,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":912,"parent_record_offset":872,"ancestor_record_offset":736,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":736,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1584,"parent_record_offset":1496,"ancestor_record_offset":736,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":736,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":1236,"parent_record_offset":1196,"ancestor_record_offset":736,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1241,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":736,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":736,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1241,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":736,"inlinee":59791,"source_line":524,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE98F
// 首条位置: {"module":1241,"inlinee":59791}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":1528,"parent_record_offset":1496,"ancestor_record_offset":1444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1584,"parent_record_offset":1496,"ancestor_record_offset":1444,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1444,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1241,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":1444,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_brick_steps
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE992
// 首条位置: {"module":1241,"inlinee":59794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":10908,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":7028,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9352,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9084,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":10936,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":11664,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":8788,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1241,"record_offset":7308,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":7172,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":8320,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":9112,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":11776,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":8816,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1241,"record_offset":11136,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":21588,"source_line":201,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1241,"record_offset":8960,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1241,"record_offset":7144,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":21523,"source_line":51,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":9056,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7936,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":24897,"source_line":669,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7336,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1241,"record_offset":7584,"parent_record_offset":6744,"ancestor_record_offset":6744,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":6744,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1241,"record_offset":8240,"parent_record_offset":8136,"ancestor_record_offset":6744,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":6744,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":6744,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":6744,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9592,"parent_record_offset":9556,"ancestor_record_offset":6744,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":6744,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1241,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":6744,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9380,"parent_record_offset":9352,"ancestor_record_offset":6744,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9556,"parent_record_offset":9380,"ancestor_record_offset":6744,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":7984,"parent_record_offset":7936,"ancestor_record_offset":6744,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":9432,"parent_record_offset":9380,"ancestor_record_offset":6744,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1241,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":6744,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1241,"record_offset":9816,"parent_record_offset":9380,"ancestor_record_offset":6744,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":6744,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":6744,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1241,"record_offset":7252,"parent_record_offset":7212,"ancestor_record_offset":6744,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1241,"record_offset":7728,"parent_record_offset":7584,"ancestor_record_offset":6744,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":7212,"parent_record_offset":7172,"ancestor_record_offset":6744,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1241,"record_offset":9624,"parent_record_offset":9380,"ancestor_record_offset":6744,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1241,"record_offset":7452,"parent_record_offset":7336,"ancestor_record_offset":6744,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1241,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":6744,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":6744,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":6744,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1241,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":6744,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":8084,"parent_record_offset":7936,"ancestor_record_offset":6744,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1241,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":6744,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":6744,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1241,"record_offset":8868,"parent_record_offset":8816,"ancestor_record_offset":6744,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1241,"record_offset":9728,"parent_record_offset":9380,"ancestor_record_offset":6744,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1241,"record_offset":7536,"parent_record_offset":7508,"ancestor_record_offset":6744,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1241,"record_offset":7368,"parent_record_offset":7336,"ancestor_record_offset":6744,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":10964,"parent_record_offset":10936,"ancestor_record_offset":6744,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1241,"record_offset":10992,"parent_record_offset":10964,"ancestor_record_offset":6744,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1241,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":6744,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":11840,"parent_record_offset":11776,"ancestor_record_offset":6744,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":8900,"parent_record_offset":8816,"ancestor_record_offset":6744,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1241,"record_offset":9172,"parent_record_offset":9136,"ancestor_record_offset":6744,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1241,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":6744,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1241,"record_offset":7508,"parent_record_offset":7336,"ancestor_record_offset":6744,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_ladder
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE991
// 首条位置: {"module":1241,"inlinee":59793}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1241,"record_offset":6560,"parent_record_offset":6168,"ancestor_record_offset":6168,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":6320,"parent_record_offset":6168,"ancestor_record_offset":6168,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":6440,"parent_record_offset":6168,"ancestor_record_offset":6168,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1241,"record_offset":6232,"parent_record_offset":6168,"ancestor_record_offset":6168,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":6168,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":6168,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1241,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":6168,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01241.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1241,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":6168,"inlinee":21456,"source_line":40,"archive":"_pdb/A/modules/01241.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_gates::DoorMeshInProgress
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_gates::DoorMeshInProgress>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_gates::DoorMeshInProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
// 信息 ID: ab2d94b49f866a3891402b0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc296d -> 0xcc5d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc3174 -> 0xcc5d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
// 信息 ID: b70d6e54424b29eb8ebd407f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=26416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 信息 ID: 39210e8ade9bc706dad2833b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x4e53ec -> 0xcc08e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 信息 ID: 1e8fa9bce8ed4b715fc0a87b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::construct_wooden_gate_doors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=28896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_arch_heights
// 信息 ID: 94536545d723ac403ca6521c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=4344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_bottom_curve
// 信息 ID: 64a0d307bde46a1d927476f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh
// 信息 ID: 68238ee280f44304cf54080e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=17620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh::closure$0
// 信息 ID: 132bc618a4bd48ad8dd2bbfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=18468; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=18768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap
// 信息 ID: 602d66fb43ad1bb9a00f6e27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=21636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 信息 ID: f9f9659d3873aa128d0effbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh_gap", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<glam::f32::vec3::Vec3,f32>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32> >,core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<f32> > >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=22864; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<f32> > >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; module=2715, inline_record=496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::impl$1::default
// 信息 ID: 0733816dc2a63492e3605fc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=17592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::offset_door_bottom_curve
// 信息 ID: 2de22f3b6f4046c84e649992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=16244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_brick_steps
// 信息 ID: ed47d49f5a1ff12865bd74d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=6744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_ladder
// 信息 ID: cad0a790714d2f4e07866a51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=6168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length
// 信息 ID: f0d7923159b838760235f575
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length::closure$0
// 信息 ID: 5945237da5cbe3fe8abba654
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::segment_length", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=1444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
