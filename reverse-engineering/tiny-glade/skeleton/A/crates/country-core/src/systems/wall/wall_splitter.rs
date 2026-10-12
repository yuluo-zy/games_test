// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/wall_splitter.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 74f6b1603db1ea5d55e32c8b38d9b48aa1cef636c70f7445bdcbf9f4dc864ad2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_splitter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 461
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":57116,"rva":6516080,"va":5375225200}
// 编译实例 RVAs: 0x636d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5612502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e82a5caaff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375226017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3649008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e81052c8ff","size":5,"soft_fail":false,"target_va":5371580704,"va":5375229707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bf350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3640225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e85f74c8ff","size":5,"soft_fail":false,"target_va":5371589456,"va":5375229676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3583703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e82951c9ff","size":5,"soft_fail":false,"target_va":5371650384,"va":5375234082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2605938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e88e3cd8ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375230957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2476651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e89535daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375226342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2479138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8de2bdaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375228845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-446829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8932ef9ff","size":5,"soft_fail":false,"target_va":5374781984,"va":5375228808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406843f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t311203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8a3bf0400","size":5,"soft_fail":false,"target_va":5375542256,"va":5375231048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140684c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t314532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8a4cc0400","size":5,"soft_fail":false,"target_va":5375544352,"va":5375229815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140685f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t321731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8c3e80400","size":5,"soft_fail":false,"target_va":5375549312,"va":5375227576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t646822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8a6de0900","size":5,"soft_fail":false,"target_va":5375881440,"va":5375234613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de760
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t676118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e816510a00","size":5,"soft_fail":false,"target_va":5375911776,"va":5375235653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1717060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e844331a00","size":5,"soft_fail":false,"target_va":5376944000,"va":5375226935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4420469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e875734300","size":5,"soft_fail":false,"target_va":5379651280,"va":5375230806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7632626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8f2767400","size":5,"soft_fail":false,"target_va":5382858112,"va":5375225481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8720128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8000f8500","size":5,"soft_fail":false,"target_va":5383948048,"va":5375227915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8a0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8724326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8661f8500","size":5,"soft_fail":false,"target_va":5383954608,"va":5375230277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8abc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8729772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8ac348500","size":5,"soft_fail":false,"target_va":5383957440,"va":5375227663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8736239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8ef4d8500","size":5,"soft_fail":false,"target_va":5383966736,"va":5375230492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f34870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9422203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e87bc58f00","size":5,"soft_fail":false,"target_va":5384652912,"va":5375230704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14104bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10570767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e80f4ca100","size":5,"soft_fail":false,"target_va":5385797360,"va":5375226588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28364608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e840cfb001","size":5,"soft_fail":false,"target_va":5403593360,"va":5375228747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t27745313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8215ca701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375232826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2302496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e820222300","size":5,"soft_fail":false,"target_va":5377529888,"va":5375227387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-3918496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e86035c4ff","size":5,"soft_fail":false,"target_va":5371308608,"va":5375227099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2294261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8f5012300","size":5,"soft_fail":false,"target_va":5377527120,"va":5375232854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27746422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e87660a701","size":5,"soft_fail":false,"target_va":5402978880,"va":5375232453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27746482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8b260a701","size":5,"soft_fail":false,"target_va":5402979152,"va":5375232665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t27746859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e82b62a701","size":5,"soft_fail":false,"target_va":5402979344,"va":5375232480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27745878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8565ea701","size":5,"soft_fail":false,"target_va":5402978944,"va":5375233061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8c0b6ffff","size":5,"soft_fail":false,"target_va":5375216304,"va":5375235051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-264991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8e1f4fbff","size":5,"soft_fail":false,"target_va":5374966368,"va":5375231354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-516908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8d41cf8ff","size":5,"soft_fail":false,"target_va":5374712960,"va":5375229863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-511266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8de32f8ff","size":5,"soft_fail":false,"target_va":5374720592,"va":5375231853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t56990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e89ede0000","size":5,"soft_fail":false,"target_va":5375286960,"va":5375229965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-404998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8fad1f9ff","size":5,"soft_fail":false,"target_va":5374826656,"va":5375231649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-405293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8d3d0f9ff","size":5,"soft_fail":false,"target_va":5374824624,"va":5375229912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-403320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e888d8f9ff","size":5,"soft_fail":false,"target_va":5374828608,"va":5375231923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-405748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e80ccff9ff","size":5,"soft_fail":false,"target_va":5374823840,"va":5375229583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2135291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e8fb942000","size":5,"soft_fail":false,"target_va":5377366224,"va":5375230928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7039852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e86c6b6b00","size":5,"soft_fail":false,"target_va":5382266928,"va":5375227071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1712193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e841201a00","size":5,"soft_fail":false,"target_va":5376939952,"va":5375227754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1715037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375225200,"raw_hex":"e85d2b1a00","size":5,"soft_fail":false,"target_va":5376943376,"va":5375228334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":461,"record_offset":60832,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":461,"record_offset":63588,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":60356,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":59560,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":57972,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":461,"record_offset":58076,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":461,"record_offset":60776,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":461,"record_offset":71568,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":461,"record_offset":69256,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":58224,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":60088,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":59468,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":461,"record_offset":59348,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":64564,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":461,"record_offset":59908,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::WallId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":461,"record_offset":68284,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22070,"source_line":83,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":461,"record_offset":69520,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22333,"source_line":117,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":461,"record_offset":70528,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":24551,"source_line":21,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::StructureType::is_fence
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":461,"record_offset":68256,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22414,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":461,"record_offset":69552,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22411,"source_line":38,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":461,"record_offset":66024,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22068,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":461,"record_offset":69580,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":24545,"source_line":62,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":461,"record_offset":59384,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":461,"record_offset":58280,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":461,"record_offset":57424,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":461,"record_offset":67052,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22964,"source_line":238,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":461,"record_offset":60944,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":23070,"source_line":243,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":461,"record_offset":60916,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":24894,"source_line":87,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":63804,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":461,"record_offset":64536,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":461,"record_offset":70220,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":24547,"source_line":17,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":461,"record_offset":60172,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":461,"record_offset":60496,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":61964,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":62844,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":61040,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21780,"source_line":664,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":59412,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":59440,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::min
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":461,"record_offset":61908,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":26375,"source_line":33,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":461,"record_offset":61480,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":461,"record_offset":69196,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":62584,"parent_record_offset":57116,"ancestor_record_offset":57116,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":59768,"parent_record_offset":59744,"ancestor_record_offset":57116,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":59792,"parent_record_offset":59768,"ancestor_record_offset":57116,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":64716,"parent_record_offset":64692,"ancestor_record_offset":57116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":63644,"parent_record_offset":63616,"ancestor_record_offset":57116,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":59744,"parent_record_offset":59644,"ancestor_record_offset":57116,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":64692,"parent_record_offset":64636,"ancestor_record_offset":57116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":63616,"parent_record_offset":63588,"ancestor_record_offset":57116,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":461,"record_offset":60880,"parent_record_offset":60856,"ancestor_record_offset":57116,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":64664,"parent_record_offset":64636,"ancestor_record_offset":57116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":60432,"parent_record_offset":60400,"ancestor_record_offset":57116,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":59644,"parent_record_offset":59616,"ancestor_record_offset":57116,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":59616,"parent_record_offset":59588,"ancestor_record_offset":57116,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":59864,"parent_record_offset":59616,"ancestor_record_offset":57116,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":64636,"parent_record_offset":64612,"ancestor_record_offset":57116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":64008,"parent_record_offset":63980,"ancestor_record_offset":57116,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":63980,"parent_record_offset":63952,"ancestor_record_offset":57116,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":63924,"parent_record_offset":63896,"ancestor_record_offset":57116,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":63952,"parent_record_offset":63924,"ancestor_record_offset":57116,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":63896,"parent_record_offset":63868,"ancestor_record_offset":57116,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":63868,"parent_record_offset":63840,"ancestor_record_offset":57116,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":63840,"parent_record_offset":63804,"ancestor_record_offset":57116,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":461,"record_offset":64404,"parent_record_offset":64380,"ancestor_record_offset":57116,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":60400,"parent_record_offset":60356,"ancestor_record_offset":57116,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":71684,"parent_record_offset":71648,"ancestor_record_offset":57116,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":59588,"parent_record_offset":59560,"ancestor_record_offset":57116,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":61104,"parent_record_offset":61040,"ancestor_record_offset":57116,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":57936,"parent_record_offset":57912,"ancestor_record_offset":57116,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":461,"record_offset":74136,"parent_record_offset":74112,"ancestor_record_offset":57116,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":461,"record_offset":74184,"parent_record_offset":74160,"ancestor_record_offset":57116,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=20
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":461,"record_offset":60800,"parent_record_offset":60776,"ancestor_record_offset":57116,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":461,"record_offset":71592,"parent_record_offset":71568,"ancestor_record_offset":57116,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":461,"record_offset":71648,"parent_record_offset":71568,"ancestor_record_offset":57116,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":59680,"parent_record_offset":59644,"ancestor_record_offset":57116,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":461,"record_offset":59708,"parent_record_offset":59680,"ancestor_record_offset":57116,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":461,"record_offset":69344,"parent_record_offset":69280,"ancestor_record_offset":57116,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":461,"record_offset":70828,"parent_record_offset":70800,"ancestor_record_offset":57116,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":461,"record_offset":70880,"parent_record_offset":70856,"ancestor_record_offset":57116,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":461,"record_offset":70856,"parent_record_offset":70800,"ancestor_record_offset":57116,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":70248,"parent_record_offset":70220,"ancestor_record_offset":57116,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":61408,"parent_record_offset":61384,"ancestor_record_offset":57116,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":61432,"parent_record_offset":61408,"ancestor_record_offset":57116,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":57116,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":461,"record_offset":59100,"parent_record_offset":59076,"ancestor_record_offset":57116,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":461,"record_offset":57764,"parent_record_offset":57732,"ancestor_record_offset":57116,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":461,"record_offset":58568,"parent_record_offset":58544,"ancestor_record_offset":57116,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":461,"record_offset":57824,"parent_record_offset":57792,"ancestor_record_offset":57116,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":461,"record_offset":69220,"parent_record_offset":69196,"ancestor_record_offset":57116,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":461,"record_offset":61636,"parent_record_offset":61480,"ancestor_record_offset":57116,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":461,"record_offset":61932,"parent_record_offset":61908,"ancestor_record_offset":57116,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":461,"record_offset":61664,"parent_record_offset":61480,"ancestor_record_offset":57116,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":58592,"parent_record_offset":58568,"ancestor_record_offset":57116,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":461,"record_offset":60316,"parent_record_offset":60292,"ancestor_record_offset":57116,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":461,"record_offset":60112,"parent_record_offset":60088,"ancestor_record_offset":57116,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":461,"record_offset":60232,"parent_record_offset":60196,"ancestor_record_offset":57116,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":461,"record_offset":60136,"parent_record_offset":60112,"ancestor_record_offset":57116,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":461,"record_offset":60256,"parent_record_offset":60232,"ancestor_record_offset":57116,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":60292,"parent_record_offset":60196,"ancestor_record_offset":57116,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":461,"record_offset":57912,"parent_record_offset":57424,"ancestor_record_offset":57116,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":461,"record_offset":58008,"parent_record_offset":57972,"ancestor_record_offset":57116,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":461,"record_offset":69368,"parent_record_offset":69344,"ancestor_record_offset":57116,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":461,"record_offset":70904,"parent_record_offset":70880,"ancestor_record_offset":57116,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":67444,"parent_record_offset":67416,"ancestor_record_offset":57116,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":71352,"parent_record_offset":71292,"ancestor_record_offset":57116,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":67468,"parent_record_offset":67444,"ancestor_record_offset":57116,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":59492,"parent_record_offset":59468,"ancestor_record_offset":57116,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":461,"record_offset":69824,"parent_record_offset":69792,"ancestor_record_offset":57116,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":461,"record_offset":58764,"parent_record_offset":58740,"ancestor_record_offset":57116,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":62424,"parent_record_offset":62348,"ancestor_record_offset":57116,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":62452,"parent_record_offset":62348,"ancestor_record_offset":57116,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":62396,"parent_record_offset":62348,"ancestor_record_offset":57116,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":461,"record_offset":69316,"parent_record_offset":69280,"ancestor_record_offset":57116,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":461,"record_offset":69280,"parent_record_offset":69256,"ancestor_record_offset":57116,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":461,"record_offset":61384,"parent_record_offset":61360,"ancestor_record_offset":57116,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":461,"record_offset":61360,"parent_record_offset":61324,"ancestor_record_offset":57116,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":461,"record_offset":70756,"parent_record_offset":70732,"ancestor_record_offset":57116,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":461,"record_offset":60856,"parent_record_offset":60832,"ancestor_record_offset":57116,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":57704,"parent_record_offset":57664,"ancestor_record_offset":57116,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":64296,"parent_record_offset":64008,"ancestor_record_offset":57116,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":69392,"parent_record_offset":69368,"ancestor_record_offset":57116,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":64380,"parent_record_offset":63804,"ancestor_record_offset":57116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":461,"record_offset":58044,"parent_record_offset":57972,"ancestor_record_offset":57116,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":461,"record_offset":59012,"parent_record_offset":58988,"ancestor_record_offset":57116,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":461,"record_offset":58904,"parent_record_offset":58880,"ancestor_record_offset":57116,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":461,"record_offset":58192,"parent_record_offset":58076,"ancestor_record_offset":57116,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":461,"record_offset":58136,"parent_record_offset":58076,"ancestor_record_offset":57116,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":461,"record_offset":58108,"parent_record_offset":58076,"ancestor_record_offset":57116,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":58160,"parent_record_offset":58136,"ancestor_record_offset":57116,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":59820,"parent_record_offset":59792,"ancestor_record_offset":57116,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":60464,"parent_record_offset":60356,"ancestor_record_offset":57116,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":461,"record_offset":61132,"parent_record_offset":61104,"ancestor_record_offset":57116,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":461,"record_offset":61160,"parent_record_offset":61132,"ancestor_record_offset":57116,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":461,"record_offset":59936,"parent_record_offset":59908,"ancestor_record_offset":57116,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":461,"record_offset":60572,"parent_record_offset":60548,"ancestor_record_offset":57116,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":64440,"parent_record_offset":64404,"ancestor_record_offset":57116,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":64464,"parent_record_offset":64440,"ancestor_record_offset":57116,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":461,"record_offset":64496,"parent_record_offset":64404,"ancestor_record_offset":57116,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":461,"record_offset":58248,"parent_record_offset":58224,"ancestor_record_offset":57116,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":461,"record_offset":67080,"parent_record_offset":67052,"ancestor_record_offset":57116,"inlinee":22965,"source_line":134,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":461,"record_offset":60972,"parent_record_offset":60944,"ancestor_record_offset":57116,"inlinee":22961,"source_line":152,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":461,"record_offset":61004,"parent_record_offset":60972,"ancestor_record_offset":57116,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":63360,"parent_record_offset":63324,"ancestor_record_offset":57116,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":63324,"parent_record_offset":63288,"ancestor_record_offset":57116,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":63288,"parent_record_offset":63260,"ancestor_record_offset":57116,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":62324,"parent_record_offset":62300,"ancestor_record_offset":57116,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":63260,"parent_record_offset":63232,"ancestor_record_offset":57116,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":62348,"parent_record_offset":62324,"ancestor_record_offset":57116,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":461,"record_offset":59076,"parent_record_offset":59052,"ancestor_record_offset":57116,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":461,"record_offset":58340,"parent_record_offset":58316,"ancestor_record_offset":57116,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":461,"record_offset":58364,"parent_record_offset":58340,"ancestor_record_offset":57116,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":461,"record_offset":57508,"parent_record_offset":57476,"ancestor_record_offset":57116,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":461,"record_offset":59052,"parent_record_offset":58832,"ancestor_record_offset":57116,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":461,"record_offset":58396,"parent_record_offset":58364,"ancestor_record_offset":57116,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58964,"parent_record_offset":58940,"ancestor_record_offset":57116,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58940,"parent_record_offset":58832,"ancestor_record_offset":57116,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58880,"parent_record_offset":58856,"ancestor_record_offset":57116,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":59260,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":57664,"parent_record_offset":57572,"ancestor_record_offset":57116,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58856,"parent_record_offset":58832,"ancestor_record_offset":57116,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58448,"parent_record_offset":58424,"ancestor_record_offset":57116,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58424,"parent_record_offset":58364,"ancestor_record_offset":57116,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":57540,"parent_record_offset":57508,"ancestor_record_offset":57116,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67384,"parent_record_offset":67352,"ancestor_record_offset":57116,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":57636,"parent_record_offset":57572,"ancestor_record_offset":57116,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":57608,"parent_record_offset":57572,"ancestor_record_offset":57116,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67296,"parent_record_offset":67272,"ancestor_record_offset":57116,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58472,"parent_record_offset":58448,"ancestor_record_offset":57116,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67352,"parent_record_offset":67296,"ancestor_record_offset":57116,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67324,"parent_record_offset":67296,"ancestor_record_offset":57116,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":57572,"parent_record_offset":57540,"ancestor_record_offset":57116,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67416,"parent_record_offset":67384,"ancestor_record_offset":57116,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":461,"record_offset":58712,"parent_record_offset":58684,"ancestor_record_offset":57116,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":461,"record_offset":58740,"parent_record_offset":58712,"ancestor_record_offset":57116,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":461,"record_offset":58800,"parent_record_offset":58684,"ancestor_record_offset":57116,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":461,"record_offset":58684,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58516,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":67272,"parent_record_offset":67248,"ancestor_record_offset":57116,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":461,"record_offset":58832,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":461,"record_offset":58544,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":461,"record_offset":57732,"parent_record_offset":57664,"ancestor_record_offset":57116,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":461,"record_offset":58628,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":461,"record_offset":59168,"parent_record_offset":58472,"ancestor_record_offset":57116,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":461,"record_offset":57792,"parent_record_offset":57732,"ancestor_record_offset":57116,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":461,"record_offset":70184,"parent_record_offset":70160,"ancestor_record_offset":57116,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":461,"record_offset":60196,"parent_record_offset":60172,"ancestor_record_offset":57116,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":461,"record_offset":58316,"parent_record_offset":58280,"ancestor_record_offset":57116,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":461,"record_offset":69612,"parent_record_offset":69580,"ancestor_record_offset":57116,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":461,"record_offset":57476,"parent_record_offset":57452,"ancestor_record_offset":57116,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":461,"record_offset":57452,"parent_record_offset":57424,"ancestor_record_offset":57116,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":461,"record_offset":70556,"parent_record_offset":70528,"ancestor_record_offset":57116,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":461,"record_offset":70160,"parent_record_offset":69580,"ancestor_record_offset":57116,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":461,"record_offset":63396,"parent_record_offset":63360,"ancestor_record_offset":57116,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":461,"record_offset":70604,"parent_record_offset":70580,"ancestor_record_offset":57116,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":461,"record_offset":70680,"parent_record_offset":70652,"ancestor_record_offset":57116,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":461,"record_offset":70800,"parent_record_offset":70652,"ancestor_record_offset":57116,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":461,"record_offset":70580,"parent_record_offset":70556,"ancestor_record_offset":57116,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":461,"record_offset":70704,"parent_record_offset":70680,"ancestor_record_offset":57116,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":461,"record_offset":70732,"parent_record_offset":70704,"ancestor_record_offset":57116,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":461,"record_offset":70652,"parent_record_offset":70628,"ancestor_record_offset":57116,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":461,"record_offset":70628,"parent_record_offset":70604,"ancestor_record_offset":57116,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":62888,"parent_record_offset":62844,"ancestor_record_offset":57116,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":61996,"parent_record_offset":61964,"ancestor_record_offset":57116,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":59316,"parent_record_offset":58280,"ancestor_record_offset":57116,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":61076,"parent_record_offset":61040,"ancestor_record_offset":57116,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":461,"record_offset":61200,"parent_record_offset":61040,"ancestor_record_offset":57116,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":461,"record_offset":61324,"parent_record_offset":61040,"ancestor_record_offset":57116,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":461,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":57116,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":461,"record_offset":62300,"parent_record_offset":61996,"ancestor_record_offset":57116,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":461,"record_offset":63232,"parent_record_offset":62844,"ancestor_record_offset":57116,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":75592,"rva":6526624,"va":5375235744}
// 编译实例 RVAs: 0x6396a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2486116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e89c10daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375235807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2486700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e8540edaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375236407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t103967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e81f960100","size":5,"soft_fail":false,"target_va":5375340432,"va":5375236460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27741823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e87f4ea701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375236316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27682906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e85a68a601","size":5,"soft_fail":false,"target_va":5402919072,"va":5375236161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2010347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e8ebac1e00","size":5,"soft_fail":false,"target_va":5377246528,"va":5375236176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t120776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375235744,"raw_hex":"e8c8d70100","size":5,"soft_fail":false,"target_va":5375356896,"va":5375236115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":75764,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77216,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":76660,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":77300,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":76632,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":76060,"parent_record_offset":75592,"ancestor_record_offset":75592,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":75820,"parent_record_offset":75796,"ancestor_record_offset":75592,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":75844,"parent_record_offset":75820,"ancestor_record_offset":75592,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77372,"parent_record_offset":77348,"ancestor_record_offset":75592,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":75796,"parent_record_offset":75764,"ancestor_record_offset":75592,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77348,"parent_record_offset":77324,"ancestor_record_offset":75592,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":77324,"parent_record_offset":77300,"ancestor_record_offset":75592,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76888,"parent_record_offset":76860,"ancestor_record_offset":75592,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76860,"parent_record_offset":76828,"ancestor_record_offset":75592,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76828,"parent_record_offset":76800,"ancestor_record_offset":75592,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76360,"parent_record_offset":76332,"ancestor_record_offset":75592,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76332,"parent_record_offset":76304,"ancestor_record_offset":75592,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":76508,"parent_record_offset":76332,"ancestor_record_offset":75592,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":76268,"parent_record_offset":76240,"ancestor_record_offset":75592,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77240,"parent_record_offset":77216,"ancestor_record_offset":75592,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":76240,"parent_record_offset":76212,"ancestor_record_offset":75592,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":76184,"parent_record_offset":76156,"ancestor_record_offset":75592,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":76212,"parent_record_offset":76184,"ancestor_record_offset":75592,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":76156,"parent_record_offset":76128,"ancestor_record_offset":75592,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":76128,"parent_record_offset":76100,"ancestor_record_offset":75592,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":76100,"parent_record_offset":76060,"ancestor_record_offset":75592,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":77264,"parent_record_offset":77240,"ancestor_record_offset":75592,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":76940,"parent_record_offset":76772,"ancestor_record_offset":75592,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":76800,"parent_record_offset":76772,"ancestor_record_offset":75592,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":76304,"parent_record_offset":76268,"ancestor_record_offset":75592,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":76708,"parent_record_offset":76684,"ancestor_record_offset":75592,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":76736,"parent_record_offset":76708,"ancestor_record_offset":75592,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":461,"record_offset":76684,"parent_record_offset":76660,"ancestor_record_offset":75592,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":76972,"parent_record_offset":76736,"ancestor_record_offset":75592,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":461,"record_offset":76772,"parent_record_offset":76736,"ancestor_record_offset":75592,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":76568,"parent_record_offset":76544,"ancestor_record_offset":75592,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":76544,"parent_record_offset":76268,"ancestor_record_offset":75592,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":75868,"parent_record_offset":75844,"ancestor_record_offset":75592,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":461,"record_offset":77416,"rva":6527360,"va":5375236480}
// 编译实例 RVAs: 0x639980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2486852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375236480,"raw_hex":"e8bc0ddaff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375236543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2487308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375236480,"raw_hex":"e8f40bdaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375237015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t103359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375236480,"raw_hex":"e8bf930100","size":5,"soft_fail":false,"target_va":5375340432,"va":5375237068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27741216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375236480,"raw_hex":"e8204ca701","size":5,"soft_fail":false,"target_va":5402978144,"va":5375236923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t120032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375236480,"raw_hex":"e8e0d40100","size":5,"soft_fail":false,"target_va":5375356896,"va":5375236859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77592,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":78992,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":461,"record_offset":77740,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":79076,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":78296,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":461,"record_offset":78960,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":26369,"source_line":184,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":461,"record_offset":78868,"parent_record_offset":77416,"ancestor_record_offset":77416,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77648,"parent_record_offset":77624,"ancestor_record_offset":77416,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77672,"parent_record_offset":77648,"ancestor_record_offset":77416,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":79148,"parent_record_offset":79124,"ancestor_record_offset":77416,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":77624,"parent_record_offset":77592,"ancestor_record_offset":77416,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":461,"record_offset":79124,"parent_record_offset":79100,"ancestor_record_offset":77416,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":461,"record_offset":79100,"parent_record_offset":79076,"ancestor_record_offset":77416,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":78028,"parent_record_offset":78000,"ancestor_record_offset":77416,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":78000,"parent_record_offset":77972,"ancestor_record_offset":77416,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":461,"record_offset":78172,"parent_record_offset":78000,"ancestor_record_offset":77416,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77936,"parent_record_offset":77908,"ancestor_record_offset":77416,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":79016,"parent_record_offset":78992,"ancestor_record_offset":77416,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77908,"parent_record_offset":77880,"ancestor_record_offset":77416,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77852,"parent_record_offset":77824,"ancestor_record_offset":77416,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77880,"parent_record_offset":77852,"ancestor_record_offset":77416,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":461,"record_offset":77824,"parent_record_offset":77796,"ancestor_record_offset":77416,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":461,"record_offset":77796,"parent_record_offset":77768,"ancestor_record_offset":77416,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":461,"record_offset":77768,"parent_record_offset":77740,"ancestor_record_offset":77416,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":79040,"parent_record_offset":79016,"ancestor_record_offset":77416,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":461,"record_offset":77972,"parent_record_offset":77936,"ancestor_record_offset":77416,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":461,"record_offset":78232,"parent_record_offset":78208,"ancestor_record_offset":77416,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":461,"record_offset":78900,"parent_record_offset":78868,"ancestor_record_offset":77416,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":461,"record_offset":78208,"parent_record_offset":77936,"ancestor_record_offset":77416,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":461,"record_offset":77696,"parent_record_offset":77672,"ancestor_record_offset":77416,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00461.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_splitter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_splitter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_splitter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::window::CursorMoved>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter
// 信息 ID: d5f761ed7f27e969932e8890
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c991e1a3ece89411445355c3: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> > @ 0xb9de6 -> 0x636d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> @ 0x1caf8c -> 0x636d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 信息 ID: bf50d2804b1d1a56793a0e63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x6348e2 -> 0x6396a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 信息 ID: b1dc5353086636753f51639b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x6349d2 -> 0x639980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
