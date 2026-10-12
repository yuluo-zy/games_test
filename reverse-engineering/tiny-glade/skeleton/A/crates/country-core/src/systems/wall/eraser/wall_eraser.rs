// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/eraser/wall_eraser.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3cc966e76cfcc657adfabe430a829c7414fb938059bc0b5263a5b7db1ac6d4dc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_eraser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 423, 1308, 1321
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":423,"record_offset":4396,"rva":5923440,"va":5374632560}
// 编译实例 RVAs: 0x5a6270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=5
//     target_aliases=9 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5021708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8f45fb3ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374635223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd120
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3052731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8456bd1ff","size":5,"soft_fail":false,"target_va":5371580704,"va":5374633430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bf350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3043944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8988dd1ff","size":5,"soft_fail":false,"target_va":5371589456,"va":5374633395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2015025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8cf40e1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374640044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1887418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e84633e3ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374637109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=23
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1883663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8f141e3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374633370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406248b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t508553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e889c20700","size":5,"soft_fail":false,"target_va":5375150256,"va":5374641698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140624b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t511892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e894cf0700","size":5,"soft_fail":false,"target_va":5375150848,"va":5374638951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406843f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t902109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8ddc30d00","size":5,"soft_fail":false,"target_va":5375542256,"va":5374640142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140684c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t906808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e838d60d00","size":5,"soft_fail":false,"target_va":5375544352,"va":5374637539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=9
//     target_aliases=14 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1241008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8b0ef1200","size":5,"soft_fail":false,"target_va":5375881440,"va":5374640427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dbab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1251509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8b5181300","size":5,"soft_fail":false,"target_va":5375900336,"va":5374648822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de760
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1264635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8fb4b1300","size":5,"soft_fail":false,"target_va":5375911776,"va":5374647136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e1d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1277900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8cc7f1300","size":5,"soft_fail":false,"target_va":5375925584,"va":5374647679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2301209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8191d2300","size":5,"soft_fail":false,"target_va":5376939904,"va":5374638690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2300842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8aa1b2300","size":5,"soft_fail":false,"target_va":5376944000,"va":5374643153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5011403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8cb774c00","size":5,"soft_fail":false,"target_va":5379651280,"va":5374639872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d77160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8190303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e85ff97c00","size":5,"soft_fail":false,"target_va":5382828384,"va":5374638076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d78d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8200830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e87e227d00","size":5,"soft_fail":false,"target_va":5382835520,"va":5374634685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8673847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8375a8400","size":5,"soft_fail":false,"target_va":5383308144,"va":5374634292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8add0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9321266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8323b8e00","size":5,"soft_fail":false,"target_va":5383957968,"va":5374636697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9328389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e805578e00","size":5,"soft_fail":false,"target_va":5383966736,"va":5374638342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f34870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10013140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8d4c99800","size":5,"soft_fail":false,"target_va":5384652912,"va":5374639767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f373c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10022255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e86fed9800","size":5,"soft_fail":false,"target_va":5384664000,"va":5374641740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28959900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e89ce4b901","size":5,"soft_fail":false,"target_va":5403593360,"va":5374633455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28333676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e86c56b001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374644463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t-3328577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8bf35cdff","size":5,"soft_fail":false,"target_va":5371305056,"va":5374633628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2882638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e84efc2b00","size":5,"soft_fail":false,"target_va":5377527120,"va":5374644477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28334084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e80458b001","size":5,"soft_fail":false,"target_va":5402978688,"va":5374644599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t28334764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8ac5ab001","size":5,"soft_fail":false,"target_va":5402978880,"va":5374644111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t28335000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8985bb001","size":5,"soft_fail":false,"target_va":5402979152,"va":5374644147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t28335464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8685db001","size":5,"soft_fail":false,"target_va":5402979344,"va":5374643875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e88dcdffff","size":5,"soft_fail":false,"target_va":5374630848,"va":5374643758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28345779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8b385b001","size":5,"soft_fail":false,"target_va":5402980128,"va":5374634344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e86af6ffff","size":5,"soft_fail":false,"target_va":5374631040,"va":5374633489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28347073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8c18ab001","size":5,"soft_fail":false,"target_va":5402980656,"va":5374633578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t75379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e873260100","size":5,"soft_fail":false,"target_va":5374712960,"va":5374637576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t79353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8f9350100","size":5,"soft_fail":false,"target_va":5374720592,"va":5374641234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t649009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e831e70900","size":5,"soft_fail":false,"target_va":5375286960,"va":5374637946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t914869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8b5f50d00","size":5,"soft_fail":false,"target_va":5375548016,"va":5374633142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_fully_erased_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t189734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e826e50200","size":5,"soft_fail":false,"target_va":5374827648,"va":5374637909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t185859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e803d60200","size":5,"soft_fail":false,"target_va":5374826656,"va":5374640792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t187006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e87eda0200","size":5,"soft_fail":false,"target_va":5374824624,"va":5374637613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t194895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e84ff90200","size":5,"soft_fail":false,"target_va":5374828608,"va":5374633708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t190533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e845e80200","size":5,"soft_fail":false,"target_va":5374823840,"va":5374633302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_rewiring_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t181670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8a6c50200","size":5,"soft_fail":false,"target_va":5374824240,"va":5374642565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t187532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e88cdc0200","size":5,"soft_fail":false,"target_va":5374825984,"va":5374638447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::rewire_enclosed_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t501442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8c2a60700","size":5,"soft_fail":false,"target_va":5375138336,"va":5374636889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2711837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e81d612900","size":5,"soft_fail":false,"target_va":5377353776,"va":5374641934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1722397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e81d481a00","size":5,"soft_fail":false,"target_va":5376356912,"va":5374634510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1726283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e84b571a00","size":5,"soft_fail":false,"target_va":5376360560,"va":5374634272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1722673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e831491a00","size":5,"soft_fail":false,"target_va":5376356144,"va":5374633466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2726217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e849992900","size":5,"soft_fail":false,"target_va":5377366224,"va":5374640002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3332522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e8aad93200","size":5,"soft_fail":false,"target_va":5377968480,"va":5374635953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2303820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374632560,"raw_hex":"e84c272300","size":5,"soft_fail":false,"target_va":5376939952,"va":5374636127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14116,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":24580,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19652,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":20868,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":11372,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":11428,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=23
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":423,"record_offset":4744,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":423,"record_offset":6744,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14444,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":11276,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":22800,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":26920,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5240,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":20408,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":423,"record_offset":18756,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":423,"record_offset":10748,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":423,"record_offset":22448,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22333,"source_line":117,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":423,"record_offset":24772,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24551,"source_line":21,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::StructureType::is_fence
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":423,"record_offset":20576,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22414,"source_line":100,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":423,"record_offset":22596,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22419,"source_line":128,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":423,"record_offset":22480,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22411,"source_line":38,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":423,"record_offset":17300,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":34504,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22068,"source_line":100,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":11816,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22067,"source_line":105,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":23824,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24545,"source_line":62,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":22564,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":23484,"source_line":125,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":28156,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":28240,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24554,"source_line":173,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":10804,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24531,"source_line":36,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":4804,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22951,"source_line":79,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":5120,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22964,"source_line":238,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":4888,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22959,"source_line":248,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":423,"record_offset":11576,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":23495,"source_line":61,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::affected_walls
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":423,"record_offset":7364,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24529,"source_line":39,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::has_anything_been_affected
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":423,"record_offset":6660,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24528,"source_line":43,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":423,"record_offset":14736,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24536,"source_line":195,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":423,"record_offset":22208,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24544,"source_line":484,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":423,"record_offset":24464,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24547,"source_line":17,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":16824,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":14648,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":9700,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":6072,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":8688,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":7772,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24530,"source_line":2357,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":16312,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":16916,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":21780,"source_line":664,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":20964,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14088,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::segments_with_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14144,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24532,"source_line":122,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":15016,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::max
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":14388,"parent_record_offset":4396,"ancestor_record_offset":4396,"inlinee":24534,"source_line":26,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17796,"parent_record_offset":17772,"ancestor_record_offset":4396,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":4396,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":5664,"parent_record_offset":5636,"ancestor_record_offset":4396,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17772,"parent_record_offset":17664,"ancestor_record_offset":4396,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":5636,"parent_record_offset":5420,"ancestor_record_offset":4396,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":18588,"parent_record_offset":18560,"ancestor_record_offset":4396,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":4396,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17664,"parent_record_offset":17628,"ancestor_record_offset":4396,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17628,"parent_record_offset":17592,"ancestor_record_offset":4396,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":17892,"parent_record_offset":17628,"ancestor_record_offset":4396,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":4396,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17548,"parent_record_offset":17508,"ancestor_record_offset":4396,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17508,"parent_record_offset":17468,"ancestor_record_offset":4396,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":17468,"parent_record_offset":17428,"ancestor_record_offset":4396,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":6900,"parent_record_offset":6860,"ancestor_record_offset":4396,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19716,"parent_record_offset":19680,"ancestor_record_offset":4396,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":6860,"parent_record_offset":6824,"ancestor_record_offset":4396,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19680,"parent_record_offset":19652,"ancestor_record_offset":4396,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":4396,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17592,"parent_record_offset":17548,"ancestor_record_offset":4396,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17428,"parent_record_offset":17368,"ancestor_record_offset":4396,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":16368,"parent_record_offset":16312,"ancestor_record_offset":4396,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":4396,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19776,"parent_record_offset":19744,"ancestor_record_offset":4396,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":4396,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19744,"parent_record_offset":19680,"ancestor_record_offset":4396,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":26076,"parent_record_offset":26052,"ancestor_record_offset":4396,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":26640,"parent_record_offset":26616,"ancestor_record_offset":4396,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":23220,"parent_record_offset":23180,"ancestor_record_offset":4396,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=33
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":423,"record_offset":4772,"parent_record_offset":4744,"ancestor_record_offset":4396,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":423,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":4396,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":423,"record_offset":6824,"parent_record_offset":6744,"ancestor_record_offset":4396,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17708,"parent_record_offset":17664,"ancestor_record_offset":4396,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17736,"parent_record_offset":17708,"ancestor_record_offset":4396,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":4396,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":25072,"parent_record_offset":25044,"ancestor_record_offset":4396,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":4396,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":4396,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10040,"parent_record_offset":10012,"ancestor_record_offset":4396,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10072,"parent_record_offset":10040,"ancestor_record_offset":4396,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10244,"parent_record_offset":10072,"ancestor_record_offset":4396,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":423,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":4396,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":4396,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":24492,"parent_record_offset":24464,"ancestor_record_offset":4396,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":15100,"parent_record_offset":15076,"ancestor_record_offset":4396,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":4396,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":4396,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":4396,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":7648,"parent_record_offset":7620,"ancestor_record_offset":4396,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":4396,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":7708,"parent_record_offset":7676,"ancestor_record_offset":4396,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":22244,"parent_record_offset":22208,"ancestor_record_offset":4396,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":4396,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":22416,"parent_record_offset":22208,"ancestor_record_offset":4396,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":423,"record_offset":28524,"parent_record_offset":28264,"ancestor_record_offset":4396,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":423,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":4396,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":423,"record_offset":14340,"parent_record_offset":14316,"ancestor_record_offset":4396,"inlinee":24533,"source_line":91,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":423,"record_offset":14496,"parent_record_offset":14468,"ancestor_record_offset":4396,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14316,"parent_record_offset":14292,"ancestor_record_offset":4396,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14268,"parent_record_offset":14244,"ancestor_record_offset":4396,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14524,"parent_record_offset":14496,"ancestor_record_offset":4396,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":22848,"parent_record_offset":22824,"ancestor_record_offset":4396,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":4396,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14292,"parent_record_offset":14268,"ancestor_record_offset":4396,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":4396,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":423,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":4396,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":14244,"parent_record_offset":14144,"ancestor_record_offset":4396,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":20032,"parent_record_offset":19680,"ancestor_record_offset":4396,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":10176,"parent_record_offset":10152,"ancestor_record_offset":4396,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":5512,"parent_record_offset":5484,"ancestor_record_offset":4396,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":25596,"parent_record_offset":25536,"ancestor_record_offset":4396,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4396,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":14216,"parent_record_offset":14144,"ancestor_record_offset":4396,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":18616,"parent_record_offset":18588,"ancestor_record_offset":4396,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":423,"record_offset":6296,"parent_record_offset":6200,"ancestor_record_offset":4396,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":423,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":4396,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":16696,"parent_record_offset":16636,"ancestor_record_offset":4396,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":16724,"parent_record_offset":16636,"ancestor_record_offset":4396,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":14676,"parent_record_offset":14648,"ancestor_record_offset":4396,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":15076,"parent_record_offset":15052,"ancestor_record_offset":4396,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":15052,"parent_record_offset":15016,"ancestor_record_offset":4396,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":4396,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":4964,"parent_record_offset":4912,"ancestor_record_offset":4396,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":9284,"parent_record_offset":8824,"ancestor_record_offset":4396,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":9984,"parent_record_offset":9808,"ancestor_record_offset":4396,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":12848,"parent_record_offset":11816,"ancestor_record_offset":4396,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":10772,"parent_record_offset":10748,"ancestor_record_offset":4396,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":23312,"parent_record_offset":23180,"ancestor_record_offset":4396,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":6608,"parent_record_offset":6580,"ancestor_record_offset":4396,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":17932,"parent_record_offset":17548,"ancestor_record_offset":4396,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5268,"parent_record_offset":5240,"ancestor_record_offset":4396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":4396,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":4396,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":5576,"parent_record_offset":5452,"ancestor_record_offset":4396,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":11544,"parent_record_offset":11428,"ancestor_record_offset":4396,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":4396,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":11488,"parent_record_offset":11428,"ancestor_record_offset":4396,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":4396,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":9244,"parent_record_offset":9212,"ancestor_record_offset":4396,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":4396,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":4396,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":6956,"parent_record_offset":6860,"ancestor_record_offset":4396,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":423,"record_offset":10708,"parent_record_offset":9772,"ancestor_record_offset":4396,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":4396,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":14864,"parent_record_offset":14736,"ancestor_record_offset":4396,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":16396,"parent_record_offset":16368,"ancestor_record_offset":4396,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":4396,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":16424,"parent_record_offset":16396,"ancestor_record_offset":4396,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":4396,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":11692,"parent_record_offset":11628,"ancestor_record_offset":4396,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":4396,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":4396,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":11628,"parent_record_offset":11600,"ancestor_record_offset":4396,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":8988,"parent_record_offset":8880,"ancestor_record_offset":4396,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":423,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":4396,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":423,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":4396,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":423,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":4396,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":423,"record_offset":8824,"parent_record_offset":8792,"ancestor_record_offset":4396,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":423,"record_offset":8768,"parent_record_offset":8744,"ancestor_record_offset":4396,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":423,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":4396,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":423,"record_offset":8744,"parent_record_offset":8720,"ancestor_record_offset":4396,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":423,"record_offset":9316,"parent_record_offset":8792,"ancestor_record_offset":4396,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6396,"parent_record_offset":6364,"ancestor_record_offset":4396,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6556,"parent_record_offset":6396,"ancestor_record_offset":4396,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":4396,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6108,"parent_record_offset":6072,"ancestor_record_offset":4396,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":4396,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6172,"parent_record_offset":6136,"ancestor_record_offset":4396,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":423,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":4396,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":423,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":4396,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":423,"record_offset":9128,"parent_record_offset":8880,"ancestor_record_offset":4396,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":423,"record_offset":9100,"parent_record_offset":8880,"ancestor_record_offset":4396,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":423,"record_offset":8932,"parent_record_offset":8880,"ancestor_record_offset":4396,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":28436,"parent_record_offset":28412,"ancestor_record_offset":4396,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":28460,"parent_record_offset":28436,"ancestor_record_offset":4396,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":423,"record_offset":14780,"parent_record_offset":14736,"ancestor_record_offset":4396,"inlinee":24537,"source_line":35,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":423,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":4396,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_try_update_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":28264,"parent_record_offset":28240,"ancestor_record_offset":4396,"inlinee":24555,"source_line":185,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_wall_state_enclosure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":423,"record_offset":28384,"parent_record_offset":28264,"ancestor_record_offset":4396,"inlinee":24557,"source_line":174,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4396,"inlinee":22952,"source_line":70,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":5148,"parent_record_offset":5120,"ancestor_record_offset":4396,"inlinee":22965,"source_line":134,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4396,"inlinee":22961,"source_line":152,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::is_active
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":423,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4396,"inlinee":22960,"source_line":167,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":423,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":4396,"inlinee":22954,"source_line":170,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::animate_wall_bulldoze::impl$1::contains::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":423,"record_offset":11752,"parent_record_offset":11600,"ancestor_record_offset":4396,"inlinee":23483,"source_line":62,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":423,"record_offset":19800,"parent_record_offset":19776,"ancestor_record_offset":4396,"inlinee":24542,"source_line":287,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":17300,"ancestor_record_offset":4396,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":16848,"parent_record_offset":16824,"ancestor_record_offset":4396,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":4396,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":4396,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":423,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":4396,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":4396,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":4396,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":4396,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":7448,"parent_record_offset":7420,"ancestor_record_offset":4396,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":12584,"parent_record_offset":12364,"ancestor_record_offset":4396,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":4396,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":423,"record_offset":11928,"parent_record_offset":11896,"ancestor_record_offset":4396,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":4396,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12472,"parent_record_offset":12364,"ancestor_record_offset":4396,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":4396,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8196,"parent_record_offset":7888,"ancestor_record_offset":4396,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12792,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7564,"parent_record_offset":7504,"ancestor_record_offset":4396,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7888,"parent_record_offset":7856,"ancestor_record_offset":4396,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":4396,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":4396,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11956,"parent_record_offset":11896,"ancestor_record_offset":4396,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7476,"parent_record_offset":7448,"ancestor_record_offset":4396,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5452,"parent_record_offset":5420,"ancestor_record_offset":4396,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11008,"parent_record_offset":10944,"ancestor_record_offset":4396,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7536,"parent_record_offset":7504,"ancestor_record_offset":4396,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4396,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":4396,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5420,"parent_record_offset":5364,"ancestor_record_offset":4396,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5392,"parent_record_offset":5364,"ancestor_record_offset":4396,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":4396,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5484,"parent_record_offset":5452,"ancestor_record_offset":4396,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":423,"record_offset":7968,"parent_record_offset":7936,"ancestor_record_offset":4396,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":423,"record_offset":8000,"parent_record_offset":7968,"ancestor_record_offset":4396,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":423,"record_offset":12332,"parent_record_offset":12216,"ancestor_record_offset":4396,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":423,"record_offset":7936,"parent_record_offset":7888,"ancestor_record_offset":4396,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12048,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":4396,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":4396,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":423,"record_offset":12364,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":423,"record_offset":12076,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":423,"record_offset":7620,"parent_record_offset":7564,"ancestor_record_offset":4396,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":423,"record_offset":12160,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":423,"record_offset":12700,"parent_record_offset":12004,"ancestor_record_offset":4396,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":423,"record_offset":7676,"parent_record_offset":7620,"ancestor_record_offset":4396,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":423,"record_offset":24428,"parent_record_offset":24404,"ancestor_record_offset":4396,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":423,"record_offset":22276,"parent_record_offset":22208,"ancestor_record_offset":4396,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":423,"record_offset":28552,"parent_record_offset":28264,"ancestor_record_offset":4396,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":423,"record_offset":28412,"parent_record_offset":28264,"ancestor_record_offset":4396,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":9736,"parent_record_offset":9700,"ancestor_record_offset":4396,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":423,"record_offset":9848,"parent_record_offset":9808,"ancestor_record_offset":4396,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":10012,"parent_record_offset":9808,"ancestor_record_offset":4396,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":423,"record_offset":10280,"parent_record_offset":10012,"ancestor_record_offset":4396,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":11848,"parent_record_offset":11816,"ancestor_record_offset":4396,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":28300,"parent_record_offset":28264,"ancestor_record_offset":4396,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":23856,"parent_record_offset":23824,"ancestor_record_offset":4396,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":4396,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":4396,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":7392,"parent_record_offset":7364,"ancestor_record_offset":4396,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":7800,"parent_record_offset":7772,"ancestor_record_offset":4396,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":24800,"parent_record_offset":24772,"ancestor_record_offset":4396,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":423,"record_offset":24404,"parent_record_offset":23824,"ancestor_record_offset":4396,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":423,"record_offset":16880,"parent_record_offset":16848,"ancestor_record_offset":4396,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":423,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":4396,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":423,"record_offset":24924,"parent_record_offset":24896,"ancestor_record_offset":4396,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":423,"record_offset":25044,"parent_record_offset":24896,"ancestor_record_offset":4396,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":423,"record_offset":24824,"parent_record_offset":24800,"ancestor_record_offset":4396,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":423,"record_offset":9876,"parent_record_offset":9848,"ancestor_record_offset":4396,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":423,"record_offset":9908,"parent_record_offset":9876,"ancestor_record_offset":4396,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":423,"record_offset":9808,"parent_record_offset":9772,"ancestor_record_offset":4396,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":423,"record_offset":9772,"parent_record_offset":9736,"ancestor_record_offset":4396,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14180,"parent_record_offset":14144,"ancestor_record_offset":4396,"inlinee":23578,"source_line":109,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":16340,"parent_record_offset":16312,"ancestor_record_offset":4396,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":16464,"parent_record_offset":16312,"ancestor_record_offset":4396,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":17144,"parent_record_offset":16916,"ancestor_record_offset":4396,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":19828,"parent_record_offset":19800,"ancestor_record_offset":4396,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":4396,"inlinee":24539,"source_line":6,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":4396,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":17368,"parent_record_offset":17300,"ancestor_record_offset":4396,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":423,"record_offset":16588,"parent_record_offset":16312,"ancestor_record_offset":4396,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD8
// 首条位置: {"module":423,"inlinee":24536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14892,"parent_record_offset":14736,"ancestor_record_offset":14736,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":14864,"parent_record_offset":14736,"ancestor_record_offset":14736,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$11::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":423,"record_offset":14780,"parent_record_offset":14736,"ancestor_record_offset":14736,"inlinee":24537,"source_line":35,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":14736,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14924,"parent_record_offset":14892,"ancestor_record_offset":14736,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":14736,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":14980,"parent_record_offset":14892,"ancestor_record_offset":14736,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":14736,"inlinee":24539,"source_line":6,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDE
// 首条位置: {"module":423,"inlinee":24542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":19828,"parent_record_offset":19800,"ancestor_record_offset":19800,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF045
// 首条位置: {"module":1308,"inlinee":61509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":145872,"parent_record_offset":145836,"ancestor_record_offset":145836,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":145896,"parent_record_offset":145872,"ancestor_record_offset":145836,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":145944,"parent_record_offset":145920,"ancestor_record_offset":145836,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1308,"record_offset":145920,"parent_record_offset":145896,"ancestor_record_offset":145836,"inlinee":24549,"source_line":6,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FE0
// 首条位置: {"module":423,"inlinee":24544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":22244,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":22416,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":423,"record_offset":22276,"parent_record_offset":22208,"ancestor_record_offset":22208,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":22328,"parent_record_offset":22276,"ancestor_record_offset":22208,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":22384,"parent_record_offset":22276,"ancestor_record_offset":22208,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":22300,"parent_record_offset":22276,"ancestor_record_offset":22208,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00423.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_eraser.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_eraser.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_eraser.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 信息 ID: 8363ceb1fd2059b81e1af2f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8ce6fb2ac42460f8ef57ae13: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> > @ 0xd2b14 -> 0x5a6270 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x141ace -> 0x5a6270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 信息 ID: 759712953f111baeea65493c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=14736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=16028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 信息 ID: cfdd7a8f33a4a59932748e6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=19800; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=19964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 信息 ID: b63e7991541500e01d16731d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; module=1308, inline_record=145836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 信息 ID: 9af7d22f463da4a9ad84017a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=22208; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=23500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
