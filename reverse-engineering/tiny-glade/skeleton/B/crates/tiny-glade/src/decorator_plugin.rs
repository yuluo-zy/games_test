// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/decorator_plugin.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4c25b80182f6a0099aa0a8eb6897d5e6f4c829d9070892afdc11fc00d031fc61
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_plugin.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 141
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::decorator_plugin::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":141,"record_offset":65468,"rva":3419152,"va":5372128272}
// 编译实例 RVAs: 0x342c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3411970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8feefcbff","size":5,"soft_fail":false,"target_va":5368718192,"va":5372130157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=7
//     target_aliases=53 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3265553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ef2bceff","size":5,"soft_fail":false,"target_va":5368879232,"va":5372144780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3122967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8e958d0ff","size":5,"soft_fail":false,"target_va":5369021504,"va":5372144466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c5c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3117047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e80970d0ff","size":5,"soft_fail":false,"target_va":5369021888,"va":5372138930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004d1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3116807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8f970d0ff","size":5,"soft_fail":false,"target_va":5369024960,"va":5372141762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004dfa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3111255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8a986d0ff","size":5,"soft_fail":false,"target_va":5369028512,"va":5372139762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050900
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3099655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8f9b3d0ff","size":5,"soft_fail":false,"target_va":5369039104,"va":5372138754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3099751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e899b3d0ff","size":5,"soft_fail":false,"target_va":5369039360,"va":5372139106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140051580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3097815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e829bbd0ff","size":5,"soft_fail":false,"target_va":5369042304,"va":5372140114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400525f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3096695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e889bfd0ff","size":5,"soft_fail":false,"target_va":5369046512,"va":5372143202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140052c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3092791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8c9ced0ff","size":5,"soft_fail":false,"target_va":5369048176,"va":5372140962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c9130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2605622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ca3dd8ff","size":5,"soft_fail":false,"target_va":5369532720,"va":5372138337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=10
//     target_aliases=35 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2528812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8d469d9ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5372129607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=9
//     target_aliases=19 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2512285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e863aad9ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5372128392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140105360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2357963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e83505dcff","size":5,"soft_fail":false,"target_va":5369779040,"va":5372136998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010b940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2327814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8fa7adcff","size":5,"soft_fail":false,"target_va":5369805120,"va":5372132929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010ba60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2327504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8307cdcff","size":5,"soft_fail":false,"target_va":5369805408,"va":5372132907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149c70
//     owner=unresolved_namespace; count=4
//     target_aliases=46 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2081700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e85c3ce0ff","size":5,"soft_fail":false,"target_va":5370059888,"va":5372141583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=5
//     target_aliases=27 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2078612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e86c48e0ff","size":5,"soft_fail":false,"target_va":5370061664,"va":5372140271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ab60
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2075876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e81c53e0ff","size":5,"soft_fail":false,"target_va":5370063712,"va":5372139583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b650
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2077620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e84c4ce0ff","size":5,"soft_fail":false,"target_va":5370066512,"va":5372144127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140244140
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1047956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e86c02f0ff","size":5,"soft_fail":false,"target_va":5371085120,"va":5372133071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402442d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1050034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e84efaefff","size":5,"soft_fail":false,"target_va":5371085520,"va":5372135549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140258060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-971298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8de2df1ff","size":5,"soft_fail":false,"target_va":5371166816,"va":5372138109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026c790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-886666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e87678f2ff","size":5,"soft_fail":false,"target_va":5371250576,"va":5372137237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140286610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-780578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8de16f4ff","size":5,"soft_fail":false,"target_va":5371356688,"va":5372137261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140287410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-776982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ea24f4ff","size":5,"soft_fail":false,"target_va":5371360272,"va":5372137249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402957c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-718423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8a909f5ff","size":5,"soft_fail":false,"target_va":5371418560,"va":5372136978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140322fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-139949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e853ddfdff","size":5,"soft_fail":false,"target_va":5371998144,"va":5372138088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140327350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-121507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e85d25feff","size":5,"soft_fail":false,"target_va":5372015440,"va":5372136942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403276f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-120626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ce28feff","size":5,"soft_fail":false,"target_va":5372016368,"va":5372136989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140327950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-121349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8fb25feff","size":5,"soft_fail":false,"target_va":5372016976,"va":5372138320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t129867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e84bfb0100","size":5,"soft_fail":false,"target_va":5372267808,"va":5372137936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t597293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e82d1d0900","size":5,"soft_fail":false,"target_va":5372734688,"va":5372137390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140492d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1367618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e842de1400","size":5,"soft_fail":false,"target_va":5373504896,"va":5372137273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140553030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2154989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ede12000","size":5,"soft_fail":false,"target_va":5374292016,"va":5372137022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2701183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e87f372900","size":5,"soft_fail":false,"target_va":5374842464,"va":5372141276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dcd50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2714604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ec6b2900","size":5,"soft_fail":false,"target_va":5374856528,"va":5372141919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dd470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2716108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8cc712900","size":5,"soft_fail":false,"target_va":5374858352,"va":5372142239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e17b0
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2736300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8acc02900","size":5,"soft_fail":false,"target_va":5374875568,"va":5372139263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2734268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8bcb82900","size":5,"soft_fail":false,"target_va":5374878896,"va":5372144623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e2db0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2739772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e83cce2900","size":5,"soft_fail":false,"target_va":5374881200,"va":5372141423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e37f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2740460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8ecd02900","size":5,"soft_fail":false,"target_va":5374883824,"va":5372143359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406088b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2894780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8bc2b2c00","size":5,"soft_fail":false,"target_va":5375035568,"va":5372140783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39714582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e816ff5d02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372132581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=37
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2288960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8c012ddff","size":5,"soft_fail":false,"target_va":5369840192,"va":5372129147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2367107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8831e2400","size":5,"soft_fail":false,"target_va":5374495568,"va":5372128456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::after
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2698738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e80ed2d6ff","size":5,"soft_fail":false,"target_va":5369438224,"va":5372136957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-460327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8d9f9f8ff","size":5,"soft_fail":false,"target_va":5371676688,"va":5372137010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2550055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8d916d9ff","size":5,"soft_fail":false,"target_va":5369587904,"va":5372137954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::schedule::Schedules::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1916475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8c5c1e2ff","size":5,"soft_fail":false,"target_va":5370220464,"va":5372136934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39403339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e84b3f5902","size":5,"soft_fail":false,"target_va":5411541904,"va":5372138560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e839e2ffff","size":5,"soft_fail":false,"target_va":5372121968,"va":5372129586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=47
//     evidence={"assembly":"\tcallq\t10528696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8b8a7a000","size":5,"soft_fail":false,"target_va":5382657856,"va":5372129155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=47
//     evidence={"assembly":"\tcallq\t10624602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e85a1ea200","size":5,"soft_fail":false,"target_va":5382753792,"va":5372129185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t39398658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372128272,"raw_hex":"e8022d5902","size":5,"soft_fail":false,"target_va":5411537104,"va":5372138441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140326f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-123970","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372128272,"raw_hex":"e9be1bfeff","size":5,"soft_fail":false,"target_va":5372014464,"va":5372138429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":141,"record_offset":65732,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":141,"record_offset":112476,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":141,"record_offset":113688,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":141,"record_offset":91876,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12860,"source_line":10,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_assembly_params::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":141,"record_offset":92608,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12862,"source_line":21,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_collider::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_collider.rs
//     evidence={"module":141,"record_offset":99340,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12878,"source_line":7,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_removed_supports::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_removed_supports.rs
//     evidence={"module":141,"record_offset":100984,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12880,"source_line":8,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":141,"record_offset":97364,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12872,"source_line":16,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$31::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":141,"record_offset":97876,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12874,"source_line":73,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::structural_graph::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/structural_graph.rs
//     evidence={"module":141,"record_offset":105400,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12896,"source_line":23,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":141,"record_offset":98608,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12876,"source_line":28,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":141,"record_offset":94992,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12864,"source_line":11,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":141,"record_offset":95020,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12866,"source_line":41,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":141,"record_offset":95048,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12868,"source_line":58,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":141,"record_offset":76080,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12836,"source_line":10,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$143::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":141,"record_offset":71424,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12819,"source_line":757,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":141,"record_offset":86088,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12856,"source_line":4,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":141,"record_offset":71512,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12824,"source_line":414,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":141,"record_offset":80592,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12848,"source_line":78,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":141,"record_offset":81324,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12850,"source_line":131,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":141,"record_offset":79832,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12844,"source_line":17,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":141,"record_offset":96632,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12870,"source_line":35,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":141,"record_offset":76896,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12842,"source_line":34,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::backup::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/backup.rs
//     evidence={"module":141,"record_offset":86912,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12858,"source_line":17,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":84596,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12852,"source_line":268,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":85328,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12854,"source_line":273,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":74436,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12830,"source_line":76,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":141,"record_offset":73968,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12828,"source_line":17,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_decorator_affordance_dispatch::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs
//     evidence={"module":141,"record_offset":80564,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12846,"source_line":76,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":141,"record_offset":109540,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12898,"source_line":61,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":141,"record_offset":111012,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12900,"source_line":71,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":141,"record_offset":111744,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12902,"source_line":79,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_move_decorator::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_move_decorator.rs
//     evidence={"module":141,"record_offset":76840,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12838,"source_line":85,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_place_decorator::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs
//     evidence={"module":141,"record_offset":76868,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12840,"source_line":60,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_stairs_viz::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":101440,"parent_record_offset":65468,"ancestor_record_offset":65468,"inlinee":12882,"source_line":104,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":66196,"parent_record_offset":66172,"ancestor_record_offset":65468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":66244,"parent_record_offset":66220,"ancestor_record_offset":65468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":66220,"parent_record_offset":66196,"ancestor_record_offset":65468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":66172,"parent_record_offset":66148,"ancestor_record_offset":65468,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":66148,"parent_record_offset":66116,"ancestor_record_offset":65468,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":92688,"parent_record_offset":92664,"ancestor_record_offset":65468,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":92664,"parent_record_offset":92608,"ancestor_record_offset":65468,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":66116,"parent_record_offset":66092,"ancestor_record_offset":65468,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":141,"record_offset":65756,"parent_record_offset":65732,"ancestor_record_offset":65468,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":141,"record_offset":112504,"parent_record_offset":112476,"ancestor_record_offset":65468,"inlinee":9172,"source_line":211,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":141,"record_offset":113592,"parent_record_offset":113568,"ancestor_record_offset":65468,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":141,"record_offset":113568,"parent_record_offset":112504,"ancestor_record_offset":65468,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":141,"record_offset":113728,"parent_record_offset":113688,"ancestor_record_offset":65468,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":141,"record_offset":113340,"parent_record_offset":113316,"ancestor_record_offset":65468,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":141,"record_offset":113316,"parent_record_offset":113292,"ancestor_record_offset":65468,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":141,"record_offset":66388,"parent_record_offset":66364,"ancestor_record_offset":65468,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":141,"record_offset":113208,"parent_record_offset":112668,"ancestor_record_offset":65468,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":141,"record_offset":66364,"parent_record_offset":66340,"ancestor_record_offset":65468,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":141,"record_offset":65864,"parent_record_offset":65840,"ancestor_record_offset":65468,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":141,"record_offset":113292,"parent_record_offset":113232,"ancestor_record_offset":65468,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":141,"record_offset":65912,"parent_record_offset":65888,"ancestor_record_offset":65468,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":141,"record_offset":65888,"parent_record_offset":65864,"ancestor_record_offset":65468,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":141,"record_offset":66092,"parent_record_offset":66068,"ancestor_record_offset":65468,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":141,"record_offset":65784,"parent_record_offset":65756,"ancestor_record_offset":65468,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":141,"record_offset":112576,"parent_record_offset":112540,"ancestor_record_offset":65468,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":141,"record_offset":113408,"parent_record_offset":113384,"ancestor_record_offset":65468,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":141,"record_offset":112540,"parent_record_offset":112504,"ancestor_record_offset":65468,"inlinee":9173,"source_line":2000,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":141,"record_offset":65840,"parent_record_offset":65816,"ancestor_record_offset":65468,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":141,"record_offset":113384,"parent_record_offset":112668,"ancestor_record_offset":65468,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":141,"record_offset":112628,"parent_record_offset":112600,"ancestor_record_offset":65468,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":141,"record_offset":112600,"parent_record_offset":112576,"ancestor_record_offset":65468,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":141,"record_offset":112668,"parent_record_offset":112600,"ancestor_record_offset":65468,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":141,"record_offset":113508,"parent_record_offset":112540,"ancestor_record_offset":65468,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":72084,"parent_record_offset":72052,"ancestor_record_offset":65468,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":72144,"parent_record_offset":72112,"ancestor_record_offset":65468,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":74288,"parent_record_offset":74232,"ancestor_record_offset":65468,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":141,"record_offset":113536,"parent_record_offset":112540,"ancestor_record_offset":65468,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=104
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":141,"record_offset":71940,"parent_record_offset":71916,"ancestor_record_offset":65468,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":72112,"parent_record_offset":72052,"ancestor_record_offset":65468,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":141,"record_offset":66004,"parent_record_offset":65940,"ancestor_record_offset":65468,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":65816,"parent_record_offset":65784,"ancestor_record_offset":65468,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":66028,"parent_record_offset":66004,"ancestor_record_offset":65468,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":66316,"parent_record_offset":65784,"ancestor_record_offset":65468,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":66068,"parent_record_offset":65888,"ancestor_record_offset":65468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":71804,"parent_record_offset":71776,"ancestor_record_offset":65468,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":141,"record_offset":74204,"parent_record_offset":74172,"ancestor_record_offset":65468,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":73740,"parent_record_offset":71512,"ancestor_record_offset":65468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":141,"record_offset":71892,"parent_record_offset":71776,"ancestor_record_offset":65468,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":71968,"parent_record_offset":71940,"ancestor_record_offset":65468,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":71916,"parent_record_offset":71892,"ancestor_record_offset":65468,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=104
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":71996,"parent_record_offset":71940,"ancestor_record_offset":65468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":141,"record_offset":65940,"parent_record_offset":65912,"ancestor_record_offset":65468,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":141,"record_offset":113156,"parent_record_offset":113132,"ancestor_record_offset":65468,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":141,"record_offset":65976,"parent_record_offset":65940,"ancestor_record_offset":65468,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":141,"record_offset":113132,"parent_record_offset":112952,"ancestor_record_offset":65468,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":141,"record_offset":113432,"parent_record_offset":113408,"ancestor_record_offset":65468,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":141,"record_offset":113456,"parent_record_offset":113432,"ancestor_record_offset":65468,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":141,"record_offset":90160,"parent_record_offset":86912,"ancestor_record_offset":65468,"inlinee":12836,"source_line":10,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$5::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":141,"record_offset":76108,"parent_record_offset":76080,"ancestor_record_offset":65468,"inlinee":7797,"source_line":16,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$143::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":141,"record_offset":91648,"parent_record_offset":86912,"ancestor_record_offset":65468,"inlinee":12819,"source_line":757,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$21::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":141,"record_offset":73012,"parent_record_offset":71512,"ancestor_record_offset":65468,"inlinee":12826,"source_line":342,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":141,"record_offset":86972,"parent_record_offset":86912,"ancestor_record_offset":65468,"inlinee":12824,"source_line":414,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":141,"record_offset":90920,"parent_record_offset":86912,"ancestor_record_offset":65468,"inlinee":12848,"source_line":78,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":141,"record_offset":86136,"parent_record_offset":86088,"ancestor_record_offset":65468,"inlinee":10944,"source_line":285,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":141,"record_offset":101396,"parent_record_offset":101056,"ancestor_record_offset":65468,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":141,"record_offset":74060,"parent_record_offset":74032,"ancestor_record_offset":65468,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":73812,"parent_record_offset":73788,"ancestor_record_offset":65468,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":141,"record_offset":74032,"parent_record_offset":74004,"ancestor_record_offset":65468,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":141,"record_offset":71476,"parent_record_offset":71452,"ancestor_record_offset":65468,"inlinee":12822,"source_line":371,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":141,"record_offset":71452,"parent_record_offset":71424,"ancestor_record_offset":65468,"inlinee":12821,"source_line":1151,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":141,"record_offset":72252,"parent_record_offset":71556,"ancestor_record_offset":65468,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":141,"record_offset":71556,"parent_record_offset":71512,"ancestor_record_offset":65468,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":141,"record_offset":74004,"parent_record_offset":73968,"ancestor_record_offset":65468,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":71604,"parent_record_offset":71580,"ancestor_record_offset":65468,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":141,"record_offset":71852,"parent_record_offset":71828,"ancestor_record_offset":65468,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":141,"record_offset":71716,"parent_record_offset":71688,"ancestor_record_offset":65468,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":72052,"parent_record_offset":71652,"ancestor_record_offset":65468,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":71580,"parent_record_offset":71556,"ancestor_record_offset":65468,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":141,"record_offset":71744,"parent_record_offset":71716,"ancestor_record_offset":65468,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":141,"record_offset":71776,"parent_record_offset":71744,"ancestor_record_offset":65468,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":141,"record_offset":71652,"parent_record_offset":71628,"ancestor_record_offset":65468,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":141,"record_offset":71628,"parent_record_offset":71604,"ancestor_record_offset":65468,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":85356,"parent_record_offset":85328,"ancestor_record_offset":65468,"inlinee":12852,"source_line":268,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":74476,"parent_record_offset":74436,"ancestor_record_offset":65468,"inlinee":12832,"source_line":29,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":75208,"parent_record_offset":74436,"ancestor_record_offset":65468,"inlinee":12834,"source_line":40,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104472,"parent_record_offset":101440,"ancestor_record_offset":65468,"inlinee":12888,"source_line":60,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104500,"parent_record_offset":101440,"ancestor_record_offset":65468,"inlinee":12890,"source_line":67,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104528,"parent_record_offset":101440,"ancestor_record_offset":65468,"inlinee":12892,"source_line":74,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104556,"parent_record_offset":101440,"ancestor_record_offset":65468,"inlinee":12894,"source_line":81,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104416,"parent_record_offset":101440,"ancestor_record_offset":65468,"inlinee":12884,"source_line":88,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":141,"record_offset":104440,"parent_record_offset":104416,"ancestor_record_offset":65468,"inlinee":12886,"source_line":1207,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::decorator_plugin::decorators_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 357
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":141,"record_offset":113876,"rva":3436688,"va":5372145808}
// 编译实例 RVAs: 0x347090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3429917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8e3a9cbff","size":5,"soft_fail":false,"target_va":5368718192,"va":5372148104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3133336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e86830d0ff","size":5,"soft_fail":false,"target_va":5369021440,"va":5372154771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3129543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8393fd0ff","size":5,"soft_fail":false,"target_va":5369021888,"va":5372151426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004cda0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3126705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e84f4ad0ff","size":5,"soft_fail":false,"target_va":5369023904,"va":5372150604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004d2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3124585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e89752d0ff","size":5,"soft_fail":false,"target_va":5369025232,"va":5372149812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004d5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3124907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e85551d0ff","size":5,"soft_fail":false,"target_va":5369025952,"va":5372150854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004eb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3118255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8516bd0ff","size":5,"soft_fail":false,"target_va":5369031440,"va":5372149690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3115319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8c976d0ff","size":5,"soft_fail":false,"target_va":5369036880,"va":5372152194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3112193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8ff82d0ff","size":5,"soft_fail":false,"target_va":5369039104,"va":5372151292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140050a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3112215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8e982d0ff","size":5,"soft_fail":false,"target_va":5369039360,"va":5372151570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140052c70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3099081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e837b6d0ff","size":5,"soft_fail":false,"target_va":5369048176,"va":5372147252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=9
//     target_aliases=35 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2546763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8b523d9ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5372147558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010b940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2345450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e81636dcff","size":5,"soft_fail":false,"target_va":5369805120,"va":5372150565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010ba60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2344370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e84e3adcff","size":5,"soft_fail":false,"target_va":5369805408,"va":5372149773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a360
//     owner=unresolved_namespace; count=1
//     target_aliases=27 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2090388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e86c1ae0ff","size":5,"soft_fail":false,"target_va":5370061664,"va":5372152047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ab60
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2088212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8ec22e0ff","size":5,"soft_fail":false,"target_va":5370063712,"va":5372151919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140244140
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1064826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e886c0efff","size":5,"soft_fail":false,"target_va":5371085120,"va":5372149941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cf990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2652862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8be7a2800","size":5,"soft_fail":false,"target_va":5374802320,"va":5372149453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dcd50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2703999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e87f422900","size":5,"soft_fail":false,"target_va":5374856528,"va":5372152524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dd470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2705343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8bf472900","size":5,"soft_fail":false,"target_va":5374858352,"va":5372153004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e17b0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2723871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e81f902900","size":5,"soft_fail":false,"target_va":5374875568,"va":5372151692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2861451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e88ba92b00","size":5,"soft_fail":false,"target_va":5375011520,"va":5372150064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140605bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2876117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8d5e22b00","size":5,"soft_fail":false,"target_va":5375024112,"va":5372147990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406061f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2876951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e817e62b00","size":5,"soft_fail":false,"target_va":5375025648,"va":5372148692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406088b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2888593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e891132c00","size":5,"soft_fail":false,"target_va":5375035568,"va":5372146970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060a9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2893562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8fa262c00","size":5,"soft_fail":false,"target_va":5375044064,"va":5372150497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060aae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2896789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e895332c00","size":5,"soft_fail":false,"target_va":5375044320,"va":5372147526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2933882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e87ac42c00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372147617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39699612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e89cc45d02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372147551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39167123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e893a45502","size":5,"soft_fail":false,"target_va":5411318240,"va":5372151112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5379469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e88d155200","size":5,"soft_fail":false,"target_va":5377530416,"va":5372150942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e83c9cffff","size":5,"soft_fail":false,"target_va":5372121968,"va":5372147503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t10511254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e89663a000","size":5,"soft_fail":false,"target_va":5382657856,"va":5372146597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t10607160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e838daa100","size":5,"soft_fail":false,"target_va":5382753792,"va":5372146627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t39385973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e875fb5802","size":5,"soft_fail":false,"target_va":5411537104,"va":5372151126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4615794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372145808,"raw_hex":"e8726e4600","size":5,"soft_fail":false,"target_va":5376766768,"va":5372150969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":141,"record_offset":116536,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":141,"record_offset":134888,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":4958,"source_line":872,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":117376,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":141,"record_offset":129132,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12860,"source_line":10,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_collider::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_collider.rs
//     evidence={"module":141,"record_offset":131372,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12878,"source_line":7,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":141,"record_offset":130828,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12872,"source_line":16,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":141,"record_offset":133848,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12876,"source_line":28,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":141,"record_offset":125912,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12856,"source_line":4,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":141,"record_offset":114080,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12824,"source_line":414,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":141,"record_offset":127780,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12904,"source_line":15,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":141,"record_offset":124648,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12844,"source_line":17,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":141,"record_offset":121656,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12842,"source_line":34,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":116592,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12852,"source_line":268,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":117520,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12854,"source_line":273,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":119816,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12830,"source_line":76,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":141,"record_offset":118560,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12828,"source_line":17,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_place_decorator::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs
//     evidence={"module":141,"record_offset":121572,"parent_record_offset":113876,"ancestor_record_offset":113876,"inlinee":12840,"source_line":60,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":141,"record_offset":119280,"parent_record_offset":119256,"ancestor_record_offset":113876,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":141,"record_offset":119256,"parent_record_offset":119164,"ancestor_record_offset":113876,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":135004,"parent_record_offset":134976,"ancestor_record_offset":113876,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":135088,"parent_record_offset":135064,"ancestor_record_offset":113876,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":119192,"parent_record_offset":119164,"ancestor_record_offset":113876,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":119164,"parent_record_offset":119140,"ancestor_record_offset":113876,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":135064,"parent_record_offset":135032,"ancestor_record_offset":113876,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":135032,"parent_record_offset":135004,"ancestor_record_offset":113876,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":141,"record_offset":119140,"parent_record_offset":119116,"ancestor_record_offset":113876,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":141,"record_offset":134976,"parent_record_offset":134944,"ancestor_record_offset":113876,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":141,"record_offset":116560,"parent_record_offset":116536,"ancestor_record_offset":113876,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":141,"record_offset":134916,"parent_record_offset":134888,"ancestor_record_offset":113876,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":141,"record_offset":134944,"parent_record_offset":134916,"ancestor_record_offset":113876,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":114652,"parent_record_offset":114620,"ancestor_record_offset":113876,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":114712,"parent_record_offset":114680,"ancestor_record_offset":113876,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":141,"record_offset":118840,"parent_record_offset":118784,"ancestor_record_offset":113876,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":141,"record_offset":114508,"parent_record_offset":114484,"ancestor_record_offset":113876,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":130528,"parent_record_offset":130500,"ancestor_record_offset":113876,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":119628,"parent_record_offset":119600,"ancestor_record_offset":113876,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":119656,"parent_record_offset":119628,"ancestor_record_offset":113876,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":119224,"parent_record_offset":119192,"ancestor_record_offset":113876,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":141,"record_offset":114680,"parent_record_offset":114620,"ancestor_record_offset":113876,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":141,"record_offset":114256,"parent_record_offset":114220,"ancestor_record_offset":113876,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":141,"record_offset":114372,"parent_record_offset":114344,"ancestor_record_offset":113876,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":141,"record_offset":118756,"parent_record_offset":118728,"ancestor_record_offset":113876,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":116308,"parent_record_offset":114080,"ancestor_record_offset":113876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":141,"record_offset":114460,"parent_record_offset":114344,"ancestor_record_offset":113876,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":141,"record_offset":119692,"parent_record_offset":119540,"ancestor_record_offset":113876,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":114536,"parent_record_offset":114508,"ancestor_record_offset":113876,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":114484,"parent_record_offset":114460,"ancestor_record_offset":113876,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":141,"record_offset":114564,"parent_record_offset":114508,"ancestor_record_offset":113876,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":141,"record_offset":115580,"parent_record_offset":114080,"ancestor_record_offset":113876,"inlinee":12826,"source_line":342,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":141,"record_offset":125960,"parent_record_offset":125912,"ancestor_record_offset":113876,"inlinee":10944,"source_line":285,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":141,"record_offset":118632,"parent_record_offset":118608,"ancestor_record_offset":113876,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119540,"parent_record_offset":119508,"ancestor_record_offset":113876,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119572,"parent_record_offset":119540,"ancestor_record_offset":113876,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119452,"parent_record_offset":119428,"ancestor_record_offset":113876,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119508,"parent_record_offset":119452,"ancestor_record_offset":113876,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119480,"parent_record_offset":119452,"ancestor_record_offset":113876,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":119600,"parent_record_offset":119540,"ancestor_record_offset":113876,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":141,"record_offset":116380,"parent_record_offset":116356,"ancestor_record_offset":113876,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":141,"record_offset":118608,"parent_record_offset":118584,"ancestor_record_offset":113876,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":141,"record_offset":114820,"parent_record_offset":114124,"ancestor_record_offset":113876,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":141,"record_offset":114124,"parent_record_offset":114080,"ancestor_record_offset":113876,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":141,"record_offset":118584,"parent_record_offset":118560,"ancestor_record_offset":113876,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":114172,"parent_record_offset":114148,"ancestor_record_offset":113876,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":141,"record_offset":114420,"parent_record_offset":114396,"ancestor_record_offset":113876,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":141,"record_offset":114284,"parent_record_offset":114256,"ancestor_record_offset":113876,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":114620,"parent_record_offset":114220,"ancestor_record_offset":113876,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":141,"record_offset":114148,"parent_record_offset":114124,"ancestor_record_offset":113876,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":141,"record_offset":114312,"parent_record_offset":114284,"ancestor_record_offset":113876,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":141,"record_offset":114344,"parent_record_offset":114312,"ancestor_record_offset":113876,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":141,"record_offset":114220,"parent_record_offset":114196,"ancestor_record_offset":113876,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":141,"record_offset":114196,"parent_record_offset":114172,"ancestor_record_offset":113876,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":141,"record_offset":117544,"parent_record_offset":117520,"ancestor_record_offset":113876,"inlinee":12852,"source_line":268,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":119856,"parent_record_offset":119816,"ancestor_record_offset":113876,"inlinee":12832,"source_line":29,"archive":"_pdb/B/modules/00141.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":141,"record_offset":120588,"parent_record_offset":119816,"ancestor_record_offset":113876,"inlinee":12834,"source_line":40,"archive":"_pdb/B/modules/00141.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_plugin.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::decorator_plugin::DecoratorPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>", "constructor": "bevy_app::plugin::sealed::impl$12::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::decorator_plugin::DecoratorPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::decorator_plugin::DecoratorPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_plugin.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_plugin.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: tiny_glade::decorator_plugin::decorators_on_glade_unload
// 信息 ID: d11ee4f6b5b64e9a51072e86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 28c90495bd362606daff0715: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)> > @ 0x309cc7 -> 0x347090 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)> > > @ 0x440997 -> 0x347090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::decorator_plugin::impl$0::build
// 信息 ID: a6b03fb909d64bea8a410c6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
