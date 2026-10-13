// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_prefabs.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fb0d0c042b411fbd48868330519f7bb5d2edc979b099eb6dda3ae89450cd3652
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_prefabs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 489, 1308, 1321, 1451
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"PDB_registry_path_only","version":"1.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1451,"record_offset":13356,"rva":17893040,"va":5386602160}
// 编译实例 RVAs: 0x11106b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16991771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8e5b9fcfe","size":5,"soft_fail":false,"target_va":5369614224,"va":5386605990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=7
//     target_aliases=4 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16971000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8080bfdfe","size":5,"soft_fail":false,"target_va":5369632448,"va":5386603443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13977314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e81eb92aff","size":5,"soft_fail":false,"target_va":5372625024,"va":5386602333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13852902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e81a9f2cff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386602593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=28
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13855674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e846942cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386605381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b5470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11910306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e85e434aff","size":5,"soft_fail":false,"target_va":5374694512,"va":5386604813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11872813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8d3d54aff","size":5,"soft_fail":false,"target_va":5374736096,"va":5386608904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c0d60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11868095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e841e84aff","size":5,"soft_fail":false,"target_va":5374741856,"va":5386609946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11431357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8439251ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5386602408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a7a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10915773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8437059ff","size":5,"soft_fail":false,"target_va":5375687296,"va":5386603064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a91b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10912392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8787d59ff","size":5,"soft_fail":false,"target_va":5375693232,"va":5386605619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a97f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10910994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8ee8259ff","size":5,"soft_fail":false,"target_va":5375694832,"va":5386605821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10719106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e87e705cff","size":5,"soft_fail":false,"target_va":5375903584,"va":5386622685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e0720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10701554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e80eb55cff","size":5,"soft_fail":false,"target_va":5375919904,"va":5386621453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e0770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10699279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8f1bd5cff","size":5,"soft_fail":false,"target_va":5375919984,"va":5386619258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10682786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e85efe5cff","size":5,"soft_fail":false,"target_va":5375936112,"va":5386618893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10672704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8c0255dff","size":5,"soft_fail":false,"target_va":5375947344,"va":5386620043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1070
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10626163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e88ddb5dff","size":5,"soft_fail":false,"target_va":5375987824,"va":5386613982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9165815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8092474ff","size":5,"soft_fail":false,"target_va":5377436496,"va":5386602306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3797457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e82f0ec6ff","size":5,"soft_fail":false,"target_va":5382810688,"va":5386608140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140def0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3294371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e85dbbcdff","size":5,"soft_fail":false,"target_va":5383319728,"va":5386614094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140def5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3293012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8acc0cdff","size":5,"soft_fail":false,"target_va":5383321056,"va":5386614063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df07b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3283390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e842e6cdff","size":5,"soft_fail":false,"target_va":5383325616,"va":5386609001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df08d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3288251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e845d3cdff","size":5,"soft_fail":false,"target_va":5383325904,"va":5386614150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1810
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3284322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e89ee2cdff","size":5,"soft_fail":false,"target_va":5383329808,"va":5386614125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df3520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3268777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8571fceff","size":5,"soft_fail":false,"target_va":5383337248,"va":5386606020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e929b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2617475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e87d0fd8ff","size":5,"soft_fail":false,"target_va":5383989680,"va":5386607150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e930f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2612309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8ab23d8ff","size":5,"soft_fail":false,"target_va":5383991536,"va":5386603840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea1050
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2562856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8d8e4d8ff","size":5,"soft_fail":false,"target_va":5384048720,"va":5386611571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f2f150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1973852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8a4e1e1ff","size":5,"soft_fail":false,"target_va":5384630608,"va":5386604455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16339908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8c453f900","size":5,"soft_fail":false,"target_va":5402958656,"va":5386618743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420dc1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16557563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8fba5fc00","size":5,"soft_fail":false,"target_va":5403165104,"va":5386607536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16989232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8303c0301","size":5,"soft_fail":false,"target_va":5403593344,"va":5386604107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t16375528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8e8def900","size":5,"soft_fail":false,"target_va":5402978144,"va":5386602611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9074801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e88f8775ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386604684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-15298585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8e78f16ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5386607012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-9088542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8e25175ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5386615657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9255513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8a7c572ff","size":5,"soft_fail":false,"target_va":5377364336,"va":5386619844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8214704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8b0587d00","size":5,"soft_fail":false,"target_va":5394819472,"va":5386604763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::prefab::scan::scan_asset_tree
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8218033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8b1657d00","size":5,"soft_fail":false,"target_va":5394820464,"va":5386602426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16360737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e821a5f900","size":5,"soft_fail":false,"target_va":5402978912,"va":5386618170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16365064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e808b6f900","size":5,"soft_fail":false,"target_va":5402979344,"va":5386614275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=37
//     evidence={"assembly":"\tcallq\t16364506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8dab3f900","size":5,"soft_fail":false,"target_va":5402978944,"va":5386614433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<std::collections::hash::map::Entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e851bcffff","size":5,"soft_fail":false,"target_va":5386601744,"va":5386619066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16365384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e848b7f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386614739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::aabb_bounds
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2044295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e879cee0ff","size":5,"soft_fail":false,"target_va":5384568240,"va":5386612530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::prefab::PrefabLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12286382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8528644ff","size":5,"soft_fail":false,"target_va":5374327488,"va":5386613865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-9236092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8841173ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5386602311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::scaled
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2191441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386602160,"raw_hex":"e8af8fdeff","size":5,"soft_fail":false,"target_va":5384421168,"va":5386612604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1451,"record_offset":13496,"parent_record_offset":13356,"ancestor_record_offset":13356,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::load_prefabs_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":13556,"parent_record_offset":13356,"ancestor_record_offset":13356,"inlinee":66967,"source_line":68,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":13356,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":13356,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21512,"parent_record_offset":21484,"ancestor_record_offset":13356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21072,"parent_record_offset":21036,"ancestor_record_offset":13356,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14860,"parent_record_offset":14768,"ancestor_record_offset":13356,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21484,"parent_record_offset":21420,"ancestor_record_offset":13356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1451,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":13356,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":13356,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":13356,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21452,"parent_record_offset":21420,"ancestor_record_offset":13356,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":30956,"parent_record_offset":30932,"ancestor_record_offset":13356,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14768,"parent_record_offset":14740,"ancestor_record_offset":13356,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14740,"parent_record_offset":14712,"ancestor_record_offset":13356,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14980,"parent_record_offset":14740,"ancestor_record_offset":13356,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":13356,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1451,"record_offset":19688,"parent_record_offset":19652,"ancestor_record_offset":13356,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46268,"parent_record_offset":46232,"ancestor_record_offset":13356,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":21000,"parent_record_offset":20480,"ancestor_record_offset":13356,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":13356,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":23532,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46232,"parent_record_offset":46208,"ancestor_record_offset":13356,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30932,"parent_record_offset":30888,"ancestor_record_offset":13356,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41512,"parent_record_offset":41164,"ancestor_record_offset":13356,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":16744,"parent_record_offset":16720,"ancestor_record_offset":13356,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30888,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":14684,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":14712,"parent_record_offset":14684,"ancestor_record_offset":13356,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41132,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35928,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":52208,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35816,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":55072,"parent_record_offset":55048,"ancestor_record_offset":13356,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":16720,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52408,"parent_record_offset":52368,"ancestor_record_offset":13356,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52368,"parent_record_offset":52340,"ancestor_record_offset":13356,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52632,"parent_record_offset":52608,"ancestor_record_offset":13356,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52264,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52480,"parent_record_offset":52456,"ancestor_record_offset":13356,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1451,"record_offset":19724,"parent_record_offset":19688,"ancestor_record_offset":13356,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1451,"record_offset":41164,"parent_record_offset":41132,"ancestor_record_offset":13356,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=15
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":17808,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":18596,"parent_record_offset":18564,"ancestor_record_offset":13356,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":19920,"parent_record_offset":19884,"ancestor_record_offset":13356,"inlinee":62240,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1451,"record_offset":20812,"parent_record_offset":20480,"ancestor_record_offset":13356,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1451,"record_offset":13524,"parent_record_offset":13496,"ancestor_record_offset":13356,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":14804,"parent_record_offset":14768,"ancestor_record_offset":13356,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":13356,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":13356,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":13356,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14300,"parent_record_offset":14272,"ancestor_record_offset":13356,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":13356,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14328,"parent_record_offset":14272,"ancestor_record_offset":13356,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":41620,"parent_record_offset":41588,"ancestor_record_offset":13356,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":19616,"parent_record_offset":19580,"ancestor_record_offset":13356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1451,"record_offset":13896,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":15756,"parent_record_offset":15728,"ancestor_record_offset":13356,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":13356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":15816,"parent_record_offset":15784,"ancestor_record_offset":13356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17848,"parent_record_offset":17808,"ancestor_record_offset":13356,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17876,"parent_record_offset":17808,"ancestor_record_offset":13356,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1451,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":13356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1451,"record_offset":31108,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1451,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":13356,"inlinee":21077,"source_line":25,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1451,"record_offset":15964,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":13356,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":13356,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41588,"parent_record_offset":41556,"ancestor_record_offset":13356,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41556,"parent_record_offset":41512,"ancestor_record_offset":13356,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1451,"record_offset":52456,"parent_record_offset":52432,"ancestor_record_offset":13356,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1451,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":13356,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":15900,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":21075,"source_line":883,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":15988,"parent_record_offset":15964,"ancestor_record_offset":13356,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":16680,"parent_record_offset":16012,"ancestor_record_offset":13356,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":16012,"parent_record_offset":15988,"ancestor_record_offset":13356,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1451,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":13356,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":26112,"parent_record_offset":26084,"ancestor_record_offset":13356,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":22396,"parent_record_offset":22364,"ancestor_record_offset":13356,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1451,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":13356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":31048,"parent_record_offset":31020,"ancestor_record_offset":13356,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":37120,"parent_record_offset":37092,"ancestor_record_offset":13356,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":13356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":50148,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":24484,"parent_record_offset":23560,"ancestor_record_offset":13356,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":13356,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":54100,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19856,"parent_record_offset":19652,"ancestor_record_offset":13356,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":13356,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":49972,"parent_record_offset":49948,"ancestor_record_offset":13356,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":18564,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":15700,"parent_record_offset":15668,"ancestor_record_offset":13356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":52528,"parent_record_offset":52504,"ancestor_record_offset":13356,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=171
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14652,"parent_record_offset":14612,"ancestor_record_offset":13356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":13356,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":13356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":13356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":22608,"parent_record_offset":22580,"ancestor_record_offset":13356,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":46300,"parent_record_offset":46268,"ancestor_record_offset":13356,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":52504,"parent_record_offset":52480,"ancestor_record_offset":13356,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":35892,"parent_record_offset":35844,"ancestor_record_offset":13356,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":13356,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":13356,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1451,"record_offset":14612,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19760,"parent_record_offset":19724,"ancestor_record_offset":13356,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19796,"parent_record_offset":19760,"ancestor_record_offset":13356,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35960,"parent_record_offset":35928,"ancestor_record_offset":13356,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35992,"parent_record_offset":35960,"ancestor_record_offset":13356,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":31184,"parent_record_offset":31156,"ancestor_record_offset":13356,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":35844,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1451,"record_offset":23416,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":13356,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":54508,"parent_record_offset":54472,"ancestor_record_offset":13356,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":13356,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46328,"parent_record_offset":46268,"ancestor_record_offset":13356,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46360,"parent_record_offset":46232,"ancestor_record_offset":13356,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1451,"record_offset":19652,"parent_record_offset":19616,"ancestor_record_offset":13356,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":23560,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":23397,"source_line":205,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":27880,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::try_from_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1451,"record_offset":49636,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":26131,"source_line":191,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$3::try_from_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1451,"record_offset":49748,"parent_record_offset":49720,"ancestor_record_offset":13356,"inlinee":26099,"source_line":192,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":18012,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":66968,"source_line":194,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":20480,"parent_record_offset":20428,"ancestor_record_offset":13356,"inlinee":66971,"source_line":211,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":44680,"parent_record_offset":44640,"ancestor_record_offset":13356,"inlinee":66976,"source_line":265,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":54124,"parent_record_offset":54100,"ancestor_record_offset":13356,"inlinee":66975,"source_line":123,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":50192,"parent_record_offset":50148,"ancestor_record_offset":13356,"inlinee":66973,"source_line":129,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":50992,"parent_record_offset":50948,"ancestor_record_offset":13356,"inlinee":66974,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::recalculate_canopy_quad_attribs
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":26524,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":66972,"source_line":297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":1451,"record_offset":49948,"parent_record_offset":49688,"ancestor_record_offset":13356,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34648,"parent_record_offset":34612,"ancestor_record_offset":13356,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34612,"parent_record_offset":34584,"ancestor_record_offset":13356,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34584,"parent_record_offset":34544,"ancestor_record_offset":13356,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50564,"parent_record_offset":50540,"ancestor_record_offset":13356,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34544,"parent_record_offset":34512,"ancestor_record_offset":13356,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34512,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":31020,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50540,"parent_record_offset":50516,"ancestor_record_offset":13356,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":37092,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1451,"record_offset":19580,"parent_record_offset":19544,"ancestor_record_offset":13356,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":13356,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18720,"parent_record_offset":18688,"ancestor_record_offset":13356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18752,"parent_record_offset":18720,"ancestor_record_offset":13356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":15520,"parent_record_offset":15492,"ancestor_record_offset":13356,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":13356,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":13356,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25676,"parent_record_offset":25644,"ancestor_record_offset":13356,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":32316,"parent_record_offset":31280,"ancestor_record_offset":13356,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31280,"parent_record_offset":31252,"ancestor_record_offset":13356,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":19544,"parent_record_offset":19292,"ancestor_record_offset":13356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":13356,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33928,"parent_record_offset":33900,"ancestor_record_offset":13356,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":13356,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18788,"parent_record_offset":18752,"ancestor_record_offset":13356,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19444,"parent_record_offset":19416,"ancestor_record_offset":13356,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19416,"parent_record_offset":19292,"ancestor_record_offset":13356,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19348,"parent_record_offset":19320,"ancestor_record_offset":13356,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16468,"parent_record_offset":16120,"ancestor_record_offset":13356,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22580,"parent_record_offset":21628,"ancestor_record_offset":13356,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":20204,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15668,"parent_record_offset":15576,"ancestor_record_offset":13356,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16120,"parent_record_offset":16088,"ancestor_record_offset":13356,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19320,"parent_record_offset":19292,"ancestor_record_offset":13356,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":13356,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18816,"parent_record_offset":18752,"ancestor_record_offset":13356,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":13356,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":13356,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":13356,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22468,"parent_record_offset":21628,"ancestor_record_offset":13356,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15640,"parent_record_offset":15576,"ancestor_record_offset":13356,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15612,"parent_record_offset":15576,"ancestor_record_offset":13356,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18880,"parent_record_offset":18848,"ancestor_record_offset":13356,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":13356,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21884,"parent_record_offset":21704,"ancestor_record_offset":13356,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22048,"parent_record_offset":21704,"ancestor_record_offset":13356,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22080,"parent_record_offset":22048,"ancestor_record_offset":13356,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":13356,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21672,"parent_record_offset":21628,"ancestor_record_offset":13356,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21760,"parent_record_offset":21704,"ancestor_record_offset":13356,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":13356,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22340,"parent_record_offset":21672,"ancestor_record_offset":13356,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=19
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16208,"parent_record_offset":16172,"ancestor_record_offset":13356,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16244,"parent_record_offset":16208,"ancestor_record_offset":13356,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16304,"parent_record_offset":16172,"ancestor_record_offset":13356,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16172,"parent_record_offset":16120,"ancestor_record_offset":13356,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18936,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":13356,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":13356,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19292,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":13356,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22440,"parent_record_offset":21628,"ancestor_record_offset":13356,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":18964,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=17
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":15728,"parent_record_offset":15668,"ancestor_record_offset":13356,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":19048,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":20112,"parent_record_offset":18880,"ancestor_record_offset":13356,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=20
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":15784,"parent_record_offset":15728,"ancestor_record_offset":13356,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":13356,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":13356,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31984,"parent_record_offset":31764,"ancestor_record_offset":13356,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1451,"record_offset":14004,"parent_record_offset":13980,"ancestor_record_offset":13356,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":44640,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":21450,"source_line":2741,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":32384,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":45000,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":30810,"source_line":2639,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20428,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":38940,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20372,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":18688,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15492,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":13356,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":25620,"parent_record_offset":18012,"ancestor_record_offset":13356,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":24168,"parent_record_offset":23560,"ancestor_record_offset":13356,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33496,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15460,"parent_record_offset":15028,"ancestor_record_offset":13356,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":22688,"parent_record_offset":20428,"ancestor_record_offset":13356,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":21580,"parent_record_offset":20428,"ancestor_record_offset":13356,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":16776,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":16040,"parent_record_offset":16012,"ancestor_record_offset":13356,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":13356,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":13356,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33872,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15028,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":27944,"parent_record_offset":26524,"ancestor_record_offset":13356,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":13356,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1451,"record_offset":13980,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":34684,"parent_record_offset":34648,"ancestor_record_offset":13356,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":13356,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":13356,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14568,"parent_record_offset":14544,"ancestor_record_offset":13356,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13356,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":13356,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":13356,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14272,"parent_record_offset":14124,"ancestor_record_offset":13356,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":13356,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":13356,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13356,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":13356,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20616,"parent_record_offset":20480,"ancestor_record_offset":13356,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":14472,"parent_record_offset":13556,"ancestor_record_offset":13356,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":13356,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1451,"record_offset":14204,"parent_record_offset":14176,"ancestor_record_offset":13356,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":13356,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":13356,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10597
// 首条位置: {"module":1451,"inlinee":66967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":49548,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":51616,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":14684,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":52208,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":16720,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52264,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":17808,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1451,"record_offset":51948,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1451,"record_offset":13896,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1451,"record_offset":15964,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":15900,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":21075,"source_line":883,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":50120,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":50148,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":54100,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":51760,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1451,"record_offset":14612,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::try_from_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1451,"record_offset":49636,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":26131,"source_line":191,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::load_prefab_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":18012,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":66968,"source_line":194,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15492,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":16776,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":17136,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15028,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1451,"record_offset":13980,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":14472,"parent_record_offset":13556,"ancestor_record_offset":13556,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":13556,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":13556,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21512,"parent_record_offset":21484,"ancestor_record_offset":13556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21072,"parent_record_offset":21036,"ancestor_record_offset":13556,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":14860,"parent_record_offset":14768,"ancestor_record_offset":13556,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21484,"parent_record_offset":21420,"ancestor_record_offset":13556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1451,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":13556,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":13556,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21452,"parent_record_offset":21420,"ancestor_record_offset":13556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":30956,"parent_record_offset":30932,"ancestor_record_offset":13556,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14768,"parent_record_offset":14740,"ancestor_record_offset":13556,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14740,"parent_record_offset":14712,"ancestor_record_offset":13556,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":14980,"parent_record_offset":14740,"ancestor_record_offset":13556,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":13556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1451,"record_offset":19688,"parent_record_offset":19652,"ancestor_record_offset":13556,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46268,"parent_record_offset":46232,"ancestor_record_offset":13556,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":21000,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":13556,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":23532,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46232,"parent_record_offset":46208,"ancestor_record_offset":13556,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30932,"parent_record_offset":30888,"ancestor_record_offset":13556,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41512,"parent_record_offset":41164,"ancestor_record_offset":13556,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":16744,"parent_record_offset":16720,"ancestor_record_offset":13556,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30888,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":14712,"parent_record_offset":14684,"ancestor_record_offset":13556,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41132,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35928,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35816,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":55072,"parent_record_offset":55048,"ancestor_record_offset":13556,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52408,"parent_record_offset":52368,"ancestor_record_offset":13556,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52368,"parent_record_offset":52340,"ancestor_record_offset":13556,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52632,"parent_record_offset":52608,"ancestor_record_offset":13556,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1451,"record_offset":52480,"parent_record_offset":52456,"ancestor_record_offset":13556,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1451,"record_offset":19724,"parent_record_offset":19688,"ancestor_record_offset":13556,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1451,"record_offset":41164,"parent_record_offset":41132,"ancestor_record_offset":13556,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":18356,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":18596,"parent_record_offset":18564,"ancestor_record_offset":13556,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":19920,"parent_record_offset":19884,"ancestor_record_offset":13556,"inlinee":62240,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1451,"record_offset":20812,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":14804,"parent_record_offset":14768,"ancestor_record_offset":13556,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":13556,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":13556,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":13556,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14300,"parent_record_offset":14272,"ancestor_record_offset":13556,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":13556,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":14328,"parent_record_offset":14272,"ancestor_record_offset":13556,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":41620,"parent_record_offset":41588,"ancestor_record_offset":13556,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":19616,"parent_record_offset":19580,"ancestor_record_offset":13556,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1451,"record_offset":50516,"parent_record_offset":50192,"ancestor_record_offset":13556,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":15756,"parent_record_offset":15728,"ancestor_record_offset":13556,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":13556,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":15816,"parent_record_offset":15784,"ancestor_record_offset":13556,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17848,"parent_record_offset":17808,"ancestor_record_offset":13556,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":17876,"parent_record_offset":17808,"ancestor_record_offset":13556,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1451,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":13556,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1451,"record_offset":31108,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1451,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":13556,"inlinee":21077,"source_line":25,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":13556,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":13556,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41588,"parent_record_offset":41556,"ancestor_record_offset":13556,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41556,"parent_record_offset":41512,"ancestor_record_offset":13556,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1451,"record_offset":52456,"parent_record_offset":52432,"ancestor_record_offset":13556,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1451,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":13556,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":15988,"parent_record_offset":15964,"ancestor_record_offset":13556,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":16680,"parent_record_offset":16012,"ancestor_record_offset":13556,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1451,"record_offset":16012,"parent_record_offset":15988,"ancestor_record_offset":13556,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1451,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":13556,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":26112,"parent_record_offset":26084,"ancestor_record_offset":13556,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":22396,"parent_record_offset":22364,"ancestor_record_offset":13556,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1451,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":13556,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":31048,"parent_record_offset":31020,"ancestor_record_offset":13556,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":37120,"parent_record_offset":37092,"ancestor_record_offset":13556,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":13556,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":24484,"parent_record_offset":23560,"ancestor_record_offset":13556,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":13556,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19856,"parent_record_offset":19652,"ancestor_record_offset":13556,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":13556,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":49972,"parent_record_offset":49948,"ancestor_record_offset":13556,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":18564,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":15700,"parent_record_offset":15668,"ancestor_record_offset":13556,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":52528,"parent_record_offset":52504,"ancestor_record_offset":13556,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=155
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14652,"parent_record_offset":14612,"ancestor_record_offset":13556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":13556,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":13556,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":13556,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":22608,"parent_record_offset":22580,"ancestor_record_offset":13556,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":46300,"parent_record_offset":46268,"ancestor_record_offset":13556,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":52504,"parent_record_offset":52480,"ancestor_record_offset":13556,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":35892,"parent_record_offset":35844,"ancestor_record_offset":13556,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14936,"parent_record_offset":14908,"ancestor_record_offset":13556,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":13556,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1451,"record_offset":20928,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19760,"parent_record_offset":19724,"ancestor_record_offset":13556,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19796,"parent_record_offset":19760,"ancestor_record_offset":13556,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35960,"parent_record_offset":35928,"ancestor_record_offset":13556,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35992,"parent_record_offset":35960,"ancestor_record_offset":13556,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":31184,"parent_record_offset":31156,"ancestor_record_offset":13556,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":35844,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1451,"record_offset":23416,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":13556,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":54508,"parent_record_offset":54472,"ancestor_record_offset":13556,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":13556,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46328,"parent_record_offset":46268,"ancestor_record_offset":13556,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46360,"parent_record_offset":46232,"ancestor_record_offset":13556,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1451,"record_offset":19652,"parent_record_offset":19616,"ancestor_record_offset":13556,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":23560,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":23397,"source_line":205,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":27880,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$3::try_from_shader::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":1451,"record_offset":49748,"parent_record_offset":49720,"ancestor_record_offset":13556,"inlinee":26099,"source_line":192,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":20480,"parent_record_offset":20428,"ancestor_record_offset":13556,"inlinee":66971,"source_line":211,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":44680,"parent_record_offset":44640,"ancestor_record_offset":13556,"inlinee":66976,"source_line":265,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":54124,"parent_record_offset":54100,"ancestor_record_offset":13556,"inlinee":66975,"source_line":123,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":50192,"parent_record_offset":50148,"ancestor_record_offset":13556,"inlinee":66973,"source_line":129,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefabs_impl::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":50992,"parent_record_offset":50948,"ancestor_record_offset":13556,"inlinee":66974,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::recalculate_canopy_quad_attribs
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":26524,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":66972,"source_line":297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":1451,"record_offset":49948,"parent_record_offset":49688,"ancestor_record_offset":13556,"inlinee":23004,"source_line":221,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34648,"parent_record_offset":34612,"ancestor_record_offset":13556,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34612,"parent_record_offset":34584,"ancestor_record_offset":13556,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34584,"parent_record_offset":34544,"ancestor_record_offset":13556,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50564,"parent_record_offset":50540,"ancestor_record_offset":13556,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34544,"parent_record_offset":34512,"ancestor_record_offset":13556,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34512,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":31020,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50540,"parent_record_offset":50516,"ancestor_record_offset":13556,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":37092,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1451,"record_offset":19580,"parent_record_offset":19544,"ancestor_record_offset":13556,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":13556,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18720,"parent_record_offset":18688,"ancestor_record_offset":13556,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18752,"parent_record_offset":18720,"ancestor_record_offset":13556,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":15520,"parent_record_offset":15492,"ancestor_record_offset":13556,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":13556,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":13556,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25676,"parent_record_offset":25644,"ancestor_record_offset":13556,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":32316,"parent_record_offset":31280,"ancestor_record_offset":13556,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31280,"parent_record_offset":31252,"ancestor_record_offset":13556,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":19544,"parent_record_offset":19292,"ancestor_record_offset":13556,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":13556,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33928,"parent_record_offset":33900,"ancestor_record_offset":13556,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":13556,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18788,"parent_record_offset":18752,"ancestor_record_offset":13556,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19444,"parent_record_offset":19416,"ancestor_record_offset":13556,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19416,"parent_record_offset":19292,"ancestor_record_offset":13556,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19348,"parent_record_offset":19320,"ancestor_record_offset":13556,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16468,"parent_record_offset":16120,"ancestor_record_offset":13556,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22580,"parent_record_offset":21628,"ancestor_record_offset":13556,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":20204,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15668,"parent_record_offset":15576,"ancestor_record_offset":13556,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16120,"parent_record_offset":16088,"ancestor_record_offset":13556,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19320,"parent_record_offset":19292,"ancestor_record_offset":13556,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":13556,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18816,"parent_record_offset":18752,"ancestor_record_offset":13556,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":13556,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":13556,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":13556,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22468,"parent_record_offset":21628,"ancestor_record_offset":13556,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15640,"parent_record_offset":15576,"ancestor_record_offset":13556,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15612,"parent_record_offset":15576,"ancestor_record_offset":13556,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18880,"parent_record_offset":18848,"ancestor_record_offset":13556,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":13556,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21884,"parent_record_offset":21704,"ancestor_record_offset":13556,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22048,"parent_record_offset":21704,"ancestor_record_offset":13556,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22080,"parent_record_offset":22048,"ancestor_record_offset":13556,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":15576,"parent_record_offset":15548,"ancestor_record_offset":13556,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21672,"parent_record_offset":21628,"ancestor_record_offset":13556,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21760,"parent_record_offset":21704,"ancestor_record_offset":13556,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":13556,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22340,"parent_record_offset":21672,"ancestor_record_offset":13556,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=19
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16208,"parent_record_offset":16172,"ancestor_record_offset":13556,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16244,"parent_record_offset":16208,"ancestor_record_offset":13556,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16304,"parent_record_offset":16172,"ancestor_record_offset":13556,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":16172,"parent_record_offset":16120,"ancestor_record_offset":13556,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18936,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":13556,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":13556,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19292,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":13556,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22440,"parent_record_offset":21628,"ancestor_record_offset":13556,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":18964,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=17
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":15728,"parent_record_offset":15668,"ancestor_record_offset":13556,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":19048,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":20112,"parent_record_offset":18880,"ancestor_record_offset":13556,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=20
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":15784,"parent_record_offset":15728,"ancestor_record_offset":13556,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":13556,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":13556,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31984,"parent_record_offset":31764,"ancestor_record_offset":13556,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1451,"record_offset":14004,"parent_record_offset":13980,"ancestor_record_offset":13556,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":44640,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":21450,"source_line":2741,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":32384,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":45000,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":30810,"source_line":2639,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20428,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":38940,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20372,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":18688,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":13556,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":13556,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":25620,"parent_record_offset":18012,"ancestor_record_offset":13556,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":24168,"parent_record_offset":23560,"ancestor_record_offset":13556,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33496,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":15460,"parent_record_offset":15028,"ancestor_record_offset":13556,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":22688,"parent_record_offset":20428,"ancestor_record_offset":13556,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":21580,"parent_record_offset":20428,"ancestor_record_offset":13556,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":16040,"parent_record_offset":16012,"ancestor_record_offset":13556,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":13556,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":13556,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33872,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":29184,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":27944,"parent_record_offset":26524,"ancestor_record_offset":13556,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":13556,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":34684,"parent_record_offset":34648,"ancestor_record_offset":13556,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":13556,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14520,"parent_record_offset":14496,"ancestor_record_offset":13556,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14568,"parent_record_offset":14544,"ancestor_record_offset":13556,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13556,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":13556,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":13556,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14272,"parent_record_offset":14124,"ancestor_record_offset":13556,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":13556,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":13556,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13556,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":13556,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20616,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20672,"parent_record_offset":20480,"ancestor_record_offset":13556,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":14176,"parent_record_offset":14152,"ancestor_record_offset":13556,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1451,"record_offset":14204,"parent_record_offset":14176,"ancestor_record_offset":13556,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":13556,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":13556,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6F
// 首条位置: {"module":489,"inlinee":27503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":489,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":27812,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":489,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":27812,"inlinee":26196,"source_line":16,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$6::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":489,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":27812,"inlinee":26194,"source_line":76,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":28696,"parent_record_offset":28644,"ancestor_record_offset":27812,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":28120,"parent_record_offset":28096,"ancestor_record_offset":27812,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":28204,"parent_record_offset":28180,"ancestor_record_offset":27812,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":489,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":27812,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":28320,"parent_record_offset":28296,"ancestor_record_offset":27812,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":27812,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":28556,"parent_record_offset":28532,"ancestor_record_offset":27812,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":27812,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":28816,"parent_record_offset":28792,"ancestor_record_offset":27812,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":489,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":27812,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":489,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27812,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27888,"parent_record_offset":27864,"ancestor_record_offset":27812,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27912,"parent_record_offset":27888,"ancestor_record_offset":27812,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":28620,"parent_record_offset":28484,"ancestor_record_offset":27812,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27944,"parent_record_offset":27912,"ancestor_record_offset":27812,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28532,"parent_record_offset":28508,"ancestor_record_offset":27812,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28508,"parent_record_offset":28484,"ancestor_record_offset":27812,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28452,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27812,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":27972,"parent_record_offset":27912,"ancestor_record_offset":27812,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":27812,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28264,"parent_record_offset":28236,"ancestor_record_offset":27812,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28296,"parent_record_offset":28264,"ancestor_record_offset":27812,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28236,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28068,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28484,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28096,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28180,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28360,"parent_record_offset":28020,"ancestor_record_offset":27812,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":489,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":27812,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B6E
// 首条位置: {"module":489,"inlinee":27502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::load_prefabs::load_prefabs_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":27812,"parent_record_offset":27788,"ancestor_record_offset":27788,"inlinee":27503,"source_line":88,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":489,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":27788,"inlinee":26196,"source_line":16,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$6::borrow
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":489,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":27788,"inlinee":26194,"source_line":76,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":28696,"parent_record_offset":28644,"ancestor_record_offset":27788,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":28120,"parent_record_offset":28096,"ancestor_record_offset":27788,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":28204,"parent_record_offset":28180,"ancestor_record_offset":27788,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":489,"record_offset":28144,"parent_record_offset":28120,"ancestor_record_offset":27788,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":28320,"parent_record_offset":28296,"ancestor_record_offset":27788,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":27788,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":28556,"parent_record_offset":28532,"ancestor_record_offset":27788,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":27788,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":28816,"parent_record_offset":28792,"ancestor_record_offset":27788,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":489,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":27788,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":489,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27788,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27888,"parent_record_offset":27864,"ancestor_record_offset":27788,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27912,"parent_record_offset":27888,"ancestor_record_offset":27788,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":28620,"parent_record_offset":28484,"ancestor_record_offset":27788,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":27944,"parent_record_offset":27912,"ancestor_record_offset":27788,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28532,"parent_record_offset":28508,"ancestor_record_offset":27788,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28508,"parent_record_offset":28484,"ancestor_record_offset":27788,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28452,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27788,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":27972,"parent_record_offset":27912,"ancestor_record_offset":27788,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":27788,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28264,"parent_record_offset":28236,"ancestor_record_offset":27788,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28296,"parent_record_offset":28264,"ancestor_record_offset":27788,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":28236,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28068,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":28484,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28096,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28180,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":28360,"parent_record_offset":28020,"ancestor_record_offset":27788,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":489,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":27788,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":489,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":27788,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1059F
// 首条位置: {"module":1451,"inlinee":66975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":54148,"parent_record_offset":54124,"ancestor_record_offset":54124,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1059D
// 首条位置: {"module":1451,"inlinee":66973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":50384,"parent_record_offset":50192,"ancestor_record_offset":50192,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":50280,"parent_record_offset":50192,"ancestor_record_offset":50192,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1451,"record_offset":50516,"parent_record_offset":50192,"ancestor_record_offset":50192,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":50352,"parent_record_offset":50280,"ancestor_record_offset":50192,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":50324,"parent_record_offset":50280,"ancestor_record_offset":50192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50564,"parent_record_offset":50540,"ancestor_record_offset":50192,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":50540,"parent_record_offset":50516,"ancestor_record_offset":50192,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefabs_impl::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1059E
// 首条位置: {"module":1451,"inlinee":66974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":51288,"parent_record_offset":50992,"ancestor_record_offset":50992,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":51080,"parent_record_offset":50992,"ancestor_record_offset":50992,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1451,"record_offset":51420,"parent_record_offset":50992,"ancestor_record_offset":50992,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":51152,"parent_record_offset":51080,"ancestor_record_offset":50992,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":51124,"parent_record_offset":51080,"ancestor_record_offset":50992,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":51468,"parent_record_offset":51444,"ancestor_record_offset":50992,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":51444,"parent_record_offset":51420,"ancestor_record_offset":50992,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefab_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10598
// 首条位置: {"module":1451,"inlinee":66968}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":44740,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":23228,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":23532,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41132,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::as_concrete
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":1451,"record_offset":18356,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":22876,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":47548,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":18564,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":25956,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1451,"record_offset":23416,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":23560,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":23397,"source_line":205,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::recalculate_canopy_quad_attribs
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":26524,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":66972,"source_line":297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::and_modify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":44640,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":21450,"source_line":2741,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":45000,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":30810,"source_line":2639,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20428,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":12248,"source_line":2663,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":38940,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20372,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":18688,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":25620,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21128,"parent_record_offset":21104,"ancestor_record_offset":18012,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":18012,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21512,"parent_record_offset":21484,"ancestor_record_offset":18012,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21072,"parent_record_offset":21036,"ancestor_record_offset":18012,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21104,"parent_record_offset":21072,"ancestor_record_offset":18012,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21484,"parent_record_offset":21420,"ancestor_record_offset":18012,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1451,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":18012,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":18012,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21452,"parent_record_offset":21420,"ancestor_record_offset":18012,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":30956,"parent_record_offset":30932,"ancestor_record_offset":18012,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":41256,"parent_record_offset":41228,"ancestor_record_offset":18012,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":41228,"parent_record_offset":41200,"ancestor_record_offset":18012,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":41472,"parent_record_offset":41228,"ancestor_record_offset":18012,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":18012,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1451,"record_offset":19688,"parent_record_offset":19652,"ancestor_record_offset":18012,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46268,"parent_record_offset":46232,"ancestor_record_offset":18012,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":21000,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":18012,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":46232,"parent_record_offset":46208,"ancestor_record_offset":18012,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30932,"parent_record_offset":30888,"ancestor_record_offset":18012,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41512,"parent_record_offset":41164,"ancestor_record_offset":18012,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30888,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":41200,"parent_record_offset":41164,"ancestor_record_offset":18012,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35928,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35816,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1451,"record_offset":19724,"parent_record_offset":19688,"ancestor_record_offset":18012,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1451,"record_offset":41164,"parent_record_offset":41132,"ancestor_record_offset":18012,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":18596,"parent_record_offset":18564,"ancestor_record_offset":18012,"inlinee":66970,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":1451,"record_offset":19920,"parent_record_offset":19884,"ancestor_record_offset":18012,"inlinee":62240,"source_line":108,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1451,"record_offset":20812,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":41288,"parent_record_offset":41256,"ancestor_record_offset":18012,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1451,"record_offset":41316,"parent_record_offset":41288,"ancestor_record_offset":18012,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":18012,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":18012,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":18012,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29508,"parent_record_offset":29484,"ancestor_record_offset":18012,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29484,"parent_record_offset":29428,"ancestor_record_offset":18012,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":41620,"parent_record_offset":41588,"ancestor_record_offset":18012,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":19616,"parent_record_offset":19580,"ancestor_record_offset":18012,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":22160,"parent_record_offset":22136,"ancestor_record_offset":18012,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":18012,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":18012,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":18428,"parent_record_offset":18356,"ancestor_record_offset":18012,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":18400,"parent_record_offset":18356,"ancestor_record_offset":18012,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1451,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":18012,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1451,"record_offset":31108,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":18012,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":18012,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41588,"parent_record_offset":41556,"ancestor_record_offset":18012,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1451,"record_offset":41556,"parent_record_offset":41512,"ancestor_record_offset":18012,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1451,"record_offset":29532,"parent_record_offset":29508,"ancestor_record_offset":18012,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":26112,"parent_record_offset":26084,"ancestor_record_offset":18012,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":22396,"parent_record_offset":22364,"ancestor_record_offset":18012,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1451,"record_offset":19212,"parent_record_offset":19184,"ancestor_record_offset":18012,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":31048,"parent_record_offset":31020,"ancestor_record_offset":18012,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":37120,"parent_record_offset":37092,"ancestor_record_offset":18012,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":29384,"parent_record_offset":29360,"ancestor_record_offset":18012,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":24484,"parent_record_offset":23560,"ancestor_record_offset":18012,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":18012,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19856,"parent_record_offset":19652,"ancestor_record_offset":18012,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":18012,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":33696,"parent_record_offset":33668,"ancestor_record_offset":18012,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=87
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":20968,"parent_record_offset":20928,"ancestor_record_offset":18012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":18012,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":18012,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":19376,"parent_record_offset":19348,"ancestor_record_offset":18012,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":22608,"parent_record_offset":22580,"ancestor_record_offset":18012,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":46300,"parent_record_offset":46268,"ancestor_record_offset":18012,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":35892,"parent_record_offset":35844,"ancestor_record_offset":18012,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":21180,"parent_record_offset":21152,"ancestor_record_offset":18012,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":22636,"parent_record_offset":22608,"ancestor_record_offset":18012,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1451,"record_offset":20928,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19760,"parent_record_offset":19724,"ancestor_record_offset":18012,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":19796,"parent_record_offset":19760,"ancestor_record_offset":18012,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35960,"parent_record_offset":35928,"ancestor_record_offset":18012,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35992,"parent_record_offset":35960,"ancestor_record_offset":18012,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":31184,"parent_record_offset":31156,"ancestor_record_offset":18012,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":35844,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1451,"record_offset":27708,"parent_record_offset":27680,"ancestor_record_offset":18012,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":18012,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":18012,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46328,"parent_record_offset":46268,"ancestor_record_offset":18012,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":46360,"parent_record_offset":46232,"ancestor_record_offset":18012,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":1451,"record_offset":19652,"parent_record_offset":19616,"ancestor_record_offset":18012,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":27880,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":20480,"parent_record_offset":20428,"ancestor_record_offset":18012,"inlinee":66971,"source_line":211,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":1451,"record_offset":44680,"parent_record_offset":44640,"ancestor_record_offset":18012,"inlinee":66976,"source_line":265,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34648,"parent_record_offset":34612,"ancestor_record_offset":18012,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34612,"parent_record_offset":34584,"ancestor_record_offset":18012,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34584,"parent_record_offset":34544,"ancestor_record_offset":18012,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34544,"parent_record_offset":34512,"ancestor_record_offset":18012,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34512,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":31020,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":37092,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1451,"record_offset":19580,"parent_record_offset":19544,"ancestor_record_offset":18012,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":18012,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18720,"parent_record_offset":18688,"ancestor_record_offset":18012,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18752,"parent_record_offset":18720,"ancestor_record_offset":18012,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":18012,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":18012,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":18012,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":25676,"parent_record_offset":25644,"ancestor_record_offset":18012,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":32316,"parent_record_offset":31280,"ancestor_record_offset":18012,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31280,"parent_record_offset":31252,"ancestor_record_offset":18012,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":19544,"parent_record_offset":19292,"ancestor_record_offset":18012,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":23640,"parent_record_offset":23612,"ancestor_record_offset":18012,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33928,"parent_record_offset":33900,"ancestor_record_offset":18012,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":18788,"parent_record_offset":18752,"ancestor_record_offset":18012,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19444,"parent_record_offset":19416,"ancestor_record_offset":18012,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19416,"parent_record_offset":19292,"ancestor_record_offset":18012,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19348,"parent_record_offset":19320,"ancestor_record_offset":18012,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":23980,"parent_record_offset":23700,"ancestor_record_offset":18012,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22580,"parent_record_offset":21628,"ancestor_record_offset":18012,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":20204,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":24320,"parent_record_offset":24264,"ancestor_record_offset":18012,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":23700,"parent_record_offset":23668,"ancestor_record_offset":18012,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19320,"parent_record_offset":19292,"ancestor_record_offset":18012,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18848,"parent_record_offset":18816,"ancestor_record_offset":18012,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18816,"parent_record_offset":18752,"ancestor_record_offset":18012,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":18012,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":18012,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":18012,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22468,"parent_record_offset":21628,"ancestor_record_offset":18012,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":24292,"parent_record_offset":24264,"ancestor_record_offset":18012,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18880,"parent_record_offset":18848,"ancestor_record_offset":18012,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":18012,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21884,"parent_record_offset":21704,"ancestor_record_offset":18012,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22048,"parent_record_offset":21704,"ancestor_record_offset":18012,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22080,"parent_record_offset":22048,"ancestor_record_offset":18012,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":18012,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21672,"parent_record_offset":21628,"ancestor_record_offset":18012,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":21760,"parent_record_offset":21704,"ancestor_record_offset":18012,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":18012,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22340,"parent_record_offset":21672,"ancestor_record_offset":18012,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=17
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":19144,"parent_record_offset":19104,"ancestor_record_offset":18012,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":19184,"parent_record_offset":19144,"ancestor_record_offset":18012,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":19256,"parent_record_offset":19104,"ancestor_record_offset":18012,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":19104,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=11
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":18936,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":18012,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":23668,"parent_record_offset":23640,"ancestor_record_offset":18012,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":19292,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":18012,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":22440,"parent_record_offset":21628,"ancestor_record_offset":18012,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":18964,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":22136,"parent_record_offset":22048,"ancestor_record_offset":18012,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=18
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":19048,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":20112,"parent_record_offset":18880,"ancestor_record_offset":18012,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":21924,"parent_record_offset":21884,"ancestor_record_offset":18012,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":22712,"parent_record_offset":22688,"ancestor_record_offset":18012,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":18012,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31984,"parent_record_offset":31764,"ancestor_record_offset":18012,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":32384,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":31252,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":26756,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":18012,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":18012,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":24168,"parent_record_offset":23560,"ancestor_record_offset":18012,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33496,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":29616,"parent_record_offset":29184,"ancestor_record_offset":18012,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":22688,"parent_record_offset":20428,"ancestor_record_offset":18012,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":21580,"parent_record_offset":20428,"ancestor_record_offset":18012,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":23612,"parent_record_offset":23584,"ancestor_record_offset":18012,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":18012,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":23584,"parent_record_offset":23560,"ancestor_record_offset":18012,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33872,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":29184,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":27944,"parent_record_offset":26524,"ancestor_record_offset":18012,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":20396,"parent_record_offset":20372,"ancestor_record_offset":18012,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":34684,"parent_record_offset":34648,"ancestor_record_offset":18012,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20720,"parent_record_offset":20696,"ancestor_record_offset":18012,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20768,"parent_record_offset":20744,"ancestor_record_offset":18012,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20744,"parent_record_offset":20720,"ancestor_record_offset":18012,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":18012,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":18012,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29428,"parent_record_offset":29280,"ancestor_record_offset":18012,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29208,"parent_record_offset":29184,"ancestor_record_offset":18012,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":18012,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":18012,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20616,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20672,"parent_record_offset":20480,"ancestor_record_offset":18012,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":18012,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1451,"record_offset":29360,"parent_record_offset":29332,"ancestor_record_offset":18012,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":18012,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":18012,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefab_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1059B
// 首条位置: {"module":1451,"inlinee":66971}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20532,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":21000,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1451,"record_offset":20812,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":23247,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":21260,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1451,"record_offset":20928,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20616,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20672,"parent_record_offset":20480,"ancestor_record_offset":20480,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21128,"parent_record_offset":21104,"ancestor_record_offset":20480,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":20480,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21512,"parent_record_offset":21484,"ancestor_record_offset":20480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21072,"parent_record_offset":21036,"ancestor_record_offset":20480,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21104,"parent_record_offset":21072,"ancestor_record_offset":20480,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":21484,"parent_record_offset":21420,"ancestor_record_offset":20480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1451,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":20480,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1451,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":20480,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21452,"parent_record_offset":21420,"ancestor_record_offset":20480,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":20480,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1451,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":20480,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":20480,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":20968,"parent_record_offset":20928,"ancestor_record_offset":20480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":21180,"parent_record_offset":21152,"ancestor_record_offset":20480,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":20480,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":20480,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20720,"parent_record_offset":20696,"ancestor_record_offset":20480,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20768,"parent_record_offset":20744,"ancestor_record_offset":20480,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1451,"record_offset":20744,"parent_record_offset":20720,"ancestor_record_offset":20480,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":20480,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1451,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":20480,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefab_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B77
// 首条位置: {"module":489,"inlinee":27511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefab_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B79
// 首条位置: {"module":489,"inlinee":27513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::load_prefab_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x105A0
// 首条位置: {"module":1451,"inlinee":66976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":44704,"parent_record_offset":44680,"ancestor_record_offset":44680,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":489,"record_offset":46980,"rva":6984112,"va":5375693232}
// 编译实例 RVAs: 0x6a91b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6079098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e8863da3ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5375693317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2943913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e85714d3ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375693604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2944425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e85712d3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375694132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t241382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e8e6ae0300","size":5,"soft_fail":false,"target_va":5375936112,"va":5375694725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dee140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7622225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e8514e7400","size":5,"soft_fail":false,"target_va":5383315776,"va":5375693546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7634486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e8367e7400","size":5,"soft_fail":false,"target_va":5383329296,"va":5375694805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27899702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e836b7a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375693653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27284517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e82554a001","size":5,"soft_fail":false,"target_va":5402978144,"va":5375693622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1832666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e8daf61b00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375694449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27284550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e84654a001","size":5,"soft_fail":false,"target_va":5402978944,"va":5375694389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::retain_vertices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8876298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375693232,"raw_hex":"e80a718700","size":5,"soft_fail":false,"target_va":5384570352,"va":5375694049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":50008,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":51196,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":50988,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":489,"record_offset":48324,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":489,"record_offset":48388,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":23399,"source_line":11,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":489,"record_offset":47220,"parent_record_offset":46980,"ancestor_record_offset":46980,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":48832,"parent_record_offset":48808,"ancestor_record_offset":46980,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":48856,"parent_record_offset":48832,"ancestor_record_offset":46980,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":49848,"parent_record_offset":49824,"ancestor_record_offset":46980,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":48808,"parent_record_offset":48708,"ancestor_record_offset":46980,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":49824,"parent_record_offset":49732,"ancestor_record_offset":46980,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":49760,"parent_record_offset":49732,"ancestor_record_offset":46980,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":48708,"parent_record_offset":48676,"ancestor_record_offset":46980,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":48676,"parent_record_offset":48644,"ancestor_record_offset":46980,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":48928,"parent_record_offset":48676,"ancestor_record_offset":46980,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":49732,"parent_record_offset":49708,"ancestor_record_offset":46980,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":48604,"parent_record_offset":48572,"ancestor_record_offset":46980,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":48572,"parent_record_offset":48540,"ancestor_record_offset":46980,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":48540,"parent_record_offset":48508,"ancestor_record_offset":46980,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":50528,"parent_record_offset":50496,"ancestor_record_offset":46980,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":50160,"parent_record_offset":50116,"ancestor_record_offset":46980,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":48644,"parent_record_offset":48604,"ancestor_record_offset":46980,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":48508,"parent_record_offset":48388,"ancestor_record_offset":46980,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":50044,"parent_record_offset":50008,"ancestor_record_offset":46980,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":50828,"parent_record_offset":50792,"ancestor_record_offset":46980,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":489,"record_offset":50912,"parent_record_offset":50828,"ancestor_record_offset":46980,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":489,"record_offset":50496,"parent_record_offset":50116,"ancestor_record_offset":46980,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":489,"record_offset":50080,"parent_record_offset":50044,"ancestor_record_offset":46980,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":489,"record_offset":50116,"parent_record_offset":50080,"ancestor_record_offset":46980,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":489,"record_offset":48752,"parent_record_offset":48708,"ancestor_record_offset":46980,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":489,"record_offset":48776,"parent_record_offset":48752,"ancestor_record_offset":46980,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":489,"record_offset":49636,"parent_record_offset":49160,"ancestor_record_offset":46980,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":48124,"parent_record_offset":48096,"ancestor_record_offset":46980,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":47484,"parent_record_offset":47460,"ancestor_record_offset":46980,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":47568,"parent_record_offset":47544,"ancestor_record_offset":46980,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":489,"record_offset":48356,"parent_record_offset":48324,"ancestor_record_offset":46980,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":489,"record_offset":51244,"parent_record_offset":51220,"ancestor_record_offset":46980,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":489,"record_offset":47508,"parent_record_offset":47484,"ancestor_record_offset":46980,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":489,"record_offset":50592,"parent_record_offset":50560,"ancestor_record_offset":46980,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":489,"record_offset":50684,"parent_record_offset":50624,"ancestor_record_offset":46980,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":50560,"parent_record_offset":50528,"ancestor_record_offset":46980,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":50792,"parent_record_offset":50684,"ancestor_record_offset":46980,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":489,"record_offset":49788,"parent_record_offset":49760,"ancestor_record_offset":46980,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":47684,"parent_record_offset":47660,"ancestor_record_offset":46980,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":49044,"parent_record_offset":48388,"ancestor_record_offset":46980,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":489,"record_offset":48972,"parent_record_offset":48604,"ancestor_record_offset":46980,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":49684,"parent_record_offset":48388,"ancestor_record_offset":46980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":48028,"parent_record_offset":48004,"ancestor_record_offset":46980,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":47920,"parent_record_offset":47896,"ancestor_record_offset":46980,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":48884,"parent_record_offset":48856,"ancestor_record_offset":46980,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":50872,"parent_record_offset":50828,"ancestor_record_offset":46980,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":48208,"parent_record_offset":48180,"ancestor_record_offset":46980,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":48236,"parent_record_offset":48208,"ancestor_record_offset":46980,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":489,"record_offset":50624,"parent_record_offset":50592,"ancestor_record_offset":46980,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":489,"record_offset":48152,"parent_record_offset":48124,"ancestor_record_offset":46980,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":51220,"parent_record_offset":51196,"ancestor_record_offset":46980,"inlinee":27512,"source_line":282,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":50716,"parent_record_offset":50684,"ancestor_record_offset":46980,"inlinee":27509,"source_line":292,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":50752,"parent_record_offset":50716,"ancestor_record_offset":46980,"inlinee":27511,"source_line":224,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":489,"record_offset":48096,"parent_record_offset":48068,"ancestor_record_offset":46980,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":47248,"parent_record_offset":47220,"ancestor_record_offset":46980,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":47276,"parent_record_offset":47248,"ancestor_record_offset":46980,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":48068,"parent_record_offset":47848,"ancestor_record_offset":46980,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":48472,"parent_record_offset":48448,"ancestor_record_offset":46980,"inlinee":10180,"source_line":196,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":47308,"parent_record_offset":47276,"ancestor_record_offset":46980,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47980,"parent_record_offset":47956,"ancestor_record_offset":46980,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47956,"parent_record_offset":47848,"ancestor_record_offset":46980,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47896,"parent_record_offset":47872,"ancestor_record_offset":46980,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47816,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47872,"parent_record_offset":47848,"ancestor_record_offset":46980,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47360,"parent_record_offset":47336,"ancestor_record_offset":46980,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47336,"parent_record_offset":47276,"ancestor_record_offset":46980,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47384,"parent_record_offset":47360,"ancestor_record_offset":46980,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":47628,"parent_record_offset":47600,"ancestor_record_offset":46980,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":47660,"parent_record_offset":47628,"ancestor_record_offset":46980,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":47600,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47432,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":49964,"parent_record_offset":49940,"ancestor_record_offset":46980,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":47848,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":47460,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":47544,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":47724,"parent_record_offset":47384,"ancestor_record_offset":46980,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":489,"record_offset":48448,"parent_record_offset":48388,"ancestor_record_offset":46980,"inlinee":10178,"source_line":1269,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":489,"record_offset":49104,"parent_record_offset":49076,"ancestor_record_offset":46980,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":489,"record_offset":49076,"parent_record_offset":49044,"ancestor_record_offset":46980,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":489,"record_offset":49132,"parent_record_offset":49104,"ancestor_record_offset":46980,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":489,"record_offset":49160,"parent_record_offset":49132,"ancestor_record_offset":46980,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":489,"record_offset":51492,"rva":6985712,"va":5375694832}
// 编译实例 RVAs: 0x6a97f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6080698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e84637a3ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5375694917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2945513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8170ed3ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375695204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2946033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e80f0cd3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375695740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t239750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e886a80300","size":5,"soft_fail":false,"target_va":5375936112,"va":5375696357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dee140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7620625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e811487400","size":5,"soft_fail":false,"target_va":5383315776,"va":5375695146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7632854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8d6777400","size":5,"soft_fail":false,"target_va":5383329296,"va":5375696437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_prefabs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27898102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8f6b0a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375695253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27282917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8e54da001","size":5,"soft_fail":false,"target_va":5402978144,"va":5375695222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1831044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e884f01b00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375696071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27282928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8f04da001","size":5,"soft_fail":false,"target_va":5402978944,"va":5375696011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::retain_vertices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8874690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375694832,"raw_hex":"e8c26a8700","size":5,"soft_fail":false,"target_va":5384570352,"va":5375695657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":54520,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":55712,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":55504,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":489,"record_offset":52836,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":489,"record_offset":52900,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":23399,"source_line":11,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":489,"record_offset":51732,"parent_record_offset":51492,"ancestor_record_offset":51492,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":53344,"parent_record_offset":53320,"ancestor_record_offset":51492,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":53368,"parent_record_offset":53344,"ancestor_record_offset":51492,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":54360,"parent_record_offset":54336,"ancestor_record_offset":51492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":53320,"parent_record_offset":53220,"ancestor_record_offset":51492,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":489,"record_offset":54336,"parent_record_offset":54244,"ancestor_record_offset":51492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":54272,"parent_record_offset":54244,"ancestor_record_offset":51492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":53220,"parent_record_offset":53188,"ancestor_record_offset":51492,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":53188,"parent_record_offset":53156,"ancestor_record_offset":51492,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":53440,"parent_record_offset":53188,"ancestor_record_offset":51492,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":489,"record_offset":54244,"parent_record_offset":54220,"ancestor_record_offset":51492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":53116,"parent_record_offset":53084,"ancestor_record_offset":51492,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":53084,"parent_record_offset":53052,"ancestor_record_offset":51492,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":489,"record_offset":53052,"parent_record_offset":53020,"ancestor_record_offset":51492,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":55040,"parent_record_offset":55008,"ancestor_record_offset":51492,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":54672,"parent_record_offset":54628,"ancestor_record_offset":51492,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":53156,"parent_record_offset":53116,"ancestor_record_offset":51492,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":53020,"parent_record_offset":52900,"ancestor_record_offset":51492,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":54556,"parent_record_offset":54520,"ancestor_record_offset":51492,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":489,"record_offset":55344,"parent_record_offset":55308,"ancestor_record_offset":51492,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":489,"record_offset":55428,"parent_record_offset":55344,"ancestor_record_offset":51492,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":489,"record_offset":55008,"parent_record_offset":54628,"ancestor_record_offset":51492,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":489,"record_offset":54592,"parent_record_offset":54556,"ancestor_record_offset":51492,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":489,"record_offset":54628,"parent_record_offset":54592,"ancestor_record_offset":51492,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":489,"record_offset":53264,"parent_record_offset":53220,"ancestor_record_offset":51492,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":489,"record_offset":53288,"parent_record_offset":53264,"ancestor_record_offset":51492,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":489,"record_offset":54148,"parent_record_offset":53672,"ancestor_record_offset":51492,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":52636,"parent_record_offset":52608,"ancestor_record_offset":51492,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":51996,"parent_record_offset":51972,"ancestor_record_offset":51492,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":489,"record_offset":52080,"parent_record_offset":52056,"ancestor_record_offset":51492,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":489,"record_offset":52868,"parent_record_offset":52836,"ancestor_record_offset":51492,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":489,"record_offset":55760,"parent_record_offset":55736,"ancestor_record_offset":51492,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":489,"record_offset":52020,"parent_record_offset":51996,"ancestor_record_offset":51492,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":489,"record_offset":55104,"parent_record_offset":55072,"ancestor_record_offset":51492,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":489,"record_offset":55196,"parent_record_offset":55136,"ancestor_record_offset":51492,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":55072,"parent_record_offset":55040,"ancestor_record_offset":51492,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":489,"record_offset":55308,"parent_record_offset":55196,"ancestor_record_offset":51492,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":489,"record_offset":54300,"parent_record_offset":54272,"ancestor_record_offset":51492,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":489,"record_offset":52196,"parent_record_offset":52172,"ancestor_record_offset":51492,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":53556,"parent_record_offset":52900,"ancestor_record_offset":51492,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":489,"record_offset":53484,"parent_record_offset":53116,"ancestor_record_offset":51492,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":54196,"parent_record_offset":52900,"ancestor_record_offset":51492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":52540,"parent_record_offset":52516,"ancestor_record_offset":51492,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":489,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":51492,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":53396,"parent_record_offset":53368,"ancestor_record_offset":51492,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":489,"record_offset":55388,"parent_record_offset":55344,"ancestor_record_offset":51492,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":52720,"parent_record_offset":52692,"ancestor_record_offset":51492,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":52748,"parent_record_offset":52720,"ancestor_record_offset":51492,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":489,"record_offset":55136,"parent_record_offset":55104,"ancestor_record_offset":51492,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":489,"record_offset":52664,"parent_record_offset":52636,"ancestor_record_offset":51492,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":55736,"parent_record_offset":55712,"ancestor_record_offset":51492,"inlinee":27512,"source_line":282,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":55232,"parent_record_offset":55196,"ancestor_record_offset":51492,"inlinee":27509,"source_line":292,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_prefabs::load_prefab_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":55268,"parent_record_offset":55232,"ancestor_record_offset":51492,"inlinee":27513,"source_line":238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":489,"record_offset":52608,"parent_record_offset":52580,"ancestor_record_offset":51492,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":51760,"parent_record_offset":51732,"ancestor_record_offset":51492,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":51788,"parent_record_offset":51760,"ancestor_record_offset":51492,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":52580,"parent_record_offset":52360,"ancestor_record_offset":51492,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":52984,"parent_record_offset":52960,"ancestor_record_offset":51492,"inlinee":10180,"source_line":196,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":489,"record_offset":51820,"parent_record_offset":51788,"ancestor_record_offset":51492,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52492,"parent_record_offset":52468,"ancestor_record_offset":51492,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52468,"parent_record_offset":52360,"ancestor_record_offset":51492,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52408,"parent_record_offset":52384,"ancestor_record_offset":51492,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52328,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52384,"parent_record_offset":52360,"ancestor_record_offset":51492,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":51492,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":51848,"parent_record_offset":51788,"ancestor_record_offset":51492,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":51896,"parent_record_offset":51872,"ancestor_record_offset":51492,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":52140,"parent_record_offset":52112,"ancestor_record_offset":51492,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":52172,"parent_record_offset":52140,"ancestor_record_offset":51492,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":489,"record_offset":52112,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":51944,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":54476,"parent_record_offset":54452,"ancestor_record_offset":51492,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":489,"record_offset":52360,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":51972,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":52056,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":489,"record_offset":52236,"parent_record_offset":51896,"ancestor_record_offset":51492,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":489,"record_offset":52960,"parent_record_offset":52900,"ancestor_record_offset":51492,"inlinee":10178,"source_line":1269,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":489,"record_offset":53616,"parent_record_offset":53588,"ancestor_record_offset":51492,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":489,"record_offset":53588,"parent_record_offset":53556,"ancestor_record_offset":51492,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":489,"record_offset":53644,"parent_record_offset":53616,"ancestor_record_offset":51492,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":489,"record_offset":53672,"parent_record_offset":53644,"ancestor_record_offset":51492,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B78
// 首条位置: {"module":489,"inlinee":27512}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":489,"record_offset":51244,"parent_record_offset":51220,"ancestor_record_offset":51220,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B75
// 首条位置: {"module":489,"inlinee":27509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::load_prefabs::load_prefab_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":50752,"parent_record_offset":50716,"ancestor_record_offset":50716,"inlinee":27511,"source_line":224,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_prefabs::load_prefab_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_prefabs.rs
//     evidence={"module":489,"record_offset":55268,"parent_record_offset":55232,"ancestor_record_offset":55232,"inlinee":27513,"source_line":238,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_prefabs::recalculate_canopy_quad_attribs
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1059C
// 首条位置: {"module":1451,"inlinee":66972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30888,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35928,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":35816,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1451,"record_offset":31108,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":37340,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":35844,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1451,"record_offset":27880,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34512,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":31020,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$5::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":37092,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":13200,"source_line":1005,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":32384,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":31252,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":26756,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":34800,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33496,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":6439,"source_line":456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":35176,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$56::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33872,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":8349,"source_line":2427,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":29184,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":27944,"parent_record_offset":26524,"ancestor_record_offset":26524,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":37636,"parent_record_offset":37608,"ancestor_record_offset":26524,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1451,"record_offset":37608,"parent_record_offset":37504,"ancestor_record_offset":26524,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":37536,"parent_record_offset":37504,"ancestor_record_offset":26524,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":30956,"parent_record_offset":30932,"ancestor_record_offset":26524,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1451,"record_offset":37504,"parent_record_offset":37476,"ancestor_record_offset":26524,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1451,"record_offset":30932,"parent_record_offset":30888,"ancestor_record_offset":26524,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":26524,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1451,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":26524,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":26524,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29508,"parent_record_offset":29484,"ancestor_record_offset":26524,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1451,"record_offset":29484,"parent_record_offset":29428,"ancestor_record_offset":26524,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":27680,"parent_record_offset":27652,"ancestor_record_offset":26524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":32980,"parent_record_offset":32956,"ancestor_record_offset":26524,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":27008,"parent_record_offset":26984,"ancestor_record_offset":26524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1451,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":26524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1451,"record_offset":27912,"parent_record_offset":27880,"ancestor_record_offset":26524,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1451,"record_offset":27032,"parent_record_offset":27008,"ancestor_record_offset":26524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":26524,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1451,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":26524,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1451,"record_offset":29532,"parent_record_offset":29508,"ancestor_record_offset":26524,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":37568,"parent_record_offset":37536,"ancestor_record_offset":26524,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1451,"record_offset":33212,"parent_record_offset":33184,"ancestor_record_offset":26524,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1451,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":26524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":31048,"parent_record_offset":31020,"ancestor_record_offset":26524,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":37120,"parent_record_offset":37092,"ancestor_record_offset":26524,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1451,"record_offset":29384,"parent_record_offset":29360,"ancestor_record_offset":26524,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1451,"record_offset":33696,"parent_record_offset":33668,"ancestor_record_offset":26524,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":37364,"parent_record_offset":37340,"ancestor_record_offset":26524,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":31208,"parent_record_offset":31184,"ancestor_record_offset":26524,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":27584,"parent_record_offset":27560,"ancestor_record_offset":26524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":27476,"parent_record_offset":27452,"ancestor_record_offset":26524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1451,"record_offset":33420,"parent_record_offset":33396,"ancestor_record_offset":26524,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":35892,"parent_record_offset":35844,"ancestor_record_offset":26524,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1451,"record_offset":29556,"parent_record_offset":29532,"ancestor_record_offset":26524,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":27764,"parent_record_offset":27736,"ancestor_record_offset":26524,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1451,"record_offset":27792,"parent_record_offset":27764,"ancestor_record_offset":26524,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35960,"parent_record_offset":35928,"ancestor_record_offset":26524,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1451,"record_offset":35992,"parent_record_offset":35960,"ancestor_record_offset":26524,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":31184,"parent_record_offset":31156,"ancestor_record_offset":26524,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1451,"record_offset":27708,"parent_record_offset":27680,"ancestor_record_offset":26524,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34648,"parent_record_offset":34612,"ancestor_record_offset":26524,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34612,"parent_record_offset":34584,"ancestor_record_offset":26524,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34584,"parent_record_offset":34544,"ancestor_record_offset":26524,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":34544,"parent_record_offset":34512,"ancestor_record_offset":26524,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1451,"record_offset":27652,"parent_record_offset":27624,"ancestor_record_offset":26524,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":26780,"parent_record_offset":26756,"ancestor_record_offset":26524,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":26804,"parent_record_offset":26780,"ancestor_record_offset":26524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":34856,"parent_record_offset":34828,"ancestor_record_offset":26524,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":26524,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":32316,"parent_record_offset":31280,"ancestor_record_offset":26524,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31280,"parent_record_offset":31252,"ancestor_record_offset":26524,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":27624,"parent_record_offset":27404,"ancestor_record_offset":26524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":35232,"parent_record_offset":35204,"ancestor_record_offset":26524,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":33928,"parent_record_offset":33900,"ancestor_record_offset":26524,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1451,"record_offset":26832,"parent_record_offset":26804,"ancestor_record_offset":26524,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27536,"parent_record_offset":27512,"ancestor_record_offset":26524,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27512,"parent_record_offset":27404,"ancestor_record_offset":26524,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27452,"parent_record_offset":27428,"ancestor_record_offset":26524,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":34296,"parent_record_offset":33988,"ancestor_record_offset":26524,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33396,"parent_record_offset":32456,"ancestor_record_offset":26524,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27372,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33668,"parent_record_offset":33608,"ancestor_record_offset":26524,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33988,"parent_record_offset":33956,"ancestor_record_offset":26524,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27428,"parent_record_offset":27404,"ancestor_record_offset":26524,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":26524,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":26860,"parent_record_offset":26804,"ancestor_record_offset":26524,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32456,"parent_record_offset":32432,"ancestor_record_offset":26524,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33580,"parent_record_offset":33552,"ancestor_record_offset":26524,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32340,"parent_record_offset":32316,"ancestor_record_offset":26524,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33256,"parent_record_offset":32456,"ancestor_record_offset":26524,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33640,"parent_record_offset":33608,"ancestor_record_offset":26524,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":26908,"parent_record_offset":26884,"ancestor_record_offset":26524,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32528,"parent_record_offset":32496,"ancestor_record_offset":26524,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32704,"parent_record_offset":32528,"ancestor_record_offset":26524,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32868,"parent_record_offset":32528,"ancestor_record_offset":26524,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32900,"parent_record_offset":32868,"ancestor_record_offset":26524,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33608,"parent_record_offset":33580,"ancestor_record_offset":26524,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32496,"parent_record_offset":32456,"ancestor_record_offset":26524,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":31448,"parent_record_offset":31372,"ancestor_record_offset":26524,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33184,"parent_record_offset":33160,"ancestor_record_offset":26524,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33160,"parent_record_offset":32496,"ancestor_record_offset":26524,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":27156,"parent_record_offset":27124,"ancestor_record_offset":26524,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":27188,"parent_record_offset":27156,"ancestor_record_offset":26524,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":27248,"parent_record_offset":27124,"ancestor_record_offset":26524,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1451,"record_offset":27124,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":26956,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":37412,"parent_record_offset":37388,"ancestor_record_offset":26524,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33956,"parent_record_offset":33928,"ancestor_record_offset":26524,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":27404,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":32924,"parent_record_offset":32900,"ancestor_record_offset":26524,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1451,"record_offset":33368,"parent_record_offset":32456,"ancestor_record_offset":26524,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":26984,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":32956,"parent_record_offset":32868,"ancestor_record_offset":26524,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":27068,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":27280,"parent_record_offset":26908,"ancestor_record_offset":26524,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1451,"record_offset":32744,"parent_record_offset":32704,"ancestor_record_offset":26524,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":32432,"parent_record_offset":32408,"ancestor_record_offset":26524,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":1451,"record_offset":31984,"parent_record_offset":31764,"ancestor_record_offset":26524,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":27976,"parent_record_offset":27944,"ancestor_record_offset":26524,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":34828,"parent_record_offset":34800,"ancestor_record_offset":26524,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33524,"parent_record_offset":33496,"ancestor_record_offset":26524,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":29616,"parent_record_offset":29184,"ancestor_record_offset":26524,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":32408,"parent_record_offset":32384,"ancestor_record_offset":26524,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":35204,"parent_record_offset":35176,"ancestor_record_offset":26524,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1451,"record_offset":33900,"parent_record_offset":33872,"ancestor_record_offset":26524,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":34684,"parent_record_offset":34648,"ancestor_record_offset":26524,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":26524,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":26524,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29428,"parent_record_offset":29280,"ancestor_record_offset":26524,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1451,"record_offset":29208,"parent_record_offset":29184,"ancestor_record_offset":26524,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1451,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":26524,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1451,"record_offset":29360,"parent_record_offset":29332,"ancestor_record_offset":26524,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":29280,"parent_record_offset":29256,"ancestor_record_offset":26524,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1451,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":26524,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01451.jsonl.gz"}
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
// 完整结构及原始类型记录: load_prefabs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>", "constructor": "tuple$", "arguments": ["array$<u32,3>", "country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["array$<u32,3>", "country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_prefabs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_prefabs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::game_profile::SaveGameProfileCmd>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$0
// 信息 ID: dc3e0f232bf420e910a9f830
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_prefabs::filter_mesh_by_vertex_color", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>; module=489, inline_record=51220; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>; module=489, inline_record=55736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::filter_mesh_by_vertex_color::closure$1
// 信息 ID: b04fab21d6d035264aa9059a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_prefabs::filter_mesh_by_vertex_color", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>; module=489, inline_record=50716; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>; module=489, inline_record=55232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
// 信息 ID: 335a1d58a68ea1a8ad6a3be2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_prefabs::filter_mesh_by_vertex_color -> ["country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x1111433 -> 0x6a91b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
// 信息 ID: 6a22f1ad2a073ede95899444
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_prefabs::filter_mesh_by_vertex_color -> ["country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::load_prefabs @ 0x11114fd -> 0x6a97f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefab_mesh
// 信息 ID: f2fc5f9768fb892dc969aad8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=18012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefab_mesh::closure$0
// 信息 ID: 4d7796c3a2d79ef6a2a7a1eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_prefabs::load_prefab_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=20480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefab_mesh::closure$1
// 信息 ID: e29775e6c97c7fa75791a3fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_prefabs::load_prefab_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1 <- country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>; module=489, inline_record=50752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefab_mesh::closure$2
// 信息 ID: cc1be5418a06ebf32eca6428
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_prefabs::load_prefab_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2 <- country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>; module=489, inline_record=55268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefab_mesh::closure$3
// 信息 ID: 1d37ec8db09e33d11693f318
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::startup::load_prefabs::load_prefab_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=44680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs
// 信息 ID: d4b4245361e932029856b4db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 884d79d890a8c582d2091a36: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)> @ 0x6a70e1 -> 0x11106b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)> > @ 0xe635f7 -> 0x11106b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl
// 信息 ID: f9866619efd2e771426b8975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=13556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl::closure$0
// 信息 ID: ea1c4816dd6dc94578cd34ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_prefabs::load_prefabs_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$1>; module=489, inline_record=27812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl::closure$1
// 信息 ID: a8450c368581b53b967c0110
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_prefabs::load_prefabs_impl", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_prefabs::load_prefabs_impl::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$1>; module=489, inline_record=27788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl::closure$2
// 信息 ID: dce5fe0787495379e46bf8af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_prefabs::load_prefabs_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=54124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl::closure$3
// 信息 ID: ce8e0616a309c0391824f0ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::startup::load_prefabs::load_prefabs_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=50192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::load_prefabs_impl::closure$4
// 信息 ID: 53717ec882962b1eb1831f47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::startup::load_prefabs::load_prefabs_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=50992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_prefabs::recalculate_canopy_quad_attribs
// 信息 ID: 9b91427e276380598e8eb49e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs; module=1451, inline_record=26524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
