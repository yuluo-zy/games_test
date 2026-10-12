// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/save/snapshot.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ade0345a8072fd0c2536d5fb9b4470295fa4685c63e84fd3441093d1200d7eb9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 424, 469, 1308, 1321, 1438
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"PDB_registry_path_only","version":"1.16.0"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"PDB_registry_path_only","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use flate2; // namespace candidate
//   package identities: [{"name":"flate2","source":"PDB_registry_path_only","version":"1.0.24"}]
// use itoa; // namespace candidate
//   package identities: [{"name":"itoa","source":"PDB_registry_path_only","version":"1.0.3"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"PDB_registry_path_only","version":"0.5.8"}]
// use motomoto; // namespace candidate
//   package identities: []
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"PDB_registry_path_only","version":"1.9.0"}]
// use ryu; // namespace candidate
//   package identities: [{"name":"ryu","source":"PDB_registry_path_only","version":"1.0.11"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1438,"record_offset":2772,"rva":17448608,"va":5386157728}
// 编译实例 RVAs: 0x10a3ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=9
//     target_aliases=89 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13851629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e813a42cff","size":5,"soft_fail":false,"target_va":5372314208,"va":5386165832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13849919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8c1aa2cff","size":5,"soft_fail":false,"target_va":5372320512,"va":5386170426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13408529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8ef6633ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386158220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=56
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13408973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8336533ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386158680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11923880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8580e4aff","size":5,"soft_fail":false,"target_va":5374241680,"va":5386165555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11926520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e808044aff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386168787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546fa0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11915699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e84d2e4aff","size":5,"soft_fail":false,"target_va":5374242720,"va":5386158414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593480
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11611383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e809d34eff","size":5,"soft_fail":false,"target_va":5374555264,"va":5386166642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11609755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e865d94eff","size":5,"soft_fail":false,"target_va":5374558176,"va":5386167926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e0c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11287441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e86fc453ff","size":5,"soft_fail":false,"target_va":5374872608,"va":5386160044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607710
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11133608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8581d56ff","size":5,"soft_fail":false,"target_va":5375031056,"va":5386164659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11131342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8322656ff","size":5,"soft_fail":false,"target_va":5375032528,"va":5386163865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11133440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8001e56ff","size":5,"soft_fail":false,"target_va":5375032896,"va":5386166331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8860
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10281345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e87f1e63ff","size":5,"soft_fail":false,"target_va":5375887456,"va":5386168796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10266852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e81c5763ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386158719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d9c20
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10276164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8bc3263ff","size":5,"soft_fail":false,"target_va":5375892512,"va":5386168671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de510
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10255599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8118363ff","size":5,"soft_fail":false,"target_va":5375911184,"va":5386166778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406df640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10259573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e88b7363ff","size":5,"soft_fail":false,"target_va":5375915584,"va":5386175152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e27c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10241624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8a8b963ff","size":5,"soft_fail":false,"target_va":5375928256,"va":5386169875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=22
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10228403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e84ded63ff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386158558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10186965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e82b8f64ff","size":5,"soft_fail":false,"target_va":5375988576,"va":5386175536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d77330
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3332438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8aa26cdff","size":5,"soft_fail":false,"target_va":5382828848,"va":5386161281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d79d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3318507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8155dcdff","size":5,"soft_fail":false,"target_va":5382839696,"va":5386158198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7fd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3295839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8a1b5cdff","size":5,"soft_fail":false,"target_va":5382864160,"va":5386159994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d81b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3286949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e85bd8cdff","size":5,"soft_fail":false,"target_va":5382871936,"va":5386158880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f25bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1570649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8a708e8ff","size":5,"soft_fail":false,"target_va":5384592304,"va":5386162948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141055620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-330662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e85af4faff","size":5,"soft_fail":false,"target_va":5385836064,"va":5386166721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410906a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-81052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e864c3feff","size":5,"soft_fail":false,"target_va":5386077856,"va":5386158903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141090ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e87ac4feff","size":5,"soft_fail":false,"target_va":5386079968,"va":5386160737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410910e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e805bcfeff","size":5,"soft_fail":false,"target_va":5386080480,"va":5386163414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410913b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-78215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e879cefeff","size":5,"soft_fail":false,"target_va":5386081200,"va":5386159410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141092270
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-76333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8d3d5feff","size":5,"soft_fail":false,"target_va":5386084976,"va":5386161304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141100d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t373964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8ccb40500","size":5,"soft_fail":false,"target_va":5386538272,"va":5386164303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141101d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t378326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8d6c50500","size":5,"soft_fail":false,"target_va":5386542416,"va":5386164085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411050e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t390037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e895f30500","size":5,"soft_fail":false,"target_va":5386555616,"va":5386165574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420caee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16927389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e89d4a0201","size":5,"soft_fail":false,"target_va":5403094752,"va":5386167358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cb2f0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16931723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e88b5b0201","size":5,"soft_fail":false,"target_va":5403095792,"va":5386164064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cb380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16930070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e816550201","size":5,"soft_fail":false,"target_va":5403095936,"va":5386165861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17428897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8a1f10901","size":5,"soft_fail":false,"target_va":5403593360,"va":5386164458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17429370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e87af30901","size":5,"soft_fail":false,"target_va":5403593392,"va":5386164017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t16809310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e85e7d0001","size":5,"soft_fail":false,"target_va":5402978144,"va":5386168829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t16755073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e881a9ff00","size":5,"soft_fail":false,"target_va":5402919072,"va":5386163994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16809233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8117d0001","size":5,"soft_fail":false,"target_va":5402978880,"va":5386169642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t16811172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8a4840001","size":5,"soft_fail":false,"target_va":5402980128,"va":5386168951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] flate2::mem::Compress::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-327526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e89a00fbff","size":5,"soft_fail":false,"target_va":5385838992,"va":5386166513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8804114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8eea879ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386157885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] motomoto::tag_from_str_trunc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t397610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e82a110600","size":5,"soft_fail":false,"target_va":5386563920,"va":5386166305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ryu::pretty::format64
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10138117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e8fb4d65ff","size":5,"soft_fail":false,"target_va":5376027264,"va":5386165376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t16795723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e84b480001","size":5,"soft_fail":false,"target_va":5402960048,"va":5386164320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8771709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386157728,"raw_hex":"e883277aff","size":5,"soft_fail":false,"target_va":5377395552,"va":5386167256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":3204,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":3528,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":33980,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1438,"record_offset":3232,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1438,"record_offset":32896,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1438,"record_offset":32648,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1438,"record_offset":3120,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1438,"record_offset":3856,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1438,"record_offset":3148,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1438,"record_offset":31792,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":3444,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":4488,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":4432,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":25044,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":31624,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::GateColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1438,"record_offset":18072,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":26981,"source_line":134,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::HalftimberColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1438,"record_offset":20204,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":66541,"source_line":194,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1438,"record_offset":15940,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":66539,"source_line":75,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1438,"record_offset":34736,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":59409,"source_line":147,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorage::take_snapshot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1438,"record_offset":10248,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":24805,"source_line":384,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::all
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1438,"record_offset":6052,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":26486,"source_line":96,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster_defs::AllWorldRasterRead::read
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1438,"record_offset":31680,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":23643,"source_line":601,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::write::DeflateEncoder::finish
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/deflate/write.rs
//     evidence={"module":1438,"record_offset":34092,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":24125,"source_line":115,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::write::DeflateEncoder::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/deflate/write.rs
//     evidence={"module":1438,"record_offset":33616,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":24124,"source_line":40,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1438,"record_offset":3176,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1438,"record_offset":3036,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] motomoto::Writer::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1438,"record_offset":3800,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":25802,"source_line":74,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":4460,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":9687,"source_line":125,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":3828,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":9688,"source_line":38,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":25076,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1438,"record_offset":6588,"parent_record_offset":2772,"ancestor_record_offset":2772,"inlinee":7847,"source_line":940,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":2772,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":2772,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=67
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":2772,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":2772,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=67
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":4676,"parent_record_offset":4620,"ancestor_record_offset":2772,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":29740,"parent_record_offset":29716,"ancestor_record_offset":2772,"inlinee":6430,"source_line":2379,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":2772,"inlinee":11269,"source_line":2502,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":5336,"parent_record_offset":5308,"ancestor_record_offset":2772,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":5396,"parent_record_offset":5336,"ancestor_record_offset":2772,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":25524,"parent_record_offset":25500,"ancestor_record_offset":2772,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":4648,"parent_record_offset":4620,"ancestor_record_offset":2772,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":25636,"parent_record_offset":25608,"ancestor_record_offset":2772,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":2772,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":25608,"parent_record_offset":25584,"ancestor_record_offset":2772,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":2772,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":2772,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":3760,"parent_record_offset":3584,"ancestor_record_offset":2772,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=67
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":4620,"parent_record_offset":4592,"ancestor_record_offset":2772,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1438,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":2772,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1438,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":2772,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1438,"record_offset":3984,"parent_record_offset":3940,"ancestor_record_offset":2772,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1438,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":2772,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1438,"record_offset":3940,"parent_record_offset":3912,"ancestor_record_offset":2772,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1438,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":2772,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1438,"record_offset":27884,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1438,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":2772,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":2772,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":25500,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":6979,"source_line":921,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":13564,"parent_record_offset":13540,"ancestor_record_offset":2772,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":25556,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":25584,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":25844,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":25732,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":7909,"source_line":2381,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":13268,"parent_record_offset":13228,"ancestor_record_offset":2772,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":2772,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2772,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":2772,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":4780,"parent_record_offset":4748,"ancestor_record_offset":2772,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1438,"record_offset":13540,"parent_record_offset":13228,"ancestor_record_offset":2772,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1438,"record_offset":13200,"parent_record_offset":13172,"ancestor_record_offset":2772,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1438,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":2772,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1438,"record_offset":34052,"parent_record_offset":34016,"ancestor_record_offset":2772,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":34016,"parent_record_offset":33980,"ancestor_record_offset":2772,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1438,"record_offset":3280,"parent_record_offset":3256,"ancestor_record_offset":2772,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":1438,"record_offset":3316,"parent_record_offset":3280,"ancestor_record_offset":2772,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1438,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":2772,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1438,"record_offset":34760,"parent_record_offset":34736,"ancestor_record_offset":2772,"inlinee":15086,"source_line":354,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":34784,"parent_record_offset":34760,"ancestor_record_offset":2772,"inlinee":15063,"source_line":253,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":34808,"parent_record_offset":34784,"ancestor_record_offset":2772,"inlinee":15064,"source_line":353,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1438,"record_offset":30052,"parent_record_offset":30012,"ancestor_record_offset":2772,"inlinee":25786,"source_line":1367,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1438,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":2772,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1438,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":2772,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1438,"record_offset":32744,"parent_record_offset":32676,"ancestor_record_offset":2772,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1438,"record_offset":6344,"parent_record_offset":6316,"ancestor_record_offset":2772,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1438,"record_offset":6404,"parent_record_offset":6372,"ancestor_record_offset":2772,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1438,"record_offset":26864,"parent_record_offset":26820,"ancestor_record_offset":2772,"inlinee":4262,"source_line":635,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1438,"record_offset":26892,"parent_record_offset":26864,"ancestor_record_offset":2772,"inlinee":11274,"source_line":501,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::partial_classify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":1438,"record_offset":26920,"parent_record_offset":26864,"ancestor_record_offset":2772,"inlinee":4263,"source_line":670,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1438,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":2772,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1438,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":2772,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":1438,"record_offset":13660,"parent_record_offset":13636,"ancestor_record_offset":2772,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1438,"record_offset":13612,"parent_record_offset":13588,"ancestor_record_offset":2772,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1438,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":2772,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1438,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2772,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":10776,"parent_record_offset":10248,"ancestor_record_offset":2772,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":13588,"parent_record_offset":13564,"ancestor_record_offset":2772,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":13780,"parent_record_offset":13756,"ancestor_record_offset":2772,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":2772,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":5428,"parent_record_offset":5336,"ancestor_record_offset":2772,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":34240,"parent_record_offset":34204,"ancestor_record_offset":2772,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":2772,"inlinee":7725,"source_line":342,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":25468,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":2772,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":25660,"parent_record_offset":25636,"ancestor_record_offset":2772,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1438,"record_offset":27008,"parent_record_offset":26980,"ancestor_record_offset":2772,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1438,"record_offset":32716,"parent_record_offset":32676,"ancestor_record_offset":2772,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1438,"record_offset":32676,"parent_record_offset":32648,"ancestor_record_offset":2772,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":1438,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":2772,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":2772,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":10708,"parent_record_offset":10276,"ancestor_record_offset":2772,"inlinee":24722,"source_line":1311,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":34204,"parent_record_offset":34176,"ancestor_record_offset":2772,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1438,"record_offset":6288,"parent_record_offset":6256,"ancestor_record_offset":2772,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1438,"record_offset":4344,"parent_record_offset":4068,"ancestor_record_offset":2772,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1438,"record_offset":26344,"parent_record_offset":26196,"ancestor_record_offset":2772,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=179
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":2772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2772,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":2772,"inlinee":25787,"source_line":1436,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":5364,"parent_record_offset":5336,"ancestor_record_offset":2772,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":3716,"parent_record_offset":3688,"ancestor_record_offset":2772,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":13828,"parent_record_offset":13804,"ancestor_record_offset":2772,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":26980,"parent_record_offset":26820,"ancestor_record_offset":2772,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":34144,"parent_record_offset":34092,"ancestor_record_offset":2772,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":25788,"parent_record_offset":25420,"ancestor_record_offset":2772,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":32768,"parent_record_offset":32744,"ancestor_record_offset":2772,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":2772,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":32792,"parent_record_offset":32768,"ancestor_record_offset":2772,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1438,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2772,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1438,"record_offset":25812,"parent_record_offset":25788,"ancestor_record_offset":2772,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2772,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2772,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1438,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":2772,"inlinee":24754,"source_line":177,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::composite
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1438,"record_offset":31704,"parent_record_offset":31680,"ancestor_record_offset":2772,"inlinee":23615,"source_line":153,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] flate2::zio::Writer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/zio.rs
//     evidence={"module":1438,"record_offset":33656,"parent_record_offset":33616,"ancestor_record_offset":2772,"inlinee":24105,"source_line":162,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] flate2::zio::Writer::take_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/zio.rs
//     evidence={"module":1438,"record_offset":34176,"parent_record_offset":34092,"ancestor_record_offset":2772,"inlinee":24106,"source_line":199,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1438,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":2772,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":6256,"parent_record_offset":6164,"ancestor_record_offset":2772,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":2772,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":6228,"parent_record_offset":6164,"ancestor_record_offset":2772,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":6200,"parent_record_offset":6164,"ancestor_record_offset":2772,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":2772,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1438,"record_offset":6316,"parent_record_offset":6256,"ancestor_record_offset":2772,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1438,"record_offset":6372,"parent_record_offset":6316,"ancestor_record_offset":2772,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] itoa::Buffer::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itoa","version":"1.0.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1438,"record_offset":26164,"parent_record_offset":26128,"ancestor_record_offset":2772,"inlinee":4281,"source_line":86,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$16::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1438,"record_offset":26196,"parent_record_offset":26164,"ancestor_record_offset":2772,"inlinee":11265,"source_line":125,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$18::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1438,"record_offset":27292,"parent_record_offset":27260,"ancestor_record_offset":2772,"inlinee":4283,"source_line":125,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::ChunkWriter::seek_back_and_write_length
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1438,"record_offset":30012,"parent_record_offset":29980,"ancestor_record_offset":2772,"inlinee":25806,"source_line":114,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::impl$3::write
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1438,"record_offset":25336,"parent_record_offset":25312,"ancestor_record_offset":2772,"inlinee":25784,"source_line":104,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::impl$5::drop
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1438,"record_offset":29980,"parent_record_offset":29952,"ancestor_record_offset":2772,"inlinee":28017,"source_line":132,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::Buffer::format_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1438,"record_offset":27068,"parent_record_offset":27040,"ancestor_record_offset":2772,"inlinee":4288,"source_line":72,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::impl$7::write_to_ryu_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1438,"record_offset":27092,"parent_record_offset":27068,"ancestor_record_offset":2772,"inlinee":4290,"source_line":167,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1438,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":2772,"inlinee":5032,"source_line":499,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$89::serialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1438,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":2772,"inlinee":5017,"source_line":200,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::Map::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":28760,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":11268,"source_line":226,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$15::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":29716,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":11271,"source_line":827,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$18::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":29772,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":11273,"source_line":458,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$6::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1438,"record_offset":26040,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":11261,"source_line":316,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":28856,"parent_record_offset":28816,"ancestor_record_offset":2772,"inlinee":5679,"source_line":1849,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":29600,"parent_record_offset":28816,"ancestor_record_offset":2772,"inlinee":5680,"source_line":1859,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":28024,"parent_record_offset":28000,"ancestor_record_offset":2772,"inlinee":11267,"source_line":1608,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":27040,"parent_record_offset":26820,"ancestor_record_offset":2772,"inlinee":4286,"source_line":1721,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":26128,"parent_record_offset":26096,"ancestor_record_offset":2772,"inlinee":11263,"source_line":1655,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":25288,"parent_record_offset":25264,"ancestor_record_offset":2772,"inlinee":4265,"source_line":1599,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":27224,"parent_record_offset":27192,"ancestor_record_offset":2772,"inlinee":4279,"source_line":1699,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":25132,"parent_record_offset":25076,"ancestor_record_offset":2772,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":2772,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":28000,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":11266,"source_line":77,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":26820,"parent_record_offset":26040,"ancestor_record_offset":2772,"inlinee":4260,"source_line":193,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":26096,"parent_record_offset":26040,"ancestor_record_offset":2772,"inlinee":11262,"source_line":113,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":28816,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":5678,"source_line":386,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":27912,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":5692,"source_line":219,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":27192,"parent_record_offset":26040,"ancestor_record_offset":2772,"inlinee":4278,"source_line":157,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":25264,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":9492,"source_line":235,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1438,"record_offset":29800,"parent_record_offset":25188,"ancestor_record_offset":2772,"inlinee":5700,"source_line":720,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$0::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1438,"record_offset":25188,"parent_record_offset":25076,"ancestor_record_offset":2772,"inlinee":11324,"source_line":17,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":2772,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":1438,"record_offset":25312,"parent_record_offset":25288,"ancestor_record_offset":2772,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$7::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1438,"record_offset":25360,"parent_record_offset":25336,"ancestor_record_offset":2772,"inlinee":24842,"source_line":552,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1438,"record_offset":25420,"parent_record_offset":25384,"ancestor_record_offset":2772,"inlinee":24839,"source_line":402,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1438,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":2772,"inlinee":24838,"source_line":463,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1438,"record_offset":25876,"parent_record_offset":25384,"ancestor_record_offset":2772,"inlinee":24840,"source_line":445,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x683F
// 首条位置: {"module":469,"inlinee":26687}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1308,"record_offset":2484,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1308,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2432,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1308,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2432,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1308,"record_offset":2836,"parent_record_offset":2800,"ancestor_record_offset":2432,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":2800,"parent_record_offset":2772,"ancestor_record_offset":2432,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":2432,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":2956,"parent_record_offset":2772,"ancestor_record_offset":2432,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1308,"record_offset":2708,"parent_record_offset":2676,"ancestor_record_offset":2432,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1308,"record_offset":2676,"parent_record_offset":2644,"ancestor_record_offset":2432,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1308,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":2432,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1308,"record_offset":2644,"parent_record_offset":2612,"ancestor_record_offset":2432,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1308,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2432,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1308,"record_offset":2548,"parent_record_offset":2516,"ancestor_record_offset":2432,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1308,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":2432,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":2744,"parent_record_offset":2708,"ancestor_record_offset":2432,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1308,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":2432,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1308,"record_offset":2996,"parent_record_offset":2708,"ancestor_record_offset":2432,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1308,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":2432,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::get_param
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1438,"record_offset":71408,"rva":17481936,"va":5386191056}
// 编译实例 RVAs: 0x10ac0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14057ac90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11736183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386191056,"raw_hex":"e889eb4cff","size":5,"soft_fail":false,"target_va":5374454928,"va":5386191106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16785976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386191056,"raw_hex":"e838220001","size":5,"soft_fail":false,"target_va":5402978944,"va":5386192963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$103::get_param
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":71552,"parent_record_offset":71408,"ancestor_record_offset":71408,"inlinee":5851,"source_line":1466,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::_::impl$0::get_param
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1438,"record_offset":75060,"parent_record_offset":71552,"ancestor_record_offset":71408,"inlinee":4392,"source_line":614,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1438,"record_offset":72244,"parent_record_offset":72220,"ancestor_record_offset":71408,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1438,"record_offset":72220,"parent_record_offset":72184,"ancestor_record_offset":71408,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1438,"record_offset":72156,"parent_record_offset":72116,"ancestor_record_offset":71408,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1438,"record_offset":72084,"parent_record_offset":71684,"ancestor_record_offset":71408,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1438,"record_offset":72116,"parent_record_offset":72084,"ancestor_record_offset":71408,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1438,"record_offset":71728,"parent_record_offset":71684,"ancestor_record_offset":71408,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":1438,"record_offset":72184,"parent_record_offset":72116,"ancestor_record_offset":71408,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1438,"record_offset":71776,"parent_record_offset":71752,"ancestor_record_offset":71408,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1438,"record_offset":71752,"parent_record_offset":71728,"ancestor_record_offset":71408,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1438,"record_offset":71868,"parent_record_offset":71804,"ancestor_record_offset":71408,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1438,"record_offset":71952,"parent_record_offset":71928,"ancestor_record_offset":71408,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":71644,"parent_record_offset":71552,"ancestor_record_offset":71408,"inlinee":4382,"source_line":620,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param::closure$0
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":72428,"parent_record_offset":72404,"ancestor_record_offset":71408,"inlinee":4407,"source_line":628,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$81::get_param
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":75092,"parent_record_offset":75060,"ancestor_record_offset":71408,"inlinee":5127,"source_line":1466,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":1438,"record_offset":72312,"parent_record_offset":72288,"ancestor_record_offset":71408,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1438,"record_offset":72288,"parent_record_offset":71684,"ancestor_record_offset":71408,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":1438,"record_offset":71684,"parent_record_offset":71644,"ancestor_record_offset":71408,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":1438,"record_offset":72456,"parent_record_offset":72428,"ancestor_record_offset":71408,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":71888,"parent_record_offset":71868,"ancestor_record_offset":71408,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":71928,"parent_record_offset":71752,"ancestor_record_offset":71408,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":72404,"parent_record_offset":71644,"ancestor_record_offset":71408,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1438,"record_offset":72032,"parent_record_offset":72008,"ancestor_record_offset":71408,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":71804,"parent_record_offset":71776,"ancestor_record_offset":71408,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":71840,"parent_record_offset":71804,"ancestor_record_offset":71408,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":72008,"parent_record_offset":71976,"ancestor_record_offset":71408,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":72336,"parent_record_offset":72312,"ancestor_record_offset":71408,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":72360,"parent_record_offset":72336,"ancestor_record_offset":71408,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1438,"record_offset":71976,"parent_record_offset":71952,"ancestor_record_offset":71408,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::init_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1438,"record_offset":70788,"rva":17481584,"va":5386190704}
// 编译实例 RVAs: 0x10abf70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..change_detection..ResMut$LT$T$GT$$u20$as$u20$bevy_ecs..system..system_param..SystemParam$GT$::init_state
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-1015924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386190704,"raw_hex":"e88c7ff0ff","size":5,"soft_fail":false,"target_va":5385174816,"va":5386190735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$103::init_state
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":70932,"parent_record_offset":70788,"ancestor_record_offset":70788,"inlinee":5850,"source_line":1443,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::_::impl$0::init_state
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1438,"record_offset":71008,"parent_record_offset":70932,"ancestor_record_offset":70788,"inlinee":4341,"source_line":614,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$81::init_state
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":71032,"parent_record_offset":71008,"ancestor_record_offset":70788,"inlinee":5120,"source_line":1443,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$85::init_state
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1438,"record_offset":70976,"parent_record_offset":70956,"ancestor_record_offset":70788,"inlinee":5139,"source_line":1443,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster_defs::_::impl$0::init_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1438,"record_offset":70956,"parent_record_offset":70932,"ancestor_record_offset":70788,"inlinee":23651,"source_line":610,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_chunk
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1A7
// 首条位置: {"module":1321,"inlinee":61863}
// 编译实例 RVAs: 0x10a8620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cb2f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16919648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176032,"raw_hex":"e8602c0201","size":5,"soft_fail":false,"target_va":5403095792,"va":5386176139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e34f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17018474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176032,"raw_hex":"e86aae0301","size":5,"soft_fail":false,"target_va":5403194608,"va":5386176129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":122072,"parent_record_offset":122008,"ancestor_record_offset":122008,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":122044,"parent_record_offset":122008,"ancestor_record_offset":122008,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":122100,"parent_record_offset":122008,"ancestor_record_offset":122008,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1438,"record_offset":44124,"rva":17467056,"va":5386176176}
// 编译实例 RVAs: 0x10a86b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-39882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e83664ffff","size":5,"soft_fail":false,"target_va":5386138016,"va":5386177893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$serde_json..value..index..Index$GT$::index_into
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-638001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8cf43f6ff","size":5,"soft_fail":false,"target_va":5385539984,"va":5386177980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16545674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8768803ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5386178117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14600795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8a53521ff","size":5,"soft_fail":false,"target_va":5371581328,"va":5386182118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13942560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8e0402bff","size":5,"soft_fail":false,"target_va":5372239520,"va":5386182075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13863303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e879762cff","size":5,"soft_fail":false,"target_va":5372314208,"va":5386177506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13863158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e80a772cff","size":5,"soft_fail":false,"target_va":5372320512,"va":5386183665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=30
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13427042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e89e1e33ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386176749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13027746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e85e3639ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5386177581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140505430
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12207387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8e5ba45ff","size":5,"soft_fail":false,"target_va":5373973552,"va":5386180934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140505520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12208198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8bab745ff","size":5,"soft_fail":false,"target_va":5373973792,"va":5386181985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405055d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12207277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e853bb45ff","size":5,"soft_fail":false,"target_va":5373973968,"va":5386181240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140509ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12189049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8870246ff","size":5,"soft_fail":false,"target_va":5373992656,"va":5386181700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11940562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e82ecd49ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5386182829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11925569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8bf074aff","size":5,"soft_fail":false,"target_va":5374252640,"va":5386178204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549f80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11924674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e83e0b4aff","size":5,"soft_fail":false,"target_va":5374254976,"va":5386179645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11626945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e83f964eff","size":5,"soft_fail":false,"target_va":5374555024,"va":5386181964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ae7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11515063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8494b50ff","size":5,"soft_fail":false,"target_va":5374666720,"va":5386181778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11005643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8351158ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5386176694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140651430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10843300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e85c8b5aff","size":5,"soft_fail":false,"target_va":5375333424,"va":5386176719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8860
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10294636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e894ea62ff","size":5,"soft_fail":false,"target_va":5375887456,"va":5386182087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10287612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8040663ff","size":5,"soft_fail":false,"target_va":5375891872,"va":5386179479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dd420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10277706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8b62c63ff","size":5,"soft_fail":false,"target_va":5375906848,"va":5386184549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406df640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10268781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8934f63ff","size":5,"soft_fail":false,"target_va":5375915584,"va":5386184360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10249015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8c99c63ff","size":5,"soft_fail":false,"target_va":5375930160,"va":5386179170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d41fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e82f93c9ff","size":5,"soft_fail":false,"target_va":5382610880,"va":5386177676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3371388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8848eccff","size":5,"soft_fail":false,"target_va":5382810432,"va":5386181815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d73d80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3362943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e881afccff","size":5,"soft_fail":false,"target_va":5382815104,"va":5386178042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14100de80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-634992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8904ff6ff","size":5,"soft_fail":false,"target_va":5385543296,"va":5386178283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14100eb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-632399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8b159f6ff","size":5,"soft_fail":false,"target_va":5385546512,"va":5386178906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14100f470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-631750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e83a5cf6ff","size":5,"soft_fail":false,"target_va":5385548912,"va":5386180657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14100fdd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-630262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e80a62f6ff","size":5,"soft_fail":false,"target_va":5385551312,"va":5386181569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410100f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-627583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8816cf6ff","size":5,"soft_fail":false,"target_va":5385552112,"va":5386179690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141010d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-626100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e84c72f6ff","size":5,"soft_fail":false,"target_va":5385555312,"va":5386181407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141018660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-593906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e80ef0f6ff","size":5,"soft_fail":false,"target_va":5385586272,"va":5386180173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141019470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-591345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e80ffaf6ff","size":5,"soft_fail":false,"target_va":5385589872,"va":5386181212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411071c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t386400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e860e50500","size":5,"soft_fail":false,"target_va":5386564032,"va":5386177627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16913018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e87a120201","size":5,"soft_fail":false,"target_va":5403091664,"va":5386178641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca450
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16914227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e833170201","size":5,"soft_fail":false,"target_va":5403092048,"va":5386177816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cb2f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16918952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8a8290201","size":5,"soft_fail":false,"target_va":5403095792,"va":5386176835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e34f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17016731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e89ba70301","size":5,"soft_fail":false,"target_va":5403194608,"va":5386177872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8646775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8890f7cff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386176658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8811999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8218a79ff","size":5,"soft_fail":false,"target_va":5377364336,"va":5386176330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16795626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8ea470001","size":5,"soft_fail":false,"target_va":5402978880,"va":5386183249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::unwrap_or<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>,alloc::alloc::Global>,serde_json::error::Error>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8b3a2ffff","size":5,"soft_fail":false,"target_va":5386157232,"va":5386181112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::snapshot::try_chunk
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e841e9ffff","size":5,"soft_fail":false,"target_va":5386176032,"va":5386181850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] flate2::mem::Decompress::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-342859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8b5c4faff","size":5,"soft_fail":false,"target_va":5385839040,"va":5386181894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8823461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e85b5d79ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386177232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-38283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8756affff","size":5,"soft_fail":false,"target_va":5386138608,"va":5386176886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10128639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8017365ff","size":5,"soft_fail":false,"target_va":5376048224,"va":5386176858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] motomoto::Reader::iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t386384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e850e50500","size":5,"soft_fail":false,"target_va":5386564000,"va":5386177611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::impl$4::clone
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-23420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e884a4ffff","size":5,"soft_fail":false,"target_va":5386157472,"va":5386180887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-8782173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8a3fe79ff","size":5,"soft_fail":false,"target_va":5377395552,"va":5386177720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8768811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e8d5327aff","size":5,"soft_fail":false,"target_va":5377408016,"va":5386176822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8736499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e80db17aff","size":5,"soft_fail":false,"target_va":5377439776,"va":5386176270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8707763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386176176,"raw_hex":"e84d217bff","size":5,"soft_fail":false,"target_va":5377469680,"va":5386177438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*25107621(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5386176176,"raw_hex":"ff15a51c7f01","size":6,"soft_fail":false,"va":5386176933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1438,"record_offset":56176,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1438,"record_offset":56204,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":44916,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::closest_snapshot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":44360,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":25211,"source_line":342,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::remove_snapshot_record
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":56260,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":25581,"source_line":382,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":44944,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":25569,"source_line":74,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_load_snapshot_data
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1438,"record_offset":45484,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":66542,"source_line":217,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1438,"record_offset":56232,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1438,"record_offset":56092,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":56852,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":5858,"source_line":1887,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1438,"record_offset":44332,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1438,"record_offset":45344,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":45292,"parent_record_offset":45268,"ancestor_record_offset":44124,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":45268,"parent_record_offset":45212,"ancestor_record_offset":44124,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1438,"record_offset":51408,"parent_record_offset":51384,"ancestor_record_offset":44124,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":47096,"parent_record_offset":47008,"ancestor_record_offset":44124,"inlinee":6210,"source_line":674,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":48708,"parent_record_offset":48680,"ancestor_record_offset":44124,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":47008,"parent_record_offset":46980,"ancestor_record_offset":44124,"inlinee":6208,"source_line":208,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":48768,"parent_record_offset":48708,"ancestor_record_offset":44124,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1438,"record_offset":44968,"parent_record_offset":44944,"ancestor_record_offset":44124,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1438,"record_offset":45016,"parent_record_offset":44992,"ancestor_record_offset":44124,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":45240,"parent_record_offset":45212,"ancestor_record_offset":44124,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":48016,"parent_record_offset":47992,"ancestor_record_offset":44124,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":45212,"parent_record_offset":45184,"ancestor_record_offset":44124,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":47992,"parent_record_offset":47964,"ancestor_record_offset":44124,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":44420,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56344,"parent_record_offset":56308,"ancestor_record_offset":44124,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":50272,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56284,"parent_record_offset":56260,"ancestor_record_offset":44124,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56308,"parent_record_offset":56284,"ancestor_record_offset":44124,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56800,"parent_record_offset":56776,"ancestor_record_offset":44124,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":44716,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":50816,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56408,"parent_record_offset":56372,"ancestor_record_offset":44124,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56776,"parent_record_offset":56752,"ancestor_record_offset":44124,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":56372,"parent_record_offset":56308,"ancestor_record_offset":44124,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":47964,"parent_record_offset":47932,"ancestor_record_offset":44124,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1438,"record_offset":46712,"parent_record_offset":46676,"ancestor_record_offset":44124,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":46676,"parent_record_offset":46636,"ancestor_record_offset":44124,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":46636,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1438,"record_offset":51172,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":15183,"source_line":364,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":51196,"parent_record_offset":51172,"ancestor_record_offset":44124,"inlinee":7723,"source_line":266,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":51220,"parent_record_offset":51196,"ancestor_record_offset":44124,"inlinee":7724,"source_line":379,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1438,"record_offset":46952,"parent_record_offset":46892,"ancestor_record_offset":44124,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1438,"record_offset":56464,"parent_record_offset":56436,"ancestor_record_offset":44124,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1438,"record_offset":44560,"parent_record_offset":44536,"ancestor_record_offset":44124,"inlinee":13852,"source_line":1204,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1438,"record_offset":45052,"parent_record_offset":44944,"ancestor_record_offset":44124,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1438,"record_offset":46576,"parent_record_offset":46508,"ancestor_record_offset":44124,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1438,"record_offset":56696,"parent_record_offset":56540,"ancestor_record_offset":44124,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1438,"record_offset":50920,"parent_record_offset":50896,"ancestor_record_offset":44124,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":50872,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":48800,"parent_record_offset":48708,"ancestor_record_offset":44124,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":51244,"parent_record_offset":51220,"ancestor_record_offset":44124,"inlinee":7725,"source_line":342,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":44660,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":44684,"parent_record_offset":44660,"ancestor_record_offset":44124,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":48164,"parent_record_offset":48136,"ancestor_record_offset":44124,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1438,"record_offset":44800,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":9450,"source_line":3203,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":47048,"parent_record_offset":47008,"ancestor_record_offset":44124,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":44884,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":44992,"parent_record_offset":44968,"ancestor_record_offset":44124,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":54452,"parent_record_offset":54428,"ancestor_record_offset":44124,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":46860,"parent_record_offset":46808,"ancestor_record_offset":44124,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":46048,"parent_record_offset":46024,"ancestor_record_offset":44124,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=135
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":45136,"parent_record_offset":45112,"ancestor_record_offset":44124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":50840,"parent_record_offset":50816,"ancestor_record_offset":44124,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":44124,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":46136,"parent_record_offset":45992,"ancestor_record_offset":44124,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":50896,"parent_record_offset":50872,"ancestor_record_offset":44124,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":45908,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":45936,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":44472,"parent_record_offset":44448,"ancestor_record_offset":44124,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":44604,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":50988,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":44856,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":44448,"parent_record_offset":44360,"ancestor_record_offset":44124,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":44512,"parent_record_offset":44472,"ancestor_record_offset":44124,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":44740,"parent_record_offset":44716,"ancestor_record_offset":44124,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":44628,"parent_record_offset":44604,"ancestor_record_offset":44124,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1438,"record_offset":44764,"parent_record_offset":44740,"ancestor_record_offset":44124,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1438,"record_offset":51012,"parent_record_offset":50988,"ancestor_record_offset":44124,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":56116,"parent_record_offset":56092,"ancestor_record_offset":44124,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":56140,"parent_record_offset":56116,"ancestor_record_offset":44124,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$18::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":56492,"parent_record_offset":56464,"ancestor_record_offset":44124,"inlinee":25540,"source_line":63,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$6::closest_snapshot::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":44536,"parent_record_offset":44512,"ancestor_record_offset":44124,"inlinee":25213,"source_line":349,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$6::remove_snapshot_record::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1438,"record_offset":56436,"parent_record_offset":56408,"ancestor_record_offset":44124,"inlinee":25538,"source_line":384,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1438,"record_offset":50408,"parent_record_offset":50384,"ancestor_record_offset":44124,"inlinee":24927,"source_line":32,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeights::as_raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1438,"record_offset":51072,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":66543,"source_line":139,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1438,"record_offset":49440,"parent_record_offset":49416,"ancestor_record_offset":44124,"inlinee":24944,"source_line":429,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1438,"record_offset":46508,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":61863,"source_line":177,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1438,"record_offset":51140,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":61865,"source_line":168,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":51824,"parent_record_offset":51800,"ancestor_record_offset":44124,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":45964,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":13488,"source_line":579,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":46024,"parent_record_offset":45992,"ancestor_record_offset":44124,"inlinee":11471,"source_line":228,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":45992,"parent_record_offset":45964,"ancestor_record_offset":44124,"inlinee":11475,"source_line":338,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":46072,"parent_record_offset":46048,"ancestor_record_offset":44124,"inlinee":11473,"source_line":229,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$21::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1438,"record_offset":45076,"parent_record_offset":45052,"ancestor_record_offset":44124,"inlinee":22992,"source_line":910,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1438,"record_offset":47748,"parent_record_offset":47724,"ancestor_record_offset":44124,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1438,"record_offset":47160,"parent_record_offset":47136,"ancestor_record_offset":44124,"inlinee":10265,"source_line":134,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$2::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":46980,"parent_record_offset":46892,"ancestor_record_offset":44124,"inlinee":7100,"source_line":302,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::from_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1438,"record_offset":47136,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":16493,"source_line":982,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::impl$4::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1438,"record_offset":46892,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":7096,"source_line":114,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1438,"record_offset":46832,"parent_record_offset":46808,"ancestor_record_offset":44124,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1438,"record_offset":46808,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1438,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":44124,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1438,"record_offset":45440,"parent_record_offset":45416,"ancestor_record_offset":44124,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1438,"record_offset":45416,"parent_record_offset":45392,"ancestor_record_offset":44124,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45792,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45816,"parent_record_offset":45792,"ancestor_record_offset":44124,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45876,"parent_record_offset":45792,"ancestor_record_offset":44124,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":1438,"record_offset":51284,"parent_record_offset":45484,"ancestor_record_offset":44124,"inlinee":48225,"source_line":947,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1438,"record_offset":46320,"parent_record_offset":46292,"ancestor_record_offset":44124,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1438,"record_offset":45368,"parent_record_offset":45344,"ancestor_record_offset":44124,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1438,"record_offset":56908,"parent_record_offset":56884,"ancestor_record_offset":44124,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1438,"record_offset":56884,"parent_record_offset":56852,"ancestor_record_offset":44124,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1438,"record_offset":45844,"parent_record_offset":45816,"ancestor_record_offset":44124,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x103EE
// 首条位置: {"module":1438,"inlinee":66542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1438,"record_offset":47640,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":50272,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":50816,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":46636,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1438,"record_offset":51172,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":15183,"source_line":364,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":50872,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":46208,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":45908,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":45936,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":50988,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::as_raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1438,"record_offset":51072,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":66543,"source_line":139,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1438,"record_offset":46508,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":61863,"source_line":177,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1438,"record_offset":51140,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":61865,"source_line":168,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":45964,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":13488,"source_line":579,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::from_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1438,"record_offset":47136,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":16493,"source_line":982,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::impl$4::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1438,"record_offset":46892,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":7096,"source_line":114,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1438,"record_offset":46808,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":22994,"source_line":212,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45792,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":1438,"record_offset":51284,"parent_record_offset":45484,"ancestor_record_offset":45484,"inlinee":48225,"source_line":947,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":48224,"parent_record_offset":48200,"ancestor_record_offset":45484,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":48200,"parent_record_offset":48108,"ancestor_record_offset":45484,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1438,"record_offset":51408,"parent_record_offset":51384,"ancestor_record_offset":45484,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":47096,"parent_record_offset":47008,"ancestor_record_offset":45484,"inlinee":6210,"source_line":674,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":48708,"parent_record_offset":48680,"ancestor_record_offset":45484,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":47008,"parent_record_offset":46980,"ancestor_record_offset":45484,"inlinee":6208,"source_line":208,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1438,"record_offset":48768,"parent_record_offset":48708,"ancestor_record_offset":45484,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1438,"record_offset":47688,"parent_record_offset":47664,"ancestor_record_offset":45484,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":48136,"parent_record_offset":48108,"ancestor_record_offset":45484,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":48016,"parent_record_offset":47992,"ancestor_record_offset":45484,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":48108,"parent_record_offset":48084,"ancestor_record_offset":45484,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":47992,"parent_record_offset":47964,"ancestor_record_offset":45484,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":47964,"parent_record_offset":47932,"ancestor_record_offset":45484,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1438,"record_offset":46712,"parent_record_offset":46676,"ancestor_record_offset":45484,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1438,"record_offset":46676,"parent_record_offset":46636,"ancestor_record_offset":45484,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":51196,"parent_record_offset":51172,"ancestor_record_offset":45484,"inlinee":7723,"source_line":266,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1438,"record_offset":51220,"parent_record_offset":51196,"ancestor_record_offset":45484,"inlinee":7724,"source_line":379,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1438,"record_offset":46952,"parent_record_offset":46892,"ancestor_record_offset":45484,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1438,"record_offset":46576,"parent_record_offset":46508,"ancestor_record_offset":45484,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1438,"record_offset":50920,"parent_record_offset":50896,"ancestor_record_offset":45484,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":48800,"parent_record_offset":48708,"ancestor_record_offset":45484,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1438,"record_offset":51244,"parent_record_offset":51220,"ancestor_record_offset":45484,"inlinee":7725,"source_line":342,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":48164,"parent_record_offset":48136,"ancestor_record_offset":45484,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":47048,"parent_record_offset":47008,"ancestor_record_offset":45484,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":47664,"parent_record_offset":47640,"ancestor_record_offset":45484,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":54452,"parent_record_offset":54428,"ancestor_record_offset":45484,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":46860,"parent_record_offset":46808,"ancestor_record_offset":45484,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":46048,"parent_record_offset":46024,"ancestor_record_offset":45484,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":46236,"parent_record_offset":46208,"ancestor_record_offset":45484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":50840,"parent_record_offset":50816,"ancestor_record_offset":45484,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":45484,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":46136,"parent_record_offset":45992,"ancestor_record_offset":45484,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":50896,"parent_record_offset":50872,"ancestor_record_offset":45484,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":46096,"parent_record_offset":46072,"ancestor_record_offset":45484,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1438,"record_offset":46604,"parent_record_offset":46508,"ancestor_record_offset":45484,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1438,"record_offset":51012,"parent_record_offset":50988,"ancestor_record_offset":45484,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1438,"record_offset":50408,"parent_record_offset":50384,"ancestor_record_offset":45484,"inlinee":24927,"source_line":32,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1438,"record_offset":49440,"parent_record_offset":49416,"ancestor_record_offset":45484,"inlinee":24944,"source_line":429,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":51824,"parent_record_offset":51800,"ancestor_record_offset":45484,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":46024,"parent_record_offset":45992,"ancestor_record_offset":45484,"inlinee":11471,"source_line":228,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":45992,"parent_record_offset":45964,"ancestor_record_offset":45484,"inlinee":11475,"source_line":338,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1438,"record_offset":46072,"parent_record_offset":46048,"ancestor_record_offset":45484,"inlinee":11473,"source_line":229,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1438,"record_offset":47748,"parent_record_offset":47724,"ancestor_record_offset":45484,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1438,"record_offset":47160,"parent_record_offset":47136,"ancestor_record_offset":45484,"inlinee":10265,"source_line":134,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$2::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1438,"record_offset":46980,"parent_record_offset":46892,"ancestor_record_offset":45484,"inlinee":7100,"source_line":302,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1438,"record_offset":46832,"parent_record_offset":46808,"ancestor_record_offset":45484,"inlinee":22996,"source_line":123,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45816,"parent_record_offset":45792,"ancestor_record_offset":45484,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1438,"record_offset":45876,"parent_record_offset":45792,"ancestor_record_offset":45484,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1438,"record_offset":46320,"parent_record_offset":46292,"ancestor_record_offset":45484,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1438,"record_offset":45844,"parent_record_offset":45816,"ancestor_record_offset":45484,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1321,"record_offset":121820,"rva":14588544,"va":5383297664}
// 编译实例 RVAs: 0xde9a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10984242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e8ce6458ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5383298445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10548147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e84d0c5fff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383297854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10548166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e83a0c5fff","size":5,"soft_fail":false,"target_va":5372749744,"va":5383297905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405932a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8743214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e8d2967aff","size":5,"soft_fail":false,"target_va":5374554784,"va":5383297993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411071c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3266326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e816d73100","size":5,"soft_fail":false,"target_va":5386564032,"va":5383297701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420caee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19796661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e8b5122e01","size":5,"soft_fail":false,"target_va":5403094752,"va":5383298086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cb2f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19798032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e810182e01","size":5,"soft_fail":false,"target_va":5403095792,"va":5383297755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e34f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19896800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e8e0992f01","size":5,"soft_fail":false,"target_va":5403194608,"va":5383297803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19680212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e8d44b2c01","size":5,"soft_fail":false,"target_va":5402978144,"va":5383297927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5771109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e89bf0a7ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5383298224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] flate2::mem::Decompress::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2541083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e81bc62600","size":5,"soft_fail":false,"target_va":5385839040,"va":5383297952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5902475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383297664,"raw_hex":"e875efa5ff","size":5,"soft_fail":false,"target_va":5377395552,"va":5383298022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":122544,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1321,"record_offset":122860,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1321,"record_offset":122160,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":45284,"source_line":321,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":122340,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":122312,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1321,"record_offset":122008,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":61863,"source_line":177,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1321,"record_offset":122800,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":61865,"source_line":168,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":1321,"record_offset":122832,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":48225,"source_line":947,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1321,"record_offset":122132,"parent_record_offset":121820,"ancestor_record_offset":121820,"inlinee":18799,"source_line":99,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":122688,"parent_record_offset":122664,"ancestor_record_offset":121820,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":122712,"parent_record_offset":122688,"ancestor_record_offset":121820,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":122496,"parent_record_offset":122472,"ancestor_record_offset":121820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":122664,"parent_record_offset":122628,"ancestor_record_offset":121820,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":122472,"parent_record_offset":122416,"ancestor_record_offset":121820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1321,"record_offset":123092,"parent_record_offset":123068,"ancestor_record_offset":121820,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":122444,"parent_record_offset":122416,"ancestor_record_offset":121820,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":122628,"parent_record_offset":122600,"ancestor_record_offset":121820,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":122600,"parent_record_offset":122572,"ancestor_record_offset":121820,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":122752,"parent_record_offset":122600,"ancestor_record_offset":121820,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":122416,"parent_record_offset":122388,"ancestor_record_offset":121820,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1321,"record_offset":122572,"parent_record_offset":122544,"ancestor_record_offset":121820,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1321,"record_offset":122928,"parent_record_offset":122892,"ancestor_record_offset":121820,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1321,"record_offset":122892,"parent_record_offset":122860,"ancestor_record_offset":121820,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1321,"record_offset":122072,"parent_record_offset":122008,"ancestor_record_offset":121820,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":122268,"parent_record_offset":122244,"ancestor_record_offset":121820,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":122364,"parent_record_offset":122340,"ancestor_record_offset":121820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":122044,"parent_record_offset":122008,"ancestor_record_offset":121820,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":122100,"parent_record_offset":122008,"ancestor_record_offset":121820,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":122244,"parent_record_offset":122216,"ancestor_record_offset":121820,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":1321,"record_offset":122188,"parent_record_offset":122160,"ancestor_record_offset":121820,"inlinee":24848,"source_line":354,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":1321,"record_offset":122216,"parent_record_offset":122188,"ancestor_record_offset":121820,"inlinee":24844,"source_line":282,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x601C
// 首条位置: {"module":424,"inlinee":24604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::init_game_state_from_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1438,"record_offset":59888,"rva":17476656,"va":5386185776}
// 编译实例 RVAs: 0x10aac30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd390
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14606795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8351e21ff","size":5,"soft_fail":false,"target_va":5371581328,"va":5386188118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13951098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8861f2bff","size":5,"soft_fail":false,"target_va":5372239520,"va":5386190613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=22
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13437097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e857f732ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386186804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ae440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11522487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8492e50ff","size":5,"soft_fail":false,"target_va":5374665792,"va":5386188274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10303239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8f9c862ff","size":5,"soft_fail":false,"target_va":5375887456,"va":5386190690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dbb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10289752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8a8fd62ff","size":5,"soft_fail":false,"target_va":5375900480,"va":5386190227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dd420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10283550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8e21563ff","size":5,"soft_fail":false,"target_va":5375906848,"va":5386190393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406df640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10274871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8c93763ff","size":5,"soft_fail":false,"target_va":5375915584,"va":5386190450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f2e740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1559060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8ec35e8ff","size":5,"soft_fail":false,"target_va":5384628032,"va":5386187087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17404883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8d3930901","size":5,"soft_fail":false,"target_va":5403593360,"va":5386188472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-14877962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8f6fa1cff","size":5,"soft_fail":false,"target_va":5371308800,"va":5386186757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16789892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e884310001","size":5,"soft_fail":false,"target_va":5402979152,"va":5386189255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16791057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e811360001","size":5,"soft_fail":false,"target_va":5402980464,"va":5386189402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11469408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8a0fd50ff","size":5,"soft_fail":false,"target_va":5374717584,"va":5386186987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::load_state
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10898771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e8adb259ff","size":5,"soft_fail":false,"target_va":5375287712,"va":5386186478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8832198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386185776,"raw_hex":"e83a3b79ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386185969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":65524,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1438,"record_offset":60268,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1438,"record_offset":60908,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1438,"record_offset":60340,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1438,"record_offset":65108,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1438,"record_offset":60184,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1438,"record_offset":60212,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":60616,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":65456,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":65024,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::as_mut_raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1438,"record_offset":65424,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":59408,"source_line":143,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_snapshot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1438,"record_offset":61020,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":24806,"source_line":404,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1438,"record_offset":60240,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1438,"record_offset":60100,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":63564,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":64200,"parent_record_offset":59888,"ancestor_record_offset":59888,"inlinee":24530,"source_line":2357,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":60852,"parent_record_offset":60828,"ancestor_record_offset":59888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1438,"record_offset":60828,"parent_record_offset":60736,"ancestor_record_offset":59888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":60764,"parent_record_offset":60736,"ancestor_record_offset":59888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":60488,"parent_record_offset":60456,"ancestor_record_offset":59888,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1438,"record_offset":60736,"parent_record_offset":60712,"ancestor_record_offset":59888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":60456,"parent_record_offset":60412,"ancestor_record_offset":59888,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":60412,"parent_record_offset":60368,"ancestor_record_offset":59888,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":61192,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1438,"record_offset":66912,"parent_record_offset":66888,"ancestor_record_offset":59888,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1438,"record_offset":60640,"parent_record_offset":60616,"ancestor_record_offset":59888,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1438,"record_offset":60688,"parent_record_offset":60664,"ancestor_record_offset":59888,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1438,"record_offset":60580,"parent_record_offset":60556,"ancestor_record_offset":59888,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1438,"record_offset":60556,"parent_record_offset":60340,"ancestor_record_offset":59888,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1438,"record_offset":60368,"parent_record_offset":60340,"ancestor_record_offset":59888,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1438,"record_offset":63776,"parent_record_offset":63748,"ancestor_record_offset":59888,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1438,"record_offset":63836,"parent_record_offset":63804,"ancestor_record_offset":59888,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1438,"record_offset":65492,"parent_record_offset":65456,"ancestor_record_offset":59888,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1438,"record_offset":61424,"parent_record_offset":61400,"ancestor_record_offset":59888,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1438,"record_offset":61400,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1438,"record_offset":60792,"parent_record_offset":60764,"ancestor_record_offset":59888,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1438,"record_offset":64436,"parent_record_offset":64412,"ancestor_record_offset":59888,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":61136,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1438,"record_offset":61160,"parent_record_offset":61136,"ancestor_record_offset":59888,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1438,"record_offset":63872,"parent_record_offset":63720,"ancestor_record_offset":59888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":60664,"parent_record_offset":60640,"ancestor_record_offset":59888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":61364,"parent_record_offset":61340,"ancestor_record_offset":59888,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":64656,"parent_record_offset":64632,"ancestor_record_offset":59888,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1438,"record_offset":64632,"parent_record_offset":64608,"ancestor_record_offset":59888,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1438,"record_offset":61552,"parent_record_offset":61488,"ancestor_record_offset":59888,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1438,"record_offset":60304,"parent_record_offset":60268,"ancestor_record_offset":59888,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":61576,"parent_record_offset":61552,"ancestor_record_offset":59888,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1438,"record_offset":60520,"parent_record_offset":60412,"ancestor_record_offset":59888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1438,"record_offset":61316,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1438,"record_offset":61340,"parent_record_offset":61316,"ancestor_record_offset":59888,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1438,"record_offset":61220,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":59888,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1438,"record_offset":60148,"parent_record_offset":60124,"ancestor_record_offset":59888,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1438,"record_offset":61104,"parent_record_offset":61080,"ancestor_record_offset":59888,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1438,"record_offset":63612,"parent_record_offset":63588,"ancestor_record_offset":59888,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1438,"record_offset":64248,"parent_record_offset":64224,"ancestor_record_offset":59888,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":64608,"parent_record_offset":64300,"ancestor_record_offset":59888,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":63720,"parent_record_offset":63660,"ancestor_record_offset":59888,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":64300,"parent_record_offset":64272,"ancestor_record_offset":59888,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":63636,"parent_record_offset":63612,"ancestor_record_offset":59888,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":63692,"parent_record_offset":63660,"ancestor_record_offset":59888,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":63660,"parent_record_offset":63636,"ancestor_record_offset":59888,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1438,"record_offset":64380,"parent_record_offset":64348,"ancestor_record_offset":59888,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1438,"record_offset":64412,"parent_record_offset":64380,"ancestor_record_offset":59888,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1438,"record_offset":64348,"parent_record_offset":64300,"ancestor_record_offset":59888,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":64976,"parent_record_offset":64952,"ancestor_record_offset":59888,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1438,"record_offset":64272,"parent_record_offset":64248,"ancestor_record_offset":59888,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1438,"record_offset":63748,"parent_record_offset":63720,"ancestor_record_offset":59888,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1438,"record_offset":63804,"parent_record_offset":63748,"ancestor_record_offset":59888,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":61080,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":63588,"parent_record_offset":63564,"ancestor_record_offset":59888,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":64224,"parent_record_offset":64200,"ancestor_record_offset":59888,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$80::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1438,"record_offset":61456,"parent_record_offset":61020,"ancestor_record_offset":59888,"inlinee":18503,"source_line":3143,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::init_game_state_from_snapshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6019
// 首条位置: {"module":424,"inlinee":24601}
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
// 完整结构及原始类型记录: snapshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot::LoadedSnapshotData
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::snapshot::LoadedSnapshotData>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::snapshot::LoadedSnapshotData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot::SnapshotLoadResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapshot.rs.enriched.json.gz
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
// 函数: country_core::systems::save::snapshot::_::impl$0::get_param
// 信息 ID: c9759709cea21993e4ee1302
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 17e72c8a0e80467e9a1e0999: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> @ 0xd24a1a -> 0x10ac0d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> > @ 0xe45905 -> 0x10ac0d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::init_state
// 信息 ID: 54663e235bb28b40fcaab2c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 17e72c8a0e80467e9a1e0999: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::event::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)> @ 0xd240d9 -> 0x10abf70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot
// 信息 ID: 908ec2205f0c7334a031c033
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_game::save_game @ 0x5dde08 -> 0x10a3ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot::closure$0
// 信息 ID: dc6bfe2db050bd939747c94a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::snapshot::capture_game_snapshot", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<usize,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<usize,country_core::resources::history::tree_edit::TreeData>,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<tuple$<usize,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot::closure_env$0>; module=1308, inline_record=2432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::init_game_state_from_snapshot
// 信息 ID: fed57be7a299cba13d3dfd96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x17fa5b3 -> 0x10aac30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::init_game_state_from_snapshot::closure$0
// 信息 ID: 8903963c6f42bcd0661c579b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::snapshot::init_game_state_from_snapshot", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0 <- country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=32520; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=33000; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=33424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_chunk
// 信息 ID: 4999638512ed07a8d711dbb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x10a9cda -> 0x10a8620 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::save::snapshot::try_load_snapshot_data::closure$0; module=1321, inline_record=122008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data; module=1438, inline_record=46508; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data
// 信息 ID: f9ddb2cf1d6d0ebcabdb6034
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data; module=1438, inline_record=45484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data::closure$0
// 信息 ID: 87c6dde1161531e93b0f5e10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::snapshot::try_load_snapshot_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$25::from_iter::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error,alloc::vec::Vec<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0> >,alloc::vec::Vec<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$25::from_iter::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error,alloc::vec::Vec<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0> >,alloc::vec::Vec<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 4 条。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0xd72394 -> 0xde9a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::in_place_collect::impl$1::from_iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > @ 0xd725b5 -> 0xde9a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data::closure$1
// 信息 ID: f6302e3b701f996e7a1fb098
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::snapshot::try_load_snapshot_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1 <- country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=34452; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=34820; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=35192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data
// 信息 ID: 8c2a357570a2f6c4c1cbf96a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x17f9fa0 -> 0x10a86b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
