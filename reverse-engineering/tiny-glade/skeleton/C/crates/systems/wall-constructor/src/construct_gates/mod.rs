// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_gates/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 37218c2579e10fc0a64a2410226e34e4c1c8f1c8d92a7f6021f5e2729198858f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1455, 1482, 1487, 1493, 1504, 3144, 3150
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1487,"record_offset":3168,"rva":21168624,"va":5389877744}
// 编译实例 RVAs: 0x14301f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-14199477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e84b5527ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389878160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=39
//     evidence={"assembly":"\tcallq\t-14200963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e87d4f27ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389879662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21155960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8882fbdfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389878147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20657331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e84dcbc4fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5389897854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20660595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e88dbec4fe","size":5,"soft_fail":false,"target_va":5369240608,"va":5389901198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400beb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20405539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8dda2c8fe","size":5,"soft_fail":false,"target_va":5369490256,"va":5389895790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=4
//     target_aliases=17 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20346471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e89989c9fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389888466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18002538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8964dedfe","size":5,"soft_fail":false,"target_va":5371879952,"va":5389882485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16304803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e85d3507ff","size":5,"soft_fail":false,"target_va":5373574768,"va":5389879566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16306909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8232d07ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5389882536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14059384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8887829ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5389888435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11776658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e86e4d4cff","size":5,"soft_fail":false,"target_va":5378112304,"va":5389888957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11759854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8128f4cff","size":5,"soft_fail":false,"target_va":5378120480,"va":5389880329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9f60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11767593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8d7704cff","size":5,"soft_fail":false,"target_va":5378121568,"va":5389889156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11756854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8ca9a4cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389883793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099d460
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11108531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e84d7f56ff","size":5,"soft_fail":false,"target_va":5378790496,"va":5389899022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6555242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e896f99bff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389880261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6568801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e89fc49bff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389893852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-269058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8fee4fbff","size":5,"soft_fail":false,"target_va":5389619632,"va":5389888685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f9f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-233435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e82570fcff","size":5,"soft_fail":false,"target_va":5389655888,"va":5389889318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-180381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8633ffdff","size":5,"soft_fail":false,"target_va":5389702352,"va":5389882728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=6
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-181306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8c63bfdff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389885973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140ab90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-171475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e82d62fdff","size":5,"soft_fail":false,"target_va":5389724560,"va":5389896030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-161153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e87f8afdff","size":5,"soft_fail":false,"target_va":5389735152,"va":5389896300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140db60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-162732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e85484fdff","size":5,"soft_fail":false,"target_va":5389736800,"va":5389899527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140ecb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-158035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8ad96fdff","size":5,"soft_fail":false,"target_va":5389741232,"va":5389899262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140ee00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-158732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8f493fdff","size":5,"soft_fail":false,"target_va":5389741568,"va":5389900295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8c28bfeff","size":5,"soft_fail":false,"target_va":5389786032,"va":5389881321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141423640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-64381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e88304ffff","size":5,"soft_fail":false,"target_va":5389825600,"va":5389889976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142c6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e82b86ffff","size":5,"soft_fail":false,"target_va":5389862560,"va":5389893744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414530a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t136654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8ce150200","size":5,"soft_fail":false,"target_va":5390020768,"va":5389884109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141473110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t273956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8242e0400","size":5,"soft_fail":false,"target_va":5390151952,"va":5389877991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414732e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t265190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8e60b0400","size":5,"soft_fail":false,"target_va":5390152416,"va":5389887221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147cbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t302597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8059e0400","size":5,"soft_fail":false,"target_va":5390191536,"va":5389888934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ddfd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17480235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e82bba0a01","size":5,"soft_fail":false,"target_va":5407367120,"va":5389886880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e18d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17500603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8bb090b01","size":5,"soft_fail":false,"target_va":5407381712,"va":5389881104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e1aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17494113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e861f00a01","size":5,"soft_fail":false,"target_va":5407382176,"va":5389888058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e1f20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17495413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e875f50a01","size":5,"soft_fail":false,"target_va":5407383328,"va":5389887910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26472794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e85af19301","size":5,"soft_fail":false,"target_va":5416352064,"va":5389879265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26468718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e86ee19301","size":5,"soft_fail":false,"target_va":5416353040,"va":5389884317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27177514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e82ab29e01","size":5,"soft_fail":false,"target_va":5417057056,"va":5389879537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27173555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8b3a29e01","size":5,"soft_fail":false,"target_va":5417057088,"va":5389883528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27165172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8f4819e01","size":5,"soft_fail":false,"target_va":5417057600,"va":5389892423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27176812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e86caf9e01","size":5,"soft_fail":false,"target_va":5417057984,"va":5389881167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26725866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8eacd9701","size":5,"soft_fail":false,"target_va":5416620769,"va":5389894898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26486036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e814259401","size":5,"soft_fail":false,"target_va":5416366080,"va":5389880039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26741355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e86b0a9801","size":5,"soft_fail":false,"target_va":5416620794,"va":5389879434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5655479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e849b4a9ff","size":5,"soft_fail":false,"target_va":5384226192,"va":5389881666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_writer::MessageWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t279781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8e5440400","size":5,"soft_fail":false,"target_va":5390173984,"va":5389894198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t26727372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8ccd39701","size":5,"soft_fail":false,"target_va":5416621600,"va":5389894223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t26727138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8e2d29701","size":5,"soft_fail":false,"target_va":5416621762,"va":5389894619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26730002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e812de9701","size":5,"soft_fail":false,"target_va":5416622240,"va":5389892233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26727305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e889d39701","size":5,"soft_fail":false,"target_va":5416621632,"va":5389894322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26727783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e867d59701","size":5,"soft_fail":false,"target_va":5416622320,"va":5389894532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10616429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e893015eff","size":5,"soft_fail":false,"target_va":5379271936,"va":5389888360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10366660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e83cd161ff","size":5,"soft_fail":false,"target_va":5379521760,"va":5389888415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10609659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8051c5eff","size":5,"soft_fail":false,"target_va":5379278928,"va":5389888582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMesh::new_with_registry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11320140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8b44453ff","size":5,"soft_fail":false,"target_va":5378569040,"va":5389889175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight_with_depth
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9985095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8b9a367ff","size":5,"soft_fail":false,"target_va":5379908848,"va":5389893938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::with_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9984987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e825a467ff","size":5,"soft_fail":false,"target_va":5379908992,"va":5389893974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::mesh_from_pts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8294618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8266f81ff","size":5,"soft_fail":false,"target_va":5381593216,"va":5389887829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6185517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e8d39da1ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5389888504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_gates::DoorMeshInProgress::add_mirrored_quad
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e891d6ffff","size":5,"soft_fail":false,"target_va":5389876112,"va":5389886714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_pieces
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17466686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e83e850a01","size":5,"soft_fail":false,"target_va":5407360208,"va":5389893517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6563831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e809d89bff","size":5,"soft_fail":false,"target_va":5383324736,"va":5389888562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6605930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389877744,"raw_hex":"e896339bff","size":5,"soft_fail":false,"target_va":5383275712,"va":5389881637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5389877744,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5389888718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1487,"record_offset":26872,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":3520,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":15816,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":8568,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":37780,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":27084,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":14736,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1487,"record_offset":33084,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":15028,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":14708,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":27056,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1487,"record_offset":27624,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1487,"record_offset":37048,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":33404,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":33348,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":33208,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":32836,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":8692,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":7064,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":27324,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":8664,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":35196,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1487,"record_offset":32808,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1487,"record_offset":32956,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1487,"record_offset":32900,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":8816,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":14764,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":14852,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":8728,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":36060,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":36116,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":36872,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":36780,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] itertools::adaptors::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/adaptors/mod.rs
//     evidence={"module":1487,"record_offset":37076,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":38548,"source_line":398,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1487,"record_offset":33052,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":14652,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":31108,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79693,"source_line":396,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_arch_heights
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":7356,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79681,"source_line":592,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_bottom_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3548,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79679,"source_line":537,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":27772,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79688,"source_line":434,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh_gap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":30352,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79691,"source_line":494,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":26360,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79685,"source_line":584,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":27744,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79687,"source_line":387,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_brick_steps
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":16024,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79684,"source_line":615,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_ladder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":15056,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79683,"source_line":851,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_wooden_steps
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":9056,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":79682,"source_line":703,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":33584,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":8540,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":26844,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":35356,"parent_record_offset":3168,"ancestor_record_offset":3168,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3168,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3168,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":3168,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":3168,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3168,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":3168,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":3168,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6488,"parent_record_offset":6396,"ancestor_record_offset":3168,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1487,"record_offset":26900,"parent_record_offset":26872,"ancestor_record_offset":3168,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5224,"parent_record_offset":5192,"ancestor_record_offset":3168,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15660,"parent_record_offset":15632,"ancestor_record_offset":3168,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4336,"parent_record_offset":4300,"ancestor_record_offset":3168,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15744,"parent_record_offset":15720,"ancestor_record_offset":3168,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":3168,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":3168,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15720,"parent_record_offset":15688,"ancestor_record_offset":3168,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":3168,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":3168,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15688,"parent_record_offset":15660,"ancestor_record_offset":3168,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4400,"parent_record_offset":4368,"ancestor_record_offset":3168,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4368,"parent_record_offset":4336,"ancestor_record_offset":3168,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":3168,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5920,"parent_record_offset":5880,"ancestor_record_offset":3168,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1487,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":3168,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1487,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":3168,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5192,"parent_record_offset":5152,"ancestor_record_offset":3168,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18772,"parent_record_offset":18740,"ancestor_record_offset":3168,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":3168,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":16276,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5124,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":3168,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":15632,"parent_record_offset":15596,"ancestor_record_offset":3168,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18740,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":6248,"parent_record_offset":5768,"ancestor_record_offset":3168,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18508,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":4264,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":4300,"parent_record_offset":4264,"ancestor_record_offset":3168,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5720,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":6616,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":3168,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1487,"record_offset":19364,"parent_record_offset":18772,"ancestor_record_offset":3168,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15568,"parent_record_offset":15540,"ancestor_record_offset":3168,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15596,"parent_record_offset":15568,"ancestor_record_offset":3168,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15512,"parent_record_offset":15476,"ancestor_record_offset":3168,"inlinee":11375,"source_line":262,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15476,"parent_record_offset":15056,"ancestor_record_offset":3168,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15540,"parent_record_offset":15476,"ancestor_record_offset":3168,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1487,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":3168,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::Guard::push_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30624,"parent_record_offset":30516,"ancestor_record_offset":3168,"inlinee":26464,"source_line":971,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1487,"record_offset":30560,"parent_record_offset":30516,"ancestor_record_offset":3168,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30440,"parent_record_offset":30352,"ancestor_record_offset":3168,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":3168,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":10572,"parent_record_offset":10540,"ancestor_record_offset":3168,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":37124,"parent_record_offset":37076,"ancestor_record_offset":3168,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":10220,"parent_record_offset":10192,"ancestor_record_offset":3168,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":3168,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":37156,"parent_record_offset":37124,"ancestor_record_offset":3168,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10248,"parent_record_offset":10220,"ancestor_record_offset":3168,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":37292,"parent_record_offset":37188,"ancestor_record_offset":3168,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10368,"parent_record_offset":10276,"ancestor_record_offset":3168,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":3168,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30516,"parent_record_offset":30488,"ancestor_record_offset":3168,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3168,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":10980,"parent_record_offset":10868,"ancestor_record_offset":3168,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":3168,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":3168,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":16492,"parent_record_offset":16432,"ancestor_record_offset":3168,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":3168,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":10868,"parent_record_offset":10840,"ancestor_record_offset":3168,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1487,"record_offset":20752,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4944,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4832,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":3168,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":3168,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3168,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1487,"record_offset":6024,"parent_record_offset":5968,"ancestor_record_offset":3168,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1487,"record_offset":28376,"parent_record_offset":28352,"ancestor_record_offset":3168,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1487,"record_offset":28424,"parent_record_offset":28400,"ancestor_record_offset":3168,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1487,"record_offset":5412,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":3168,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10804,"parent_record_offset":10776,"ancestor_record_offset":3168,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10540,"parent_record_offset":10124,"ancestor_record_offset":3168,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10164,"parent_record_offset":10124,"ancestor_record_offset":3168,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":33372,"parent_record_offset":33348,"ancestor_record_offset":3168,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3168,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":3168,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3940,"parent_record_offset":3900,"ancestor_record_offset":3168,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":3168,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4740,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":3168,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4780,"parent_record_offset":4740,"ancestor_record_offset":3168,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16536,"parent_record_offset":16432,"ancestor_record_offset":3168,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16432,"parent_record_offset":16388,"ancestor_record_offset":3168,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16388,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3168,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":4212,"parent_record_offset":3940,"ancestor_record_offset":3168,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1487,"record_offset":33452,"parent_record_offset":33428,"ancestor_record_offset":3168,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":16776,"parent_record_offset":16660,"ancestor_record_offset":3168,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10032,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":3168,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10484,"parent_record_offset":10368,"ancestor_record_offset":3168,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3168,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10088,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10840,"parent_record_offset":10804,"ancestor_record_offset":3168,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10124,"parent_record_offset":10088,"ancestor_record_offset":3168,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3828,"parent_record_offset":3716,"ancestor_record_offset":3168,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":4600,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":28448,"parent_record_offset":28424,"ancestor_record_offset":3168,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1487,"record_offset":10400,"parent_record_offset":10368,"ancestor_record_offset":3168,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1487,"record_offset":30656,"parent_record_offset":30624,"ancestor_record_offset":3168,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":3168,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":3168,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":3168,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":6456,"parent_record_offset":6424,"ancestor_record_offset":3168,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":13212,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":3168,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":3168,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":18416,"parent_record_offset":18384,"ancestor_record_offset":3168,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1487,"record_offset":4872,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":32763,"source_line":38,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3168,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":10896,"parent_record_offset":10868,"ancestor_record_offset":3168,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::assume_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10712,"parent_record_offset":10668,"ancestor_record_offset":3168,"inlinee":10923,"source_line":120,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10312,"parent_record_offset":10276,"ancestor_record_offset":3168,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":37188,"parent_record_offset":37156,"ancestor_record_offset":3168,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":3168,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":3168,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":16940,"parent_record_offset":16908,"ancestor_record_offset":3168,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":10776,"parent_record_offset":10740,"ancestor_record_offset":3168,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":30588,"parent_record_offset":30516,"ancestor_record_offset":3168,"inlinee":26462,"source_line":448,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":30756,"parent_record_offset":30488,"ancestor_record_offset":3168,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":37584,"parent_record_offset":37076,"ancestor_record_offset":3168,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":3168,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":28400,"parent_record_offset":28376,"ancestor_record_offset":3168,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":4564,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1487,"record_offset":6192,"parent_record_offset":5768,"ancestor_record_offset":3168,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1487,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":3168,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":3168,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=83
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":6324,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3168,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1487,"record_offset":18352,"parent_record_offset":18280,"ancestor_record_offset":3168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1487,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":3168,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":10448,"parent_record_offset":10424,"ancestor_record_offset":3168,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":5312,"parent_record_offset":5152,"ancestor_record_offset":3168,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1487,"record_offset":6288,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1487,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":3168,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":18536,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":9236,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":5348,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":3168,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":3168,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":26504,"parent_record_offset":26360,"ancestor_record_offset":3168,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":3168,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":3168,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":3168,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":3168,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":3168,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1487,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3168,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":3168,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":3168,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":18280,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1487,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":3168,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1487,"record_offset":32980,"parent_record_offset":32956,"ancestor_record_offset":3168,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1487,"record_offset":21008,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":11681,"source_line":201,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13576,"parent_record_offset":13240,"ancestor_record_offset":3168,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":8916,"parent_record_offset":8880,"ancestor_record_offset":3168,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":8880,"parent_record_offset":8840,"ancestor_record_offset":3168,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":3168,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26812,"parent_record_offset":26360,"ancestor_record_offset":3168,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26624,"parent_record_offset":26360,"ancestor_record_offset":3168,"inlinee":27634,"source_line":546,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":3168,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11296,"parent_record_offset":11184,"ancestor_record_offset":3168,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":18384,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":3168,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":3168,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4108,"parent_record_offset":4020,"ancestor_record_offset":3168,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12836,"parent_record_offset":12808,"ancestor_record_offset":3168,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12808,"parent_record_offset":12768,"ancestor_record_offset":3168,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":5012,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":34248,"parent_record_offset":34224,"ancestor_record_offset":3168,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12768,"parent_record_offset":12732,"ancestor_record_offset":3168,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12732,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3168,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":9556,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3168,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1487,"record_offset":28352,"parent_record_offset":27936,"ancestor_record_offset":3168,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1487,"record_offset":27936,"parent_record_offset":27772,"ancestor_record_offset":3168,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1487,"record_offset":28196,"parent_record_offset":28164,"ancestor_record_offset":3168,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12100,"parent_record_offset":11016,"ancestor_record_offset":3168,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3168,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":13240,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":79698,"source_line":814,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":3168,"inlinee":79610,"source_line":826,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":3168,"inlinee":79690,"source_line":443,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11016,"parent_record_offset":10804,"ancestor_record_offset":3168,"inlinee":79696,"source_line":741,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":3168,"inlinee":79642,"source_line":744,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":3168,"inlinee":79643,"source_line":745,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":3168,"inlinee":79697,"source_line":756,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":3168,"inlinee":79695,"source_line":757,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3716,"parent_record_offset":3548,"ancestor_record_offset":3168,"inlinee":79680,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3168,"inlinee":79627,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::WoodenStairDebugParams::style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1487,"record_offset":13160,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":26140,"source_line":130,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":1487,"record_offset":12432,"parent_record_offset":9056,"ancestor_record_offset":3168,"inlinee":26124,"source_line":68,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1487,"record_offset":16360,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":11668,"source_line":50,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":16832,"parent_record_offset":16660,"ancestor_record_offset":3168,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":17204,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":26303,"source_line":851,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":16660,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":33616,"parent_record_offset":33584,"ancestor_record_offset":3168,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":7432,"parent_record_offset":7356,"ancestor_record_offset":3168,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29204,"parent_record_offset":29164,"ancestor_record_offset":3168,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29296,"parent_record_offset":29164,"ancestor_record_offset":3168,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1487,"record_offset":16908,"parent_record_offset":16024,"ancestor_record_offset":3168,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29164,"parent_record_offset":27772,"ancestor_record_offset":3168,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1487,"record_offset":34224,"parent_record_offset":33812,"ancestor_record_offset":3168,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135FF
// 首条位置: {"module":1455,"inlinee":79359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1455,"record_offset":133344,"parent_record_offset":132988,"ancestor_record_offset":132988,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1455,"record_offset":133020,"parent_record_offset":132988,"ancestor_record_offset":132988,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1455,"record_offset":133112,"parent_record_offset":132988,"ancestor_record_offset":132988,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":133372,"parent_record_offset":133344,"ancestor_record_offset":132988,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1455,"record_offset":133140,"parent_record_offset":133112,"ancestor_record_offset":132988,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1455,"record_offset":133052,"parent_record_offset":133020,"ancestor_record_offset":132988,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::impl$5::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13747
// 首条位置: {"module":1487,"inlinee":79687}
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
// 源码行: 396
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1374D
// 首条位置: {"module":1487,"inlinee":79693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":31248,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":31164,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":31432,"parent_record_offset":31408,"ancestor_record_offset":31108,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":31108,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":31480,"parent_record_offset":31456,"ancestor_record_offset":31108,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":31408,"parent_record_offset":31320,"ancestor_record_offset":31108,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":31348,"parent_record_offset":31320,"ancestor_record_offset":31108,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":31320,"parent_record_offset":31296,"ancestor_record_offset":31108,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":31296,"parent_record_offset":31272,"ancestor_record_offset":31108,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":31376,"parent_record_offset":31348,"ancestor_record_offset":31108,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":31272,"parent_record_offset":31248,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":31212,"parent_record_offset":31188,"ancestor_record_offset":31108,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1487,"record_offset":31188,"parent_record_offset":31164,"ancestor_record_offset":31108,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::DoorMeshInProgress::add_mirrored_quad
// 记录形式: compiled_procedure; instances=1
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1487,"record_offset":280,"rva":21166992,"va":5389876112}
// 编译实例 RVAs: 0x142fb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26475657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389876112,"raw_hex":"e889fc9301","size":5,"soft_fail":false,"target_va":5416352064,"va":5389876402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26503710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389876112,"raw_hex":"e81e6a9401","size":5,"soft_fail":false,"target_va":5416380224,"va":5389876509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26744033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389876112,"raw_hex":"e8e1149801","size":5,"soft_fail":false,"target_va":5416621762,"va":5389877724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":1608,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1487,"record_offset":456,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11834,"source_line":225,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":1396,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":1492,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":1836,"parent_record_offset":1748,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":1916,"parent_record_offset":1876,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":1876,"parent_record_offset":1836,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":1748,"parent_record_offset":1656,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":1656,"parent_record_offset":1608,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":652,"parent_record_offset":616,"ancestor_record_offset":280,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":692,"parent_record_offset":652,"ancestor_record_offset":280,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":772,"parent_record_offset":732,"ancestor_record_offset":280,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1487,"record_offset":576,"parent_record_offset":496,"ancestor_record_offset":280,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1487,"record_offset":496,"parent_record_offset":456,"ancestor_record_offset":280,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1487,"record_offset":1060,"parent_record_offset":1032,"ancestor_record_offset":280,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1487,"record_offset":1032,"parent_record_offset":496,"ancestor_record_offset":280,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1487,"record_offset":812,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":1536,"parent_record_offset":1492,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":1428,"parent_record_offset":1396,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":616,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":1304,"parent_record_offset":1144,"ancestor_record_offset":280,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":280,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":996,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":732,"parent_record_offset":692,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1487,"record_offset":852,"parent_record_offset":812,"ancestor_record_offset":280,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":892,"parent_record_offset":852,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":1972,"parent_record_offset":1656,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":280,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":1264,"parent_record_offset":1236,"ancestor_record_offset":280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 434
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13748
// 首条位置: {"module":1487,"inlinee":79688}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30180,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30236,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1487,"record_offset":27936,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1487,"record_offset":28848,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29164,"parent_record_offset":27772,"ancestor_record_offset":27772,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":28940,"parent_record_offset":28912,"ancestor_record_offset":27772,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":28968,"parent_record_offset":28940,"ancestor_record_offset":27772,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":28068,"parent_record_offset":28040,"ancestor_record_offset":27772,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1487,"record_offset":28376,"parent_record_offset":28352,"ancestor_record_offset":27772,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1487,"record_offset":28424,"parent_record_offset":28400,"ancestor_record_offset":27772,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":27980,"parent_record_offset":27936,"ancestor_record_offset":27772,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":28040,"parent_record_offset":28012,"ancestor_record_offset":27772,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":28012,"parent_record_offset":27980,"ancestor_record_offset":27772,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":28448,"parent_record_offset":28424,"ancestor_record_offset":27772,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28304,"parent_record_offset":28196,"ancestor_record_offset":27772,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28248,"parent_record_offset":28196,"ancestor_record_offset":27772,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28220,"parent_record_offset":28196,"ancestor_record_offset":27772,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":28136,"parent_record_offset":28108,"ancestor_record_offset":27772,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":28912,"parent_record_offset":28884,"ancestor_record_offset":27772,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":28884,"parent_record_offset":28848,"ancestor_record_offset":27772,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":28108,"parent_record_offset":27980,"ancestor_record_offset":27772,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":28400,"parent_record_offset":28376,"ancestor_record_offset":27772,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":29232,"parent_record_offset":29164,"ancestor_record_offset":27772,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":29260,"parent_record_offset":29232,"ancestor_record_offset":27772,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30064,"parent_record_offset":29784,"ancestor_record_offset":27772,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30120,"parent_record_offset":29784,"ancestor_record_offset":27772,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30000,"parent_record_offset":29784,"ancestor_record_offset":27772,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1487,"record_offset":28352,"parent_record_offset":27936,"ancestor_record_offset":27772,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1487,"record_offset":28196,"parent_record_offset":28164,"ancestor_record_offset":27772,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":27772,"inlinee":79690,"source_line":443,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29204,"parent_record_offset":29164,"ancestor_record_offset":27772,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":29296,"parent_record_offset":29164,"ancestor_record_offset":27772,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1374A
// 首条位置: {"module":1487,"inlinee":79690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1487,"record_offset":28196,"parent_record_offset":28164,"ancestor_record_offset":28164,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28304,"parent_record_offset":28196,"ancestor_record_offset":28164,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28248,"parent_record_offset":28196,"ancestor_record_offset":28164,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":28220,"parent_record_offset":28196,"ancestor_record_offset":28164,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 494
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1374B
// 首条位置: {"module":1487,"inlinee":79691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30440,"parent_record_offset":30352,"ancestor_record_offset":30352,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30992,"parent_record_offset":30352,"ancestor_record_offset":30352,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":30796,"parent_record_offset":30352,"ancestor_record_offset":30352,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::Guard::push_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30624,"parent_record_offset":30516,"ancestor_record_offset":30352,"inlinee":26464,"source_line":971,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1487,"record_offset":30560,"parent_record_offset":30516,"ancestor_record_offset":30352,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30464,"parent_record_offset":30440,"ancestor_record_offset":30352,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":30352,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1487,"record_offset":30516,"parent_record_offset":30488,"ancestor_record_offset":30352,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1487,"record_offset":30656,"parent_record_offset":30624,"ancestor_record_offset":30352,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":30836,"parent_record_offset":30796,"ancestor_record_offset":30352,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":31016,"parent_record_offset":30992,"ancestor_record_offset":30352,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":30588,"parent_record_offset":30516,"ancestor_record_offset":30352,"inlinee":26462,"source_line":448,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":30756,"parent_record_offset":30488,"ancestor_record_offset":30352,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 509
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC0A
// 首条位置: {"module":3144,"inlinee":117770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3144,"record_offset":580,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3144,"record_offset":724,"parent_record_offset":532,"ancestor_record_offset":532,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3144,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":532,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3144,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":532,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3144,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":532,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3144,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":532,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3144,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":532,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3144,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":532,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":3144,"record_offset":612,"parent_record_offset":580,"ancestor_record_offset":532,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3144,"record_offset":792,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3144,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":532,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3144,"record_offset":1004,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3144,"record_offset":1060,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3144,"record_offset":948,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3144,"record_offset":764,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3144,"record_offset":856,"parent_record_offset":724,"ancestor_record_offset":532,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/03144.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_bottom_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 537
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1373F
// 首条位置: {"module":1487,"inlinee":79679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5124,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":4264,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5720,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":6616,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4944,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4832,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1487,"record_offset":5412,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4740,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":4600,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1487,"record_offset":4872,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":32763,"source_line":38,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":4564,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":6324,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1487,"record_offset":6288,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":5348,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":5496,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":5012,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3716,"parent_record_offset":3548,"ancestor_record_offset":3548,"inlinee":79680,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3548,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3548,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":3548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":3548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3548,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":3548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":3548,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":6488,"parent_record_offset":6396,"ancestor_record_offset":3548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5224,"parent_record_offset":5192,"ancestor_record_offset":3548,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4336,"parent_record_offset":4300,"ancestor_record_offset":3548,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":3548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":3548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":3548,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":3548,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4400,"parent_record_offset":4368,"ancestor_record_offset":3548,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":4368,"parent_record_offset":4336,"ancestor_record_offset":3548,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":3548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":5920,"parent_record_offset":5880,"ancestor_record_offset":3548,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1487,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":3548,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1487,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":3548,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5192,"parent_record_offset":5152,"ancestor_record_offset":3548,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":3548,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":6248,"parent_record_offset":5768,"ancestor_record_offset":3548,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":4300,"parent_record_offset":4264,"ancestor_record_offset":3548,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1487,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":3548,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3548,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":3548,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3548,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1487,"record_offset":6024,"parent_record_offset":5968,"ancestor_record_offset":3548,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3548,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3940,"parent_record_offset":3900,"ancestor_record_offset":3548,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":3548,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":3548,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":4780,"parent_record_offset":4740,"ancestor_record_offset":3548,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3548,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":4212,"parent_record_offset":3940,"ancestor_record_offset":3548,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3548,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3828,"parent_record_offset":3716,"ancestor_record_offset":3548,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":3548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":5996,"parent_record_offset":5968,"ancestor_record_offset":3548,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":6456,"parent_record_offset":6424,"ancestor_record_offset":3548,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":3548,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":3548,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3548,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1487,"record_offset":6192,"parent_record_offset":5768,"ancestor_record_offset":3548,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":3548,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":3548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3548,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1487,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":3548,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":5312,"parent_record_offset":5152,"ancestor_record_offset":3548,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":3548,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":3548,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":3548,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":3548,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1487,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3548,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":3548,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":3548,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4108,"parent_record_offset":4020,"ancestor_record_offset":3548,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3548,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3548,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3548,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3548,"inlinee":79627,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_normal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 584
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13745
// 首条位置: {"module":1487,"inlinee":79685}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":26408,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":26532,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":26504,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26812,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26624,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":27634,"source_line":546,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26568,"parent_record_offset":26360,"ancestor_record_offset":26360,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":26360,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":26592,"parent_record_offset":26568,"ancestor_record_offset":26360,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":26436,"parent_record_offset":26408,"ancestor_record_offset":26360,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":26464,"parent_record_offset":26436,"ancestor_record_offset":26360,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26708,"parent_record_offset":26676,"ancestor_record_offset":26360,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":26360,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":26652,"parent_record_offset":26624,"ancestor_record_offset":26360,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":26740,"parent_record_offset":26708,"ancestor_record_offset":26360,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_arch_heights
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 592
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13741
// 首条位置: {"module":1487,"inlinee":79681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":7740,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":7460,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":8108,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1487,"record_offset":7952,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":8216,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":7432,"parent_record_offset":7356,"ancestor_record_offset":7356,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":7356,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":7356,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":8420,"parent_record_offset":8392,"ancestor_record_offset":7356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":8448,"parent_record_offset":8420,"ancestor_record_offset":7356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":7356,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":7356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":7608,"parent_record_offset":7576,"ancestor_record_offset":7356,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":8392,"parent_record_offset":8292,"ancestor_record_offset":7356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7356,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":7356,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":8324,"parent_record_offset":8292,"ancestor_record_offset":7356,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":8292,"parent_record_offset":8264,"ancestor_record_offset":7356,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7356,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7356,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7576,"parent_record_offset":7544,"ancestor_record_offset":7356,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":7544,"parent_record_offset":7516,"ancestor_record_offset":7356,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":8264,"parent_record_offset":8240,"ancestor_record_offset":7356,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":7804,"parent_record_offset":7768,"ancestor_record_offset":7356,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":7768,"parent_record_offset":7740,"ancestor_record_offset":7356,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":7356,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":7356,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1487,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7356,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":8356,"parent_record_offset":8324,"ancestor_record_offset":7356,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":7356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7356,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":7916,"parent_record_offset":7768,"ancestor_record_offset":7356,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":7356,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":8172,"parent_record_offset":8140,"ancestor_record_offset":7356,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7356,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 606
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13740
// 首条位置: {"module":1487,"inlinee":79680}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3828,"parent_record_offset":3716,"ancestor_record_offset":3716,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":3716,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3716,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3716,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":3940,"parent_record_offset":3900,"ancestor_record_offset":3716,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3716,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1487,"record_offset":4212,"parent_record_offset":3940,"ancestor_record_offset":3716,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3716,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3716,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3716,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1487,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3716,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":3716,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4108,"parent_record_offset":4020,"ancestor_record_offset":3716,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3716,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3716,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3716,"inlinee":79627,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 611
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1370B
// 首条位置: {"module":1482,"inlinee":79627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":3968,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3968,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":4052,"parent_record_offset":4020,"ancestor_record_offset":3968,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4108,"parent_record_offset":4020,"ancestor_record_offset":3968,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3968,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":4140,"parent_record_offset":4108,"ancestor_record_offset":3968,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_brick_steps
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 615
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13744
// 首条位置: {"module":1487,"inlinee":79684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":16276,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18740,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18508,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":20808,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1487,"record_offset":20752,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":16632,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16388,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":17668,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":18536,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":18280,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1487,"record_offset":21008,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":11681,"source_line":201,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":18384,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1487,"record_offset":16360,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":11668,"source_line":50,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":17204,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":26303,"source_line":851,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":16660,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":18480,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1487,"record_offset":16908,"parent_record_offset":16024,"ancestor_record_offset":16024,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":17548,"parent_record_offset":17520,"ancestor_record_offset":16024,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":16024,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":16024,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":17520,"parent_record_offset":17420,"ancestor_record_offset":16024,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":19060,"parent_record_offset":19004,"ancestor_record_offset":16024,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":18868,"parent_record_offset":18840,"ancestor_record_offset":16024,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":18952,"parent_record_offset":18928,"ancestor_record_offset":16024,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":17452,"parent_record_offset":17420,"ancestor_record_offset":16024,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":17420,"parent_record_offset":17392,"ancestor_record_offset":16024,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":18928,"parent_record_offset":18896,"ancestor_record_offset":16024,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":19116,"parent_record_offset":19088,"ancestor_record_offset":16024,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":19088,"parent_record_offset":19060,"ancestor_record_offset":16024,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":16024,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":16024,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":19004,"parent_record_offset":18772,"ancestor_record_offset":16024,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18772,"parent_record_offset":18740,"ancestor_record_offset":16024,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":16024,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":17248,"parent_record_offset":17204,"ancestor_record_offset":16024,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":18840,"parent_record_offset":18772,"ancestor_record_offset":16024,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":16024,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1487,"record_offset":19364,"parent_record_offset":18772,"ancestor_record_offset":16024,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":16988,"parent_record_offset":16964,"ancestor_record_offset":16024,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":16024,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":16492,"parent_record_offset":16432,"ancestor_record_offset":16024,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16536,"parent_record_offset":16432,"ancestor_record_offset":16024,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":16432,"parent_record_offset":16388,"ancestor_record_offset":16024,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1487,"record_offset":19160,"parent_record_offset":18772,"ancestor_record_offset":16024,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":16776,"parent_record_offset":16660,"ancestor_record_offset":16024,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":16576,"parent_record_offset":16536,"ancestor_record_offset":16024,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":17484,"parent_record_offset":17452,"ancestor_record_offset":16024,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":18416,"parent_record_offset":18384,"ancestor_record_offset":16024,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":16964,"parent_record_offset":16940,"ancestor_record_offset":16024,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1487,"record_offset":16940,"parent_record_offset":16908,"ancestor_record_offset":16024,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":17344,"parent_record_offset":17204,"ancestor_record_offset":16024,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1487,"record_offset":16740,"parent_record_offset":16716,"ancestor_record_offset":16024,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1487,"record_offset":18352,"parent_record_offset":18280,"ancestor_record_offset":16024,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1487,"record_offset":18320,"parent_record_offset":18280,"ancestor_record_offset":16024,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":19276,"parent_record_offset":18772,"ancestor_record_offset":16024,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1487,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":16024,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":16692,"parent_record_offset":16660,"ancestor_record_offset":16024,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":20836,"parent_record_offset":20808,"ancestor_record_offset":16024,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":16024,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1487,"record_offset":16716,"parent_record_offset":16692,"ancestor_record_offset":16024,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1487,"record_offset":18596,"parent_record_offset":18560,"ancestor_record_offset":16024,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1487,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":16024,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1487,"record_offset":16832,"parent_record_offset":16660,"ancestor_record_offset":16024,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_brick_steps::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 659
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1378E
// 首条位置: {"module":1493,"inlinee":79758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13742
// 首条位置: {"module":1487,"inlinee":79682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10032,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10088,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":13212,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":12312,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":14008,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":9236,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":9264,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12732,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":9376,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":9556,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12640,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":13240,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":79698,"source_line":814,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::WoodenStairDebugParams::style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1487,"record_offset":13160,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":26140,"source_line":130,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":1487,"record_offset":12432,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":26124,"source_line":68,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":14212,"parent_record_offset":14184,"ancestor_record_offset":9056,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":9056,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":14268,"parent_record_offset":14240,"ancestor_record_offset":9056,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1487,"record_offset":14184,"parent_record_offset":14084,"ancestor_record_offset":9056,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13880,"parent_record_offset":13848,"ancestor_record_offset":9056,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":14116,"parent_record_offset":14084,"ancestor_record_offset":9056,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":14084,"parent_record_offset":14056,"ancestor_record_offset":9056,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":9056,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13904,"parent_record_offset":13880,"ancestor_record_offset":9056,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":9056,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13848,"parent_record_offset":13808,"ancestor_record_offset":9056,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13784,"parent_record_offset":13240,"ancestor_record_offset":9056,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":9056,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":10572,"parent_record_offset":10540,"ancestor_record_offset":9056,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":10220,"parent_record_offset":10192,"ancestor_record_offset":9056,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":9056,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10248,"parent_record_offset":10220,"ancestor_record_offset":9056,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":10368,"parent_record_offset":10276,"ancestor_record_offset":9056,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":10980,"parent_record_offset":10868,"ancestor_record_offset":9056,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":10868,"parent_record_offset":10840,"ancestor_record_offset":9056,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12072,"parent_record_offset":11016,"ancestor_record_offset":9056,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":9056,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":9056,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":9056,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10804,"parent_record_offset":10776,"ancestor_record_offset":9056,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10540,"parent_record_offset":10124,"ancestor_record_offset":9056,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1487,"record_offset":10164,"parent_record_offset":10124,"ancestor_record_offset":9056,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11400,"parent_record_offset":11084,"ancestor_record_offset":9056,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":9056,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":9056,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":9056,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10484,"parent_record_offset":10368,"ancestor_record_offset":9056,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":11060,"parent_record_offset":11016,"ancestor_record_offset":9056,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10840,"parent_record_offset":10804,"ancestor_record_offset":9056,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":10124,"parent_record_offset":10088,"ancestor_record_offset":9056,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1487,"record_offset":10400,"parent_record_offset":10368,"ancestor_record_offset":9056,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":9056,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":14148,"parent_record_offset":14116,"ancestor_record_offset":9056,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":13756,"parent_record_offset":13240,"ancestor_record_offset":9056,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":9056,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":9056,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":12672,"parent_record_offset":12640,"ancestor_record_offset":9056,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":10896,"parent_record_offset":10868,"ancestor_record_offset":9056,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::assume_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10712,"parent_record_offset":10668,"ancestor_record_offset":9056,"inlinee":10923,"source_line":120,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10312,"parent_record_offset":10276,"ancestor_record_offset":9056,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":9056,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":10776,"parent_record_offset":10740,"ancestor_record_offset":9056,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":9056,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1487,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":9056,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":9056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":10448,"parent_record_offset":10424,"ancestor_record_offset":9056,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":13968,"parent_record_offset":13808,"ancestor_record_offset":9056,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":9056,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":9056,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13576,"parent_record_offset":13240,"ancestor_record_offset":9056,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13656,"parent_record_offset":13600,"ancestor_record_offset":9056,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":9056,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11296,"parent_record_offset":11184,"ancestor_record_offset":9056,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":9056,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12872,"parent_record_offset":12836,"ancestor_record_offset":9056,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12836,"parent_record_offset":12808,"ancestor_record_offset":9056,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12808,"parent_record_offset":12768,"ancestor_record_offset":9056,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":12768,"parent_record_offset":12732,"ancestor_record_offset":9056,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":9288,"parent_record_offset":9264,"ancestor_record_offset":9056,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12100,"parent_record_offset":11016,"ancestor_record_offset":9056,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12908,"parent_record_offset":12872,"ancestor_record_offset":9056,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":9056,"inlinee":79610,"source_line":826,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11016,"parent_record_offset":10804,"ancestor_record_offset":9056,"inlinee":79696,"source_line":741,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":9056,"inlinee":79642,"source_line":744,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":9056,"inlinee":79643,"source_line":745,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":9056,"inlinee":79697,"source_line":756,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":9056,"inlinee":79695,"source_line":757,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 741
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13750
// 首条位置: {"module":1487,"inlinee":79696}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3150,"record_offset":1600,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/03150.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12072,"parent_record_offset":11016,"ancestor_record_offset":11016,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":11060,"parent_record_offset":11016,"ancestor_record_offset":11016,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12100,"parent_record_offset":11016,"ancestor_record_offset":11016,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1487,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":11016,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":11016,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":11016,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1487,"record_offset":11524,"parent_record_offset":11496,"ancestor_record_offset":11016,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":11996,"parent_record_offset":11884,"ancestor_record_offset":11016,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1487,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":11016,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":11016,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11400,"parent_record_offset":11084,"ancestor_record_offset":11016,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11112,"parent_record_offset":11084,"ancestor_record_offset":11016,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":11016,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":11860,"parent_record_offset":11632,"ancestor_record_offset":11016,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":11016,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":11016,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":11236,"parent_record_offset":11208,"ancestor_record_offset":11016,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":11016,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1487,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":11016,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1487,"record_offset":11608,"parent_record_offset":11524,"ancestor_record_offset":11016,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":11016,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1487,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":11016,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1487,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":11016,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":11016,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11296,"parent_record_offset":11184,"ancestor_record_offset":11016,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":11016,"inlinee":79642,"source_line":744,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":11016,"inlinee":79643,"source_line":745,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 744
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1371A
// 首条位置: {"module":1482,"inlinee":79642}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":11296,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":11184,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":11236,"parent_record_offset":11208,"ancestor_record_offset":11184,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 745
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1371B
// 首条位置: {"module":1482,"inlinee":79643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":11936,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 756
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13751
// 首条位置: {"module":1487,"inlinee":79697}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_wooden_steps::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 757
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1374F
// 首条位置: {"module":1487,"inlinee":79695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12368,"parent_record_offset":12336,"ancestor_record_offset":12336,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":12396,"parent_record_offset":12336,"ancestor_record_offset":12336,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 814
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13752
// 首条位置: {"module":1487,"inlinee":79698}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13784,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":13292,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1487,"record_offset":13756,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13576,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13520,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13464,"parent_record_offset":13240,"ancestor_record_offset":13240,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13880,"parent_record_offset":13848,"ancestor_record_offset":13240,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13928,"parent_record_offset":13904,"ancestor_record_offset":13240,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":13904,"parent_record_offset":13880,"ancestor_record_offset":13240,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13848,"parent_record_offset":13808,"ancestor_record_offset":13240,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":13240,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1487,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":13240,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1487,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":13240,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1487,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":13240,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":13240,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":13544,"parent_record_offset":13520,"ancestor_record_offset":13240,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":13628,"parent_record_offset":13600,"ancestor_record_offset":13240,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1487,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":13240,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1487,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":13240,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1487,"record_offset":13968,"parent_record_offset":13808,"ancestor_record_offset":13240,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13656,"parent_record_offset":13600,"ancestor_record_offset":13240,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1487,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":13240,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1487,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":13240,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1487,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":13240,"inlinee":79610,"source_line":826,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 826
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136FA
// 首条位置: {"module":1482,"inlinee":79610}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 827
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136FB
// 首条位置: {"module":1482,"inlinee":79611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":4972,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_ladder
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 851
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13743
// 首条位置: {"module":1487,"inlinee":79683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15476,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":15116,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":15236,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1487,"record_offset":15356,"parent_record_offset":15056,"ancestor_record_offset":15056,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15660,"parent_record_offset":15632,"ancestor_record_offset":15056,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15744,"parent_record_offset":15720,"ancestor_record_offset":15056,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15720,"parent_record_offset":15688,"ancestor_record_offset":15056,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1487,"record_offset":15688,"parent_record_offset":15660,"ancestor_record_offset":15056,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1487,"record_offset":15632,"parent_record_offset":15596,"ancestor_record_offset":15056,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15568,"parent_record_offset":15540,"ancestor_record_offset":15056,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15596,"parent_record_offset":15568,"ancestor_record_offset":15056,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15512,"parent_record_offset":15476,"ancestor_record_offset":15056,"inlinee":11375,"source_line":262,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":1487,"record_offset":15540,"parent_record_offset":15476,"ancestor_record_offset":15056,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":15264,"parent_record_offset":15236,"ancestor_record_offset":15056,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":15056,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1487,"record_offset":15384,"parent_record_offset":15356,"ancestor_record_offset":15056,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01487.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::add_mirrored_quad
// 信息 ID: f6fb21053836805dd9278f94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x14324fa -> 0x142fb90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1432882 -> 0x142fb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
// 信息 ID: 019d618bff695f8a6ee65c50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=31108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler
// 信息 ID: d44323dd9de4714067ff8b9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=13240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$0
// 信息 ID: 9dd2f9fc64c6471aab160c72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$0 <- core::iter::adapters::map::impl$2::fold<f32,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$0,f32,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<f32,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$0,f32,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<f32,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$1> >; module=1482, inline_record=4868; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=13416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure$1
// 信息 ID: 9c5e4c6e8d5685b72d3da2b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$1 <- core::iter::adapters::map::impl$2::fold<f32,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$0,f32,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<f32,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$0,f32,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<f32,system_wall_constructor::construct_gates::add_wooden_gate_terrain_filler::closure_env$1> >; module=1482, inline_record=4972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 信息 ID: c53f1c8ca82ab124e4c534cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x8017c8 -> 0x14301f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 信息 ID: 371c63197b9c7c74232a96b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::construct_wooden_gate_doors", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0> >; module=1455, inline_record=132988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_arch_heights
// 信息 ID: e110d6c87d911aeb72107ef3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=7356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_bottom_curve
// 信息 ID: f57633358215c3424bfbe5d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=3548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh
// 信息 ID: c164b2d75e1188cf56209301
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=27772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh::closure$0
// 信息 ID: 473db04d1bde5f6e12a1d131
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=28164; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=28636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap
// 信息 ID: e44dac846c8d1c710d0ddd99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=30352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 信息 ID: ca659e62a752398d56caab40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh_gap", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0 <- core::ops::try_trait::impl$1::call_mut<tuple$<f32,glam::f32::vec3::Vec3,f32,f32>,tuple$<f32,f32>,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::try_trait::impl$1::call_mut<tuple$<f32,glam::f32::vec3::Vec3,f32,f32>,tuple$<f32,f32>,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; module=3144, inline_record=532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_normal
// 信息 ID: c64de3959e4a5c6b18704aa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=26360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::impl$5::default
// 信息 ID: 119bce9d94eee9a6357ef6c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=27744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_brick_steps
// 信息 ID: b01edbebd72b236f40c9faee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=16024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_brick_steps::closure$0
// 信息 ID: 09d1a7bbe86e15e95126e050
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::place_brick_steps", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::place_brick_steps::closure_env$0 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_gates::place_brick_steps::closure_env$0 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_gates::place_brick_steps::closure_env$0 <- system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=94344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_ladder
// 信息 ID: 1b77599d3cc426578d098cc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=15056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps
// 信息 ID: 157707c25a2ddb9a1e8ca99c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=9056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps::closure$0
// 信息 ID: ab67e08a29110b00d3981088
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::place_wooden_steps", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<f32>,system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=11016; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<f32>,system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0>; module=3150, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$0
// 信息 ID: cea417d8776c6d7c10f774a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::place_wooden_steps", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=11184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=11656; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<f32>,system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0>; module=3150, inline_record=672; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<f32>,system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0>; module=3150, inline_record=1052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps::closure$0::closure$1
// 信息 ID: 92241ad42556d1b0707c9ccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::place_wooden_steps", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=11936; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<f32>,system_wall_constructor::construct_gates::place_wooden_steps::closure_env$0>; module=3150, inline_record=1396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps::closure$1
// 信息 ID: df27dc4a61c41e9305121511
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_gates::place_wooden_steps", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=10920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_wooden_steps::closure$2
// 信息 ID: 1e93383ddb8bd2224d110e58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_gates::place_wooden_steps", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=12336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length
// 信息 ID: 4b630090777ca11829fd530a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=3716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length::closure$0
// 信息 ID: f6132b699a4c9ce3d5c877ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::segment_length", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=3968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
