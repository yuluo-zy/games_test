// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a50a6f7c48ff257482f6f6171f4dc95a23ea990c99eb3323654a8a3ffcce08ce
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 104, 183, 184, 187, 409, 1418, 1452, 3012
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":187,"record_offset":81076,"rva":4810752,"va":5373519872}
// 编译实例 RVAs: 0x496800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$itertools..tuple_impl..TupleWindows$LT$I$C$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t168053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e875900200","size":5,"soft_fail":false,"target_va":5373699472,"va":5373531414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4160465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e82f84c0ff","size":5,"soft_fail":false,"target_va":5369374080,"va":5373534540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cce50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3984530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e86e33c3ff","size":5,"soft_fail":false,"target_va":5369548368,"va":5373532893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=5
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3462336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8402bcbff","size":5,"soft_fail":false,"target_va":5370067488,"va":5373529819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140153640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3426164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e88cb8cbff","size":5,"soft_fail":false,"target_va":5370099264,"va":5373525423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140153900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3430479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8b1a7cbff","size":5,"soft_fail":false,"target_va":5370099968,"va":5373530442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140153bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3422643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e84dc6cbff","size":5,"soft_fail":false,"target_va":5370100672,"va":5373523310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3375197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8a37fccff","size":5,"soft_fail":false,"target_va":5370157760,"va":5373532952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140252c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2388141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8538fdbff","size":5,"soft_fail":false,"target_va":5371145360,"va":5373533496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140257ba0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2363511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e889efdbff","size":5,"soft_fail":false,"target_va":5371165600,"va":5373529106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b9bb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1965155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e89d03e2ff","size":5,"soft_fail":false,"target_va":5371567024,"va":5373532174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1964323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8dd06e2ff","size":5,"soft_fail":false,"target_va":5371568608,"va":5373532926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba500
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1955401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8b729e2ff","size":5,"soft_fail":false,"target_va":5371569408,"va":5373524804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1952168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e85836e2ff","size":5,"soft_fail":false,"target_va":5371570656,"va":5373522819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402babe0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1959247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8b11ae2ff","size":5,"soft_fail":false,"target_va":5371571168,"va":5373530410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402baf50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1952254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e80236e2ff","size":5,"soft_fail":false,"target_va":5371572048,"va":5373524297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bb570
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1954137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8a72ee2ff","size":5,"soft_fail":false,"target_va":5371573616,"va":5373527748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bc3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1950740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8ec3be2ff","size":5,"soft_fail":false,"target_va":5371577312,"va":5373528047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bc5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1952761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e80734e2ff","size":5,"soft_fail":false,"target_va":5371577792,"va":5373530548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1944507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e84554e2ff","size":5,"soft_fail":false,"target_va":5371580896,"va":5373525398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1950096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8703ee2ff","size":5,"soft_fail":false,"target_va":5371582912,"va":5373533003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1784448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e880c5e4ff","size":5,"soft_fail":false,"target_va":5371746832,"va":5373531275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-56688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e89022ffff","size":5,"soft_fail":false,"target_va":5373471568,"va":5373528251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1158606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8cead1100","size":5,"soft_fail":false,"target_va":5374680032,"va":5373521421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ca870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1260931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8833d1300","size":5,"soft_fail":false,"target_va":5374781552,"va":5373520616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=10
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1283439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e86f951300","size":5,"soft_fail":false,"target_va":5374813424,"va":5373529980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d7690
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1300672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8c0d81300","size":5,"soft_fail":false,"target_va":5374834320,"va":5373533643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1334284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e80c5c1400","size":5,"soft_fail":false,"target_va":5374864432,"va":5373530143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1471686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8c6741600","size":5,"soft_fail":false,"target_va":5375007712,"va":5373536021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140606810
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1496093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e81dd41600","size":5,"soft_fail":false,"target_va":5375027216,"va":5373531118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1559354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e83acb1700","size":5,"soft_fail":false,"target_va":5375081488,"va":5373522129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=72
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1556350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e87ebf1700","size":5,"soft_fail":false,"target_va":5375081504,"va":5373525149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3936322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e842103c00","size":5,"soft_fail":false,"target_va":5377460048,"va":5373523721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140959e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4981673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8a9034c00","size":5,"soft_fail":false,"target_va":5378514544,"va":5373532866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8405652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e894428000","size":5,"soft_fail":false,"target_va":5381934560,"va":5373528903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37788630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8d69b4002","size":5,"soft_fail":false,"target_va":5411313216,"va":5373524581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38326455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8b7d04802","size":5,"soft_fail":false,"target_va":5411847152,"va":5373520692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38313818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e85a9f4802","size":5,"soft_fail":false,"target_va":5411847168,"va":5373533345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38005112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e878e94302","size":5,"soft_fail":false,"target_va":5411540992,"va":5373535875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1804932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8848a1b00","size":5,"soft_fail":false,"target_va":5375328336,"va":5373523399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38005747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8f3eb4302","size":5,"soft_fail":false,"target_va":5411540963,"va":5373535211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38006970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8baf04302","size":5,"soft_fail":false,"target_va":5411541584,"va":5373534609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38008177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e871f54302","size":5,"soft_fail":false,"target_va":5411541808,"va":5373533626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38008174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e86ef54302","size":5,"soft_fail":false,"target_va":5411541776,"va":5373533597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37947836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8bc094302","size":5,"soft_fail":false,"target_va":5411482433,"va":5373534592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=32
//     evidence={"assembly":"\tcallq\t38007552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e800f34302","size":5,"soft_fail":false,"target_va":5411542296,"va":5373534739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t38008404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e854f64302","size":5,"soft_fail":false,"target_va":5411543104,"va":5373534695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38008540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8dcf64302","size":5,"soft_fail":false,"target_va":5411543200,"va":5373534655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38008440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e878f64302","size":5,"soft_fail":false,"target_va":5411543120,"va":5373534675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e86db40000","size":5,"soft_fail":false,"target_va":5373566256,"va":5373520062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38023216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e830304402","size":5,"soft_fail":false,"target_va":5411543360,"va":5373520139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickSourceId::with_transient_stair_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3936305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e831103c00","size":5,"soft_fail":false,"target_va":5377460096,"va":5373523786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator::get_or_allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4615841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8a16e4600","size":5,"soft_fail":false,"target_va":5378139616,"va":5373523770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs_walls::StairsPillarProposals::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4621474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8a2844600","size":5,"soft_fail":false,"target_va":5378151024,"va":5373529545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4410940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e83c4e4300","size":5,"soft_fail":false,"target_va":5377944336,"va":5373533391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4411978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e84a524300","size":5,"soft_fail":false,"target_va":5377945152,"va":5373533169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4407411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e873404300","size":5,"soft_fail":false,"target_va":5377936416,"va":5373529000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37958848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8c0344302","size":5,"soft_fail":false,"target_va":5411492368,"va":5373533515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37961262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e82e3e4302","size":5,"soft_fail":false,"target_va":5411494800,"va":5373533533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8301691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e87bac7e00","size":5,"soft_fail":false,"target_va":5381823600,"va":5373521904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8297953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8e19d7e00","size":5,"soft_fail":false,"target_va":5381824016,"va":5373526058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7864963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e883027800","size":5,"soft_fail":false,"target_va":5381385536,"va":5373520568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7870152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8c8167800","size":5,"soft_fail":false,"target_va":5381390640,"va":5373520483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7861226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8eaf37700","size":5,"soft_fail":false,"target_va":5381386480,"va":5373525249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7864656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e850017800","size":5,"soft_fail":false,"target_va":5381384704,"va":5373520043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38003211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e80be24302","size":5,"soft_fail":false,"target_va":5411537104,"va":5373533888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2833229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e84d3b2b00","size":5,"soft_fail":false,"target_va":5376361168,"va":5373527934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_playermade_stairs::preprocess_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3054483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e8939b2e00","size":5,"soft_fail":false,"target_va":5376577056,"va":5373522568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2950012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373519872,"raw_hex":"e87c032d00","size":5,"soft_fail":false,"target_va":5376481120,"va":5373531103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373519872,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5373520712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":187,"record_offset":115756,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":84260,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":115472,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":88620,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":90312,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":187,"record_offset":84580,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":91888,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":95312,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":109092,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":115016,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":112492,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":187,"record_offset":84200,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":187,"record_offset":106676,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":187,"record_offset":115556,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":95540,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":104636,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":104560,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":95408,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":111684,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":187,"record_offset":84552,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":87532,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":97224,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":97000,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":87036,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":88528,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":84164,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":95568,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":187,"record_offset":88564,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":90228,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":90496,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":187,"record_offset":83924,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::instanced_wall::impl$25::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":187,"record_offset":112620,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14292,"source_line":739,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":87100,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":187,"record_offset":108164,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":187,"record_offset":112464,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14290,"source_line":201,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":103320,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107672,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107104,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":108012,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$37::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":113312,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14294,"source_line":1310,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107368,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107616,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107160,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":187,"record_offset":94832,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":187,"record_offset":104732,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":82916,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":81484,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":81940,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":84932,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":85960,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":187,"record_offset":91832,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":14282,"source_line":50,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":103544,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":91860,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::reinterpret_as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":88704,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":9596,"source_line":1132,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":90556,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":187,"record_offset":104668,"parent_record_offset":81076,"ancestor_record_offset":81076,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":88016,"parent_record_offset":87992,"ancestor_record_offset":81076,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":88040,"parent_record_offset":88016,"ancestor_record_offset":81076,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":90172,"parent_record_offset":90144,"ancestor_record_offset":81076,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":115824,"parent_record_offset":115788,"ancestor_record_offset":81076,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":87992,"parent_record_offset":87812,"ancestor_record_offset":81076,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":90144,"parent_record_offset":90044,"ancestor_record_offset":81076,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":187,"record_offset":115788,"parent_record_offset":115756,"ancestor_record_offset":81076,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":84320,"parent_record_offset":84296,"ancestor_record_offset":81076,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":87744,"parent_record_offset":87712,"ancestor_record_offset":81076,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":90076,"parent_record_offset":90044,"ancestor_record_offset":81076,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":90044,"parent_record_offset":90016,"ancestor_record_offset":81076,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":84368,"parent_record_offset":84344,"ancestor_record_offset":81076,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":84344,"parent_record_offset":84320,"ancestor_record_offset":81076,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":87812,"parent_record_offset":87776,"ancestor_record_offset":81076,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":87776,"parent_record_offset":87744,"ancestor_record_offset":81076,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":90016,"parent_record_offset":89992,"ancestor_record_offset":81076,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":87852,"parent_record_offset":87812,"ancestor_record_offset":81076,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":90352,"parent_record_offset":90312,"ancestor_record_offset":81076,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":84296,"parent_record_offset":84260,"ancestor_record_offset":81076,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":109140,"parent_record_offset":109116,"ancestor_record_offset":81076,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":88120,"parent_record_offset":88096,"ancestor_record_offset":81076,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":92176,"parent_record_offset":91920,"ancestor_record_offset":81076,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":84408,"parent_record_offset":84260,"ancestor_record_offset":81076,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":84436,"parent_record_offset":84260,"ancestor_record_offset":81076,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":87680,"parent_record_offset":87640,"ancestor_record_offset":81076,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":87712,"parent_record_offset":87680,"ancestor_record_offset":81076,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":187,"record_offset":84796,"parent_record_offset":84764,"ancestor_record_offset":81076,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":90628,"parent_record_offset":90556,"ancestor_record_offset":81076,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":87568,"parent_record_offset":87532,"ancestor_record_offset":81076,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":88336,"parent_record_offset":88292,"ancestor_record_offset":81076,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":109116,"parent_record_offset":109092,"ancestor_record_offset":81076,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":112672,"parent_record_offset":112648,"ancestor_record_offset":81076,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":112720,"parent_record_offset":112696,"ancestor_record_offset":81076,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":187,"record_offset":88448,"parent_record_offset":88336,"ancestor_record_offset":81076,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":187,"record_offset":88096,"parent_record_offset":87640,"ancestor_record_offset":81076,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":187,"record_offset":91920,"parent_record_offset":91888,"ancestor_record_offset":81076,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":187,"record_offset":87604,"parent_record_offset":87568,"ancestor_record_offset":81076,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":187,"record_offset":87640,"parent_record_offset":87604,"ancestor_record_offset":81076,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":187,"record_offset":84228,"parent_record_offset":84200,"ancestor_record_offset":81076,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":187,"record_offset":108196,"parent_record_offset":108164,"ancestor_record_offset":81076,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":187,"record_offset":108220,"parent_record_offset":108196,"ancestor_record_offset":81076,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":187,"record_offset":108244,"parent_record_offset":108220,"ancestor_record_offset":81076,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":187,"record_offset":87876,"parent_record_offset":87852,"ancestor_record_offset":81076,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":187,"record_offset":87900,"parent_record_offset":87876,"ancestor_record_offset":81076,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":187,"record_offset":115040,"parent_record_offset":115016,"ancestor_record_offset":81076,"inlinee":4555,"source_line":56,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":187,"record_offset":106792,"parent_record_offset":106700,"ancestor_record_offset":81076,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83256,"parent_record_offset":83228,"ancestor_record_offset":81076,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83308,"parent_record_offset":83284,"ancestor_record_offset":81076,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83284,"parent_record_offset":83256,"ancestor_record_offset":81076,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83204,"parent_record_offset":83172,"ancestor_record_offset":81076,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83228,"parent_record_offset":83204,"ancestor_record_offset":81076,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83400,"parent_record_offset":83228,"ancestor_record_offset":81076,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":83512,"parent_record_offset":83488,"ancestor_record_offset":81076,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":97096,"parent_record_offset":97072,"ancestor_record_offset":81076,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":92288,"parent_record_offset":92252,"ancestor_record_offset":81076,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":111732,"parent_record_offset":111708,"ancestor_record_offset":81076,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":83992,"parent_record_offset":83952,"ancestor_record_offset":81076,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":97072,"parent_record_offset":97048,"ancestor_record_offset":81076,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":187,"record_offset":85224,"parent_record_offset":85200,"ancestor_record_offset":81076,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":187,"record_offset":85308,"parent_record_offset":85284,"ancestor_record_offset":81076,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":104696,"parent_record_offset":104668,"ancestor_record_offset":81076,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":187,"record_offset":85248,"parent_record_offset":85224,"ancestor_record_offset":81076,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":187,"record_offset":85784,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":187,"record_offset":95040,"parent_record_offset":95016,"ancestor_record_offset":81076,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":187,"record_offset":95088,"parent_record_offset":95064,"ancestor_record_offset":81076,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":97248,"parent_record_offset":97224,"ancestor_record_offset":81076,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":88168,"parent_record_offset":88144,"ancestor_record_offset":81076,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":89320,"parent_record_offset":89004,"ancestor_record_offset":81076,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":88248,"parent_record_offset":88192,"ancestor_record_offset":81076,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":104212,"parent_record_offset":104184,"ancestor_record_offset":81076,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":97024,"parent_record_offset":97000,"ancestor_record_offset":81076,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":105260,"parent_record_offset":105220,"ancestor_record_offset":81076,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":97048,"parent_record_offset":97024,"ancestor_record_offset":81076,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":104592,"parent_record_offset":104560,"ancestor_record_offset":81076,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":95436,"parent_record_offset":95408,"ancestor_record_offset":81076,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":105380,"parent_record_offset":105260,"ancestor_record_offset":81076,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":92320,"parent_record_offset":92252,"ancestor_record_offset":81076,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":92252,"parent_record_offset":92220,"ancestor_record_offset":81076,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":92220,"parent_record_offset":92176,"ancestor_record_offset":81076,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":187,"record_offset":92616,"parent_record_offset":92592,"ancestor_record_offset":81076,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":88840,"parent_record_offset":88704,"ancestor_record_offset":81076,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":88144,"parent_record_offset":88120,"ancestor_record_offset":81076,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":88292,"parent_record_offset":88248,"ancestor_record_offset":81076,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":95112,"parent_record_offset":95088,"ancestor_record_offset":81076,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":187,"record_offset":106816,"parent_record_offset":106792,"ancestor_record_offset":81076,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":187,"record_offset":83332,"parent_record_offset":83308,"ancestor_record_offset":81076,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":87924,"parent_record_offset":87900,"ancestor_record_offset":81076,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":87456,"parent_record_offset":87428,"ancestor_record_offset":81076,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":87948,"parent_record_offset":87924,"ancestor_record_offset":81076,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":92344,"parent_record_offset":92320,"ancestor_record_offset":81076,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":90108,"parent_record_offset":90076,"ancestor_record_offset":81076,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":187,"record_offset":81640,"parent_record_offset":81608,"ancestor_record_offset":81076,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":187,"record_offset":85456,"parent_record_offset":85428,"ancestor_record_offset":81076,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":187,"record_offset":85428,"parent_record_offset":85400,"ancestor_record_offset":81076,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":91744,"parent_record_offset":91452,"ancestor_record_offset":81076,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":91716,"parent_record_offset":91452,"ancestor_record_offset":81076,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":107392,"parent_record_offset":107368,"ancestor_record_offset":81076,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":107184,"parent_record_offset":107160,"ancestor_record_offset":81076,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":91688,"parent_record_offset":91452,"ancestor_record_offset":81076,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":187,"record_offset":83068,"parent_record_offset":83024,"ancestor_record_offset":81076,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":187,"record_offset":106736,"parent_record_offset":106700,"ancestor_record_offset":81076,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":187,"record_offset":106700,"parent_record_offset":106676,"ancestor_record_offset":81076,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":187,"record_offset":102692,"parent_record_offset":102656,"ancestor_record_offset":81076,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":187,"record_offset":95504,"parent_record_offset":95436,"ancestor_record_offset":81076,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":87364,"parent_record_offset":87100,"ancestor_record_offset":81076,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":84764,"parent_record_offset":84580,"ancestor_record_offset":81076,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":95064,"parent_record_offset":95040,"ancestor_record_offset":81076,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":87176,"parent_record_offset":87100,"ancestor_record_offset":81076,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":87400,"parent_record_offset":87100,"ancestor_record_offset":81076,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":81888,"parent_record_offset":81860,"ancestor_record_offset":81076,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":83880,"parent_record_offset":83024,"ancestor_record_offset":81076,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":106840,"parent_record_offset":106816,"ancestor_record_offset":81076,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":89416,"parent_record_offset":89392,"ancestor_record_offset":81076,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=114
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":83464,"parent_record_offset":83172,"ancestor_record_offset":81076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":84512,"parent_record_offset":84488,"ancestor_record_offset":81076,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":85576,"parent_record_offset":85552,"ancestor_record_offset":81076,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":89392,"parent_record_offset":89368,"ancestor_record_offset":81076,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":84724,"parent_record_offset":84700,"ancestor_record_offset":81076,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":82352,"parent_record_offset":82328,"ancestor_record_offset":81076,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":112560,"parent_record_offset":112492,"ancestor_record_offset":81076,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":82328,"parent_record_offset":82304,"ancestor_record_offset":81076,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":89368,"parent_record_offset":89344,"ancestor_record_offset":81076,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":82536,"parent_record_offset":82508,"ancestor_record_offset":81076,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":84828,"parent_record_offset":84796,"ancestor_record_offset":81076,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":115932,"parent_record_offset":115904,"ancestor_record_offset":81076,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":83356,"parent_record_offset":83332,"ancestor_record_offset":81076,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":187,"record_offset":88588,"parent_record_offset":88564,"ancestor_record_offset":81076,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":187,"record_offset":84076,"parent_record_offset":84048,"ancestor_record_offset":81076,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":187,"record_offset":84104,"parent_record_offset":84076,"ancestor_record_offset":81076,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":84464,"parent_record_offset":84260,"ancestor_record_offset":81076,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":87336,"parent_record_offset":87100,"ancestor_record_offset":81076,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":103800,"parent_record_offset":103776,"ancestor_record_offset":81076,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":103776,"parent_record_offset":103704,"ancestor_record_offset":81076,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":90656,"parent_record_offset":90628,"ancestor_record_offset":81076,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":102656,"parent_record_offset":102628,"ancestor_record_offset":81076,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":90684,"parent_record_offset":90656,"ancestor_record_offset":81076,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":102720,"parent_record_offset":102656,"ancestor_record_offset":81076,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":187,"record_offset":84488,"parent_record_offset":84464,"ancestor_record_offset":81076,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":82280,"parent_record_offset":82256,"ancestor_record_offset":81076,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":84700,"parent_record_offset":84676,"ancestor_record_offset":81076,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":82304,"parent_record_offset":82280,"ancestor_record_offset":81076,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":82608,"parent_record_offset":82580,"ancestor_record_offset":81076,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":88192,"parent_record_offset":88168,"ancestor_record_offset":81076,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":84612,"parent_record_offset":84580,"ancestor_record_offset":81076,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":89344,"parent_record_offset":89320,"ancestor_record_offset":81076,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":82256,"parent_record_offset":82140,"ancestor_record_offset":81076,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":84676,"parent_record_offset":84612,"ancestor_record_offset":81076,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":187,"record_offset":81608,"parent_record_offset":81580,"ancestor_record_offset":81076,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":187,"record_offset":81968,"parent_record_offset":81940,"ancestor_record_offset":81076,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":187,"record_offset":82108,"parent_record_offset":82076,"ancestor_record_offset":81076,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":187,"record_offset":82076,"parent_record_offset":82040,"ancestor_record_offset":81076,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":82016,"parent_record_offset":81992,"ancestor_record_offset":81076,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":82040,"parent_record_offset":82016,"ancestor_record_offset":81076,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":81992,"parent_record_offset":81968,"ancestor_record_offset":81076,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":82580,"parent_record_offset":82040,"ancestor_record_offset":81076,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81580,"parent_record_offset":81548,"ancestor_record_offset":81076,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81548,"parent_record_offset":81520,"ancestor_record_offset":81076,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81740,"parent_record_offset":81708,"ancestor_record_offset":81076,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81836,"parent_record_offset":81740,"ancestor_record_offset":81076,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81708,"parent_record_offset":81684,"ancestor_record_offset":81076,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":84020,"parent_record_offset":83992,"ancestor_record_offset":81076,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81520,"parent_record_offset":81484,"ancestor_record_offset":81076,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":81860,"parent_record_offset":81836,"ancestor_record_offset":81076,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":82140,"parent_record_offset":82108,"ancestor_record_offset":81076,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":82424,"parent_record_offset":82140,"ancestor_record_offset":81076,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":82396,"parent_record_offset":82140,"ancestor_record_offset":81076,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":82200,"parent_record_offset":82140,"ancestor_record_offset":81076,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":100856,"parent_record_offset":100792,"ancestor_record_offset":81076,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":87248,"parent_record_offset":87212,"ancestor_record_offset":81076,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":100880,"parent_record_offset":100856,"ancestor_record_offset":81076,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":87272,"parent_record_offset":87248,"ancestor_record_offset":81076,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":187,"record_offset":83952,"parent_record_offset":83924,"ancestor_record_offset":81076,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":87428,"parent_record_offset":87400,"ancestor_record_offset":81076,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":100764,"parent_record_offset":100736,"ancestor_record_offset":81076,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":187,"record_offset":85740,"parent_record_offset":85716,"ancestor_record_offset":81076,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":103424,"parent_record_offset":103348,"ancestor_record_offset":81076,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107748,"parent_record_offset":107724,"ancestor_record_offset":81076,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107724,"parent_record_offset":107696,"ancestor_record_offset":81076,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":107696,"parent_record_offset":107672,"ancestor_record_offset":81076,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":113336,"parent_record_offset":113312,"ancestor_record_offset":81076,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":103348,"parent_record_offset":103320,"ancestor_record_offset":81076,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":85368,"parent_record_offset":85340,"ancestor_record_offset":81076,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":85400,"parent_record_offset":85368,"ancestor_record_offset":81076,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":85340,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":85200,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":85812,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":85284,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":187,"record_offset":85172,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":187,"record_offset":85692,"parent_record_offset":85668,"ancestor_record_offset":81076,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":86028,"parent_record_offset":85992,"ancestor_record_offset":81076,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":86052,"parent_record_offset":86028,"ancestor_record_offset":81076,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":84980,"parent_record_offset":84956,"ancestor_record_offset":81076,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":84956,"parent_record_offset":84932,"ancestor_record_offset":81076,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":85668,"parent_record_offset":85500,"ancestor_record_offset":81076,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":85012,"parent_record_offset":84980,"ancestor_record_offset":81076,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85608,"parent_record_offset":85524,"ancestor_record_offset":81076,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85552,"parent_record_offset":85524,"ancestor_record_offset":81076,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85904,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85524,"parent_record_offset":85500,"ancestor_record_offset":81076,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85064,"parent_record_offset":85040,"ancestor_record_offset":81076,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":86112,"parent_record_offset":86052,"ancestor_record_offset":81076,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85040,"parent_record_offset":84980,"ancestor_record_offset":81076,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85088,"parent_record_offset":85064,"ancestor_record_offset":81076,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85144,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":85500,"parent_record_offset":85088,"ancestor_record_offset":81076,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":187,"record_offset":95016,"parent_record_offset":94832,"ancestor_record_offset":81076,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":187,"record_offset":87212,"parent_record_offset":87100,"ancestor_record_offset":81076,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":187,"record_offset":100792,"parent_record_offset":100764,"ancestor_record_offset":81076,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":187,"record_offset":87304,"parent_record_offset":87212,"ancestor_record_offset":81076,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":187,"record_offset":100828,"parent_record_offset":100792,"ancestor_record_offset":81076,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":82952,"parent_record_offset":82916,"ancestor_record_offset":81076,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":95880,"parent_record_offset":95840,"ancestor_record_offset":81076,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":96448,"parent_record_offset":96176,"ancestor_record_offset":81076,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":187,"record_offset":83096,"parent_record_offset":83068,"ancestor_record_offset":81076,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":83172,"parent_record_offset":83024,"ancestor_record_offset":81076,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":83436,"parent_record_offset":83172,"ancestor_record_offset":81076,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":85992,"parent_record_offset":85960,"ancestor_record_offset":81076,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":187,"record_offset":103472,"parent_record_offset":103424,"ancestor_record_offset":81076,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":187,"record_offset":83124,"parent_record_offset":83096,"ancestor_record_offset":81076,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":187,"record_offset":83024,"parent_record_offset":82988,"ancestor_record_offset":81076,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":187,"record_offset":82988,"parent_record_offset":82952,"ancestor_record_offset":81076,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":90600,"parent_record_offset":90556,"ancestor_record_offset":81076,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":90724,"parent_record_offset":90556,"ancestor_record_offset":81076,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":187,"record_offset":89680,"parent_record_offset":89636,"ancestor_record_offset":81076,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$6::reinterpret_as_curve2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":89636,"parent_record_offset":89604,"ancestor_record_offset":81076,"inlinee":9584,"source_line":1139,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":409,"record_offset":272,"rva":7867936,"va":5376577056}
// 编译实例 RVAs: 0x780e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5006550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e82a9bb3ff","size":5,"soft_fail":false,"target_va":5371570656,"va":5376577201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1769115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e86501e5ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5376582534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1495948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8742ce9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376577431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=34
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1497035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e83528e9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376578534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10972773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8656ea700","size":5,"soft_fail":false,"target_va":5387551024,"va":5376578246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10979928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8588aa700","size":5,"soft_fail":false,"target_va":5387557152,"va":5376577219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fac00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10983479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e83798a700","size":5,"soft_fail":false,"target_va":5387561984,"va":5376578500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141237ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11234545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8f16cab00","size":5,"soft_fail":false,"target_va":5387812592,"va":5376578042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35268268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8ac261a02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376578895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35270316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8ac2e1a02","size":5,"soft_fail":false,"target_va":5411848096,"va":5376577775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34958571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8eb6c1502","size":5,"soft_fail":false,"target_va":5411540992,"va":5376582416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-2223157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8cb13deff","size":5,"soft_fail":false,"target_va":5374354992,"va":5376578144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34760687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8ef671202","size":5,"soft_fail":false,"target_va":5411339744,"va":5376579052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t34960236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e86c731502","size":5,"soft_fail":false,"target_va":5411542064,"va":5376581823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34959857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8f1711502","size":5,"soft_fail":false,"target_va":5411542296,"va":5376582434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34959927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e837721502","size":5,"soft_fail":false,"target_va":5411541904,"va":5376581972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376577056,"raw_hex":"e8ce1e0000","size":5,"soft_fail":false,"target_va":5376585552,"va":5376577661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":844,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":409,"record_offset":996,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":4420,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":816,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":4156,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":1232,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":12492,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":3060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":1136,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":13008,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":788,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":409,"record_offset":1164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":2560,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":409,"record_offset":1728,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":409,"record_offset":6072,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":409,"record_offset":696,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":409,"record_offset":732,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":409,"record_offset":1108,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":760,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":2504,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_resample
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":6404,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9587,"source_line":316,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_resample_approx_pt_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":2532,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9591,"source_line":385,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":1420,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4978,"source_line":1152,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":409,"record_offset":4752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":876,"parent_record_offset":844,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":1652,"parent_record_offset":1560,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":272,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":272,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":272,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":272,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":272,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":409,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":272,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":3140,"parent_record_offset":3096,"ancestor_record_offset":272,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":1268,"parent_record_offset":1232,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":272,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":272,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":3484,"parent_record_offset":3140,"ancestor_record_offset":272,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":1460,"parent_record_offset":1420,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":3188,"parent_record_offset":3140,"ancestor_record_offset":272,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":409,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":409,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":409,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":272,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":1960,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":4904,"parent_record_offset":4752,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":1876,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":1904,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":409,"record_offset":4932,"parent_record_offset":4752,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":636,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":409,"record_offset":1620,"parent_record_offset":1588,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":409,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":272,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":409,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":409,"record_offset":4792,"parent_record_offset":4752,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":409,"record_offset":1196,"parent_record_offset":1164,"ancestor_record_offset":272,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":1488,"parent_record_offset":1420,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":409,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":952,"parent_record_offset":924,"ancestor_record_offset":272,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":1388,"parent_record_offset":1232,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":409,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":12520,"parent_record_offset":12492,"ancestor_record_offset":272,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":12548,"parent_record_offset":12520,"ancestor_record_offset":272,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":409,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":409,"record_offset":1932,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":409,"record_offset":1764,"parent_record_offset":1728,"ancestor_record_offset":272,"inlinee":24728,"source_line":201,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":409,"record_offset":6372,"parent_record_offset":6072,"ancestor_record_offset":272,"inlinee":24729,"source_line":242,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_resample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":272,"inlinee":9587,"source_line":316,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_resample_remap
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":1828,"parent_record_offset":1796,"ancestor_record_offset":272,"inlinee":9539,"source_line":359,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":409,"record_offset":14448,"rva":7876432,"va":5376585552}
// 编译实例 RVAs: 0x782f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141247420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11289713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e87144ac00","size":5,"soft_fail":false,"target_va":5387875360,"va":5376585642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141247900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11290930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e83249ac00","size":5,"soft_fail":false,"target_va":5387876608,"va":5376585673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34956152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e878631502","size":5,"soft_fail":false,"target_va":5411542064,"va":5376585907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2718068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e874792900","size":5,"soft_fail":false,"target_va":5379303920,"va":5376585847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2726336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e8c0992900","size":5,"soft_fail":false,"target_va":5379311920,"va":5376585579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2725255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e887952900","size":5,"soft_fail":false,"target_va":5379310864,"va":5376585604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3206808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376585552,"raw_hex":"e898ee3000","size":5,"soft_fail":false,"target_va":5379792624,"va":5376585811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15028,"parent_record_offset":14448,"ancestor_record_offset":14448,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":409,"record_offset":14628,"parent_record_offset":14448,"ancestor_record_offset":14448,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edge_weight
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":14680,"parent_record_offset":14448,"ancestor_record_offset":14448,"inlinee":4650,"source_line":525,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edges
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":14736,"parent_record_offset":14448,"ancestor_record_offset":14448,"inlinee":4971,"source_line":494,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14448,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":14916,"parent_record_offset":14892,"ancestor_record_offset":14448,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":14892,"parent_record_offset":14868,"ancestor_record_offset":14448,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":14448,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":14820,"parent_record_offset":14788,"ancestor_record_offset":14448,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":14788,"parent_record_offset":14760,"ancestor_record_offset":14448,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":409,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":14448,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15544,"parent_record_offset":15056,"ancestor_record_offset":14448,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":14448,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15056,"parent_record_offset":15028,"ancestor_record_offset":14448,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":14448,"inlinee":10601,"source_line":4049,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":14448,"inlinee":10603,"source_line":4098,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":409,"record_offset":15444,"parent_record_offset":15092,"ancestor_record_offset":14448,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":409,"record_offset":14984,"parent_record_offset":14960,"ancestor_record_offset":14448,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":409,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":14448,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":409,"record_offset":15304,"parent_record_offset":15268,"ancestor_record_offset":14448,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":409,"record_offset":14960,"parent_record_offset":14788,"ancestor_record_offset":14448,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":409,"record_offset":15356,"parent_record_offset":15332,"ancestor_record_offset":14448,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":409,"record_offset":15268,"parent_record_offset":15240,"ancestor_record_offset":14448,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":409,"record_offset":15332,"parent_record_offset":15268,"ancestor_record_offset":14448,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::edge_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14448,"inlinee":4651,"source_line":234,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":14760,"parent_record_offset":14736,"ancestor_record_offset":14448,"inlinee":4972,"source_line":454,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$11::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":15092,"parent_record_offset":15056,"ancestor_record_offset":14448,"inlinee":10647,"source_line":809,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$11::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":15472,"parent_record_offset":15444,"ancestor_record_offset":14448,"inlinee":10605,"source_line":810,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":409,"record_offset":15120,"parent_record_offset":15092,"ancestor_record_offset":14448,"inlinee":10600,"source_line":719,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":409,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14448,"inlinee":24731,"source_line":102,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":409,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":14448,"inlinee":24732,"source_line":125,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x609B
// 首条位置: {"module":409,"inlinee":24731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x609C
// 首条位置: {"module":409,"inlinee":24732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114BA
// 首条位置: {"module":1418,"inlinee":70842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1418,"record_offset":1708,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1418,"record_offset":1852,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1418,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1668,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1418,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1668,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1418,"record_offset":1992,"parent_record_offset":1852,"ancestor_record_offset":1668,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1418,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":2260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1418,"record_offset":1820,"parent_record_offset":1708,"ancestor_record_offset":1668,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1418,"record_offset":1764,"parent_record_offset":1708,"ancestor_record_offset":1668,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1418,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":1668,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1418,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1668,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1418,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":1668,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01418.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6098
// 首条位置: {"module":409,"inlinee":24728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":1996,"parent_record_offset":1764,"ancestor_record_offset":1764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_resample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":1764,"inlinee":9587,"source_line":316,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1764,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":409,"record_offset":2184,"parent_record_offset":2092,"ancestor_record_offset":1764,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":1764,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1764,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":409,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1764,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":1960,"parent_record_offset":1828,"ancestor_record_offset":1764,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":1876,"parent_record_offset":1828,"ancestor_record_offset":1764,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":409,"record_offset":1904,"parent_record_offset":1828,"ancestor_record_offset":1764,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":409,"record_offset":2152,"parent_record_offset":2120,"ancestor_record_offset":1764,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":409,"record_offset":2020,"parent_record_offset":1996,"ancestor_record_offset":1764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":409,"record_offset":1932,"parent_record_offset":1828,"ancestor_record_offset":1764,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::try_resample_remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":409,"record_offset":1828,"parent_record_offset":1796,"ancestor_record_offset":1764,"inlinee":9539,"source_line":359,"archive":"_pdb/B/modules/00409.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6099
// 首条位置: {"module":409,"inlinee":24729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x37D8
// 首条位置: {"module":187,"inlinee":14296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":141932,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":146292,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":147964,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":187,"record_offset":142252,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":149540,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":152944,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":166716,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":170084,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":187,"record_offset":141872,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":187,"record_offset":164396,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":153172,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":162248,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":162172,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":153040,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":169276,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":187,"record_offset":142224,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":145204,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":154836,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":154612,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":144708,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":146200,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":141836,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":153200,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":187,"record_offset":146236,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":147880,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":148148,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":187,"record_offset":141596,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::instanced_wall::impl$25::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":187,"record_offset":170212,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":14292,"source_line":739,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":144772,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":187,"record_offset":165884,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":187,"record_offset":170056,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":14290,"source_line":201,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":160932,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165392,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":164824,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165732,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165088,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165336,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":164880,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":187,"record_offset":152464,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":187,"record_offset":162440,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":140592,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":139160,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":139616,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":142604,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":143632,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":187,"record_offset":149484,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":14282,"source_line":50,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":161156,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":149512,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::reinterpret_as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":146376,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":9596,"source_line":1132,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":148208,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":187,"record_offset":162376,"parent_record_offset":137988,"ancestor_record_offset":137988,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":145688,"parent_record_offset":145664,"ancestor_record_offset":137988,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":145712,"parent_record_offset":145688,"ancestor_record_offset":137988,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":147828,"parent_record_offset":147804,"ancestor_record_offset":137988,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":145664,"parent_record_offset":145484,"ancestor_record_offset":137988,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":187,"record_offset":147804,"parent_record_offset":147712,"ancestor_record_offset":137988,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":141992,"parent_record_offset":141968,"ancestor_record_offset":137988,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":145416,"parent_record_offset":145384,"ancestor_record_offset":137988,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":147740,"parent_record_offset":147712,"ancestor_record_offset":137988,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":147712,"parent_record_offset":147688,"ancestor_record_offset":137988,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":142040,"parent_record_offset":142016,"ancestor_record_offset":137988,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":142016,"parent_record_offset":141992,"ancestor_record_offset":137988,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":145484,"parent_record_offset":145448,"ancestor_record_offset":137988,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":145448,"parent_record_offset":145416,"ancestor_record_offset":137988,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":147688,"parent_record_offset":147664,"ancestor_record_offset":137988,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":187,"record_offset":145524,"parent_record_offset":145484,"ancestor_record_offset":137988,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":148004,"parent_record_offset":147964,"ancestor_record_offset":137988,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":141968,"parent_record_offset":141932,"ancestor_record_offset":137988,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":166764,"parent_record_offset":166740,"ancestor_record_offset":137988,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":145792,"parent_record_offset":145768,"ancestor_record_offset":137988,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":149828,"parent_record_offset":149572,"ancestor_record_offset":137988,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":142080,"parent_record_offset":141932,"ancestor_record_offset":137988,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":142108,"parent_record_offset":141932,"ancestor_record_offset":137988,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":145352,"parent_record_offset":145312,"ancestor_record_offset":137988,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":145384,"parent_record_offset":145352,"ancestor_record_offset":137988,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":187,"record_offset":142468,"parent_record_offset":142436,"ancestor_record_offset":137988,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":148280,"parent_record_offset":148208,"ancestor_record_offset":137988,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":145240,"parent_record_offset":145204,"ancestor_record_offset":137988,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":146008,"parent_record_offset":145964,"ancestor_record_offset":137988,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":187,"record_offset":166740,"parent_record_offset":166716,"ancestor_record_offset":137988,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":170264,"parent_record_offset":170240,"ancestor_record_offset":137988,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":187,"record_offset":170312,"parent_record_offset":170288,"ancestor_record_offset":137988,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":187,"record_offset":146120,"parent_record_offset":146008,"ancestor_record_offset":137988,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":187,"record_offset":145768,"parent_record_offset":145312,"ancestor_record_offset":137988,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":187,"record_offset":149572,"parent_record_offset":149540,"ancestor_record_offset":137988,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":187,"record_offset":145276,"parent_record_offset":145240,"ancestor_record_offset":137988,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":187,"record_offset":145312,"parent_record_offset":145276,"ancestor_record_offset":137988,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":187,"record_offset":141900,"parent_record_offset":141872,"ancestor_record_offset":137988,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":187,"record_offset":165916,"parent_record_offset":165884,"ancestor_record_offset":137988,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":187,"record_offset":165940,"parent_record_offset":165916,"ancestor_record_offset":137988,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":187,"record_offset":165964,"parent_record_offset":165940,"ancestor_record_offset":137988,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":187,"record_offset":145548,"parent_record_offset":145524,"ancestor_record_offset":137988,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":187,"record_offset":145572,"parent_record_offset":145548,"ancestor_record_offset":137988,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":187,"record_offset":164512,"parent_record_offset":164420,"ancestor_record_offset":137988,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":140928,"parent_record_offset":140900,"ancestor_record_offset":137988,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":140980,"parent_record_offset":140956,"ancestor_record_offset":137988,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":140956,"parent_record_offset":140928,"ancestor_record_offset":137988,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":140876,"parent_record_offset":140848,"ancestor_record_offset":137988,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":140900,"parent_record_offset":140876,"ancestor_record_offset":137988,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":141072,"parent_record_offset":140900,"ancestor_record_offset":137988,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":187,"record_offset":141184,"parent_record_offset":141160,"ancestor_record_offset":137988,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":154708,"parent_record_offset":154684,"ancestor_record_offset":137988,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":149940,"parent_record_offset":149904,"ancestor_record_offset":137988,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":169324,"parent_record_offset":169300,"ancestor_record_offset":137988,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":141664,"parent_record_offset":141624,"ancestor_record_offset":137988,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":187,"record_offset":154684,"parent_record_offset":154660,"ancestor_record_offset":137988,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":187,"record_offset":142896,"parent_record_offset":142872,"ancestor_record_offset":137988,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":187,"record_offset":142980,"parent_record_offset":142956,"ancestor_record_offset":137988,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":187,"record_offset":162404,"parent_record_offset":162376,"ancestor_record_offset":137988,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":187,"record_offset":142920,"parent_record_offset":142896,"ancestor_record_offset":137988,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":187,"record_offset":143456,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":187,"record_offset":152672,"parent_record_offset":152648,"ancestor_record_offset":137988,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":187,"record_offset":152720,"parent_record_offset":152696,"ancestor_record_offset":137988,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":154860,"parent_record_offset":154836,"ancestor_record_offset":137988,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":145840,"parent_record_offset":145816,"ancestor_record_offset":137988,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":146992,"parent_record_offset":146676,"ancestor_record_offset":137988,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":187,"record_offset":145920,"parent_record_offset":145864,"ancestor_record_offset":137988,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":161824,"parent_record_offset":161796,"ancestor_record_offset":137988,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":154636,"parent_record_offset":154612,"ancestor_record_offset":137988,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":162980,"parent_record_offset":162940,"ancestor_record_offset":137988,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":154660,"parent_record_offset":154636,"ancestor_record_offset":137988,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":187,"record_offset":162204,"parent_record_offset":162172,"ancestor_record_offset":137988,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":153068,"parent_record_offset":153040,"ancestor_record_offset":137988,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":163100,"parent_record_offset":162980,"ancestor_record_offset":137988,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":149972,"parent_record_offset":149904,"ancestor_record_offset":137988,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":149904,"parent_record_offset":149872,"ancestor_record_offset":137988,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":187,"record_offset":149872,"parent_record_offset":149828,"ancestor_record_offset":137988,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":187,"record_offset":150268,"parent_record_offset":150244,"ancestor_record_offset":137988,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":146512,"parent_record_offset":146376,"ancestor_record_offset":137988,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":145816,"parent_record_offset":145792,"ancestor_record_offset":137988,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":145964,"parent_record_offset":145920,"ancestor_record_offset":137988,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":187,"record_offset":152744,"parent_record_offset":152720,"ancestor_record_offset":137988,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":187,"record_offset":164536,"parent_record_offset":164512,"ancestor_record_offset":137988,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":187,"record_offset":141004,"parent_record_offset":140980,"ancestor_record_offset":137988,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":145596,"parent_record_offset":145572,"ancestor_record_offset":137988,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":145128,"parent_record_offset":145100,"ancestor_record_offset":137988,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":145620,"parent_record_offset":145596,"ancestor_record_offset":137988,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":149996,"parent_record_offset":149972,"ancestor_record_offset":137988,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":187,"record_offset":147772,"parent_record_offset":147740,"ancestor_record_offset":137988,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":187,"record_offset":139316,"parent_record_offset":139284,"ancestor_record_offset":137988,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":187,"record_offset":143128,"parent_record_offset":143100,"ancestor_record_offset":137988,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":187,"record_offset":143100,"parent_record_offset":143072,"ancestor_record_offset":137988,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":149396,"parent_record_offset":149104,"ancestor_record_offset":137988,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":149368,"parent_record_offset":149104,"ancestor_record_offset":137988,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":165112,"parent_record_offset":165088,"ancestor_record_offset":137988,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":164904,"parent_record_offset":164880,"ancestor_record_offset":137988,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":187,"record_offset":149340,"parent_record_offset":149104,"ancestor_record_offset":137988,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":187,"record_offset":140744,"parent_record_offset":140700,"ancestor_record_offset":137988,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":187,"record_offset":164456,"parent_record_offset":164420,"ancestor_record_offset":137988,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":187,"record_offset":164420,"parent_record_offset":164396,"ancestor_record_offset":137988,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":187,"record_offset":160304,"parent_record_offset":160268,"ancestor_record_offset":137988,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":187,"record_offset":153136,"parent_record_offset":153068,"ancestor_record_offset":137988,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":145036,"parent_record_offset":144772,"ancestor_record_offset":137988,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":142436,"parent_record_offset":142252,"ancestor_record_offset":137988,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":152696,"parent_record_offset":152672,"ancestor_record_offset":137988,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":144848,"parent_record_offset":144772,"ancestor_record_offset":137988,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":187,"record_offset":145072,"parent_record_offset":144772,"ancestor_record_offset":137988,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":139564,"parent_record_offset":139536,"ancestor_record_offset":137988,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":141552,"parent_record_offset":140700,"ancestor_record_offset":137988,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":164560,"parent_record_offset":164536,"ancestor_record_offset":137988,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":187,"record_offset":147088,"parent_record_offset":147064,"ancestor_record_offset":137988,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=110
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":141136,"parent_record_offset":140848,"ancestor_record_offset":137988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":142184,"parent_record_offset":142160,"ancestor_record_offset":137988,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":143248,"parent_record_offset":143224,"ancestor_record_offset":137988,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":187,"record_offset":147064,"parent_record_offset":147040,"ancestor_record_offset":137988,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":142396,"parent_record_offset":142372,"ancestor_record_offset":137988,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":140028,"parent_record_offset":140004,"ancestor_record_offset":137988,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":170152,"parent_record_offset":170084,"ancestor_record_offset":137988,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":140004,"parent_record_offset":139980,"ancestor_record_offset":137988,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":147040,"parent_record_offset":147016,"ancestor_record_offset":137988,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":187,"record_offset":140212,"parent_record_offset":140184,"ancestor_record_offset":137988,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":142500,"parent_record_offset":142468,"ancestor_record_offset":137988,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":187,"record_offset":141028,"parent_record_offset":141004,"ancestor_record_offset":137988,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":187,"record_offset":146260,"parent_record_offset":146236,"ancestor_record_offset":137988,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":187,"record_offset":141748,"parent_record_offset":141720,"ancestor_record_offset":137988,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":187,"record_offset":141776,"parent_record_offset":141748,"ancestor_record_offset":137988,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":142136,"parent_record_offset":141932,"ancestor_record_offset":137988,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":187,"record_offset":145008,"parent_record_offset":144772,"ancestor_record_offset":137988,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":161412,"parent_record_offset":161388,"ancestor_record_offset":137988,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":161388,"parent_record_offset":161316,"ancestor_record_offset":137988,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":148308,"parent_record_offset":148280,"ancestor_record_offset":137988,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":160268,"parent_record_offset":160240,"ancestor_record_offset":137988,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":148336,"parent_record_offset":148308,"ancestor_record_offset":137988,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":187,"record_offset":160332,"parent_record_offset":160268,"ancestor_record_offset":137988,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":187,"record_offset":142160,"parent_record_offset":142136,"ancestor_record_offset":137988,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":139956,"parent_record_offset":139932,"ancestor_record_offset":137988,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":142372,"parent_record_offset":142348,"ancestor_record_offset":137988,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":139980,"parent_record_offset":139956,"ancestor_record_offset":137988,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":140284,"parent_record_offset":140256,"ancestor_record_offset":137988,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":145864,"parent_record_offset":145840,"ancestor_record_offset":137988,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":142284,"parent_record_offset":142252,"ancestor_record_offset":137988,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":147016,"parent_record_offset":146992,"ancestor_record_offset":137988,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":139932,"parent_record_offset":139816,"ancestor_record_offset":137988,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":187,"record_offset":142348,"parent_record_offset":142284,"ancestor_record_offset":137988,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":187,"record_offset":139284,"parent_record_offset":139256,"ancestor_record_offset":137988,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":187,"record_offset":139644,"parent_record_offset":139616,"ancestor_record_offset":137988,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":187,"record_offset":139784,"parent_record_offset":139752,"ancestor_record_offset":137988,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":187,"record_offset":139752,"parent_record_offset":139716,"ancestor_record_offset":137988,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":139692,"parent_record_offset":139668,"ancestor_record_offset":137988,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":139716,"parent_record_offset":139692,"ancestor_record_offset":137988,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":139668,"parent_record_offset":139644,"ancestor_record_offset":137988,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":187,"record_offset":140256,"parent_record_offset":139716,"ancestor_record_offset":137988,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139256,"parent_record_offset":139224,"ancestor_record_offset":137988,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139224,"parent_record_offset":139196,"ancestor_record_offset":137988,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139416,"parent_record_offset":139384,"ancestor_record_offset":137988,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139512,"parent_record_offset":139416,"ancestor_record_offset":137988,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139384,"parent_record_offset":139360,"ancestor_record_offset":137988,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":141692,"parent_record_offset":141664,"ancestor_record_offset":137988,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139196,"parent_record_offset":139160,"ancestor_record_offset":137988,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":187,"record_offset":139536,"parent_record_offset":139512,"ancestor_record_offset":137988,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":139816,"parent_record_offset":139784,"ancestor_record_offset":137988,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":140100,"parent_record_offset":139816,"ancestor_record_offset":137988,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":140072,"parent_record_offset":139816,"ancestor_record_offset":137988,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":187,"record_offset":139876,"parent_record_offset":139816,"ancestor_record_offset":137988,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":158468,"parent_record_offset":158404,"ancestor_record_offset":137988,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":144920,"parent_record_offset":144884,"ancestor_record_offset":137988,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":158492,"parent_record_offset":158468,"ancestor_record_offset":137988,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":187,"record_offset":144944,"parent_record_offset":144920,"ancestor_record_offset":137988,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":187,"record_offset":141624,"parent_record_offset":141596,"ancestor_record_offset":137988,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":145100,"parent_record_offset":145072,"ancestor_record_offset":137988,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":187,"record_offset":158376,"parent_record_offset":158348,"ancestor_record_offset":137988,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":187,"record_offset":143412,"parent_record_offset":143388,"ancestor_record_offset":137988,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":161036,"parent_record_offset":160960,"ancestor_record_offset":137988,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165468,"parent_record_offset":165444,"ancestor_record_offset":137988,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165444,"parent_record_offset":165416,"ancestor_record_offset":137988,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":165416,"parent_record_offset":165392,"ancestor_record_offset":137988,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":187,"record_offset":160960,"parent_record_offset":160932,"ancestor_record_offset":137988,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":143040,"parent_record_offset":143012,"ancestor_record_offset":137988,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":143072,"parent_record_offset":143040,"ancestor_record_offset":137988,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":187,"record_offset":143012,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":142872,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":143484,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":187,"record_offset":142956,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":187,"record_offset":142844,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":187,"record_offset":143364,"parent_record_offset":143340,"ancestor_record_offset":137988,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":143700,"parent_record_offset":143664,"ancestor_record_offset":137988,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":143724,"parent_record_offset":143700,"ancestor_record_offset":137988,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":142652,"parent_record_offset":142628,"ancestor_record_offset":137988,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":142628,"parent_record_offset":142604,"ancestor_record_offset":137988,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":143340,"parent_record_offset":143172,"ancestor_record_offset":137988,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":187,"record_offset":142684,"parent_record_offset":142652,"ancestor_record_offset":137988,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143280,"parent_record_offset":143196,"ancestor_record_offset":137988,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143224,"parent_record_offset":143196,"ancestor_record_offset":137988,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143576,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143196,"parent_record_offset":143172,"ancestor_record_offset":137988,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":142736,"parent_record_offset":142712,"ancestor_record_offset":137988,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143784,"parent_record_offset":143724,"ancestor_record_offset":137988,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":142712,"parent_record_offset":142652,"ancestor_record_offset":137988,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":142760,"parent_record_offset":142736,"ancestor_record_offset":137988,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":142816,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":187,"record_offset":143172,"parent_record_offset":142760,"ancestor_record_offset":137988,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":187,"record_offset":152648,"parent_record_offset":152464,"ancestor_record_offset":137988,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":187,"record_offset":144884,"parent_record_offset":144772,"ancestor_record_offset":137988,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":187,"record_offset":158404,"parent_record_offset":158376,"ancestor_record_offset":137988,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":187,"record_offset":144976,"parent_record_offset":144884,"ancestor_record_offset":137988,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":187,"record_offset":158440,"parent_record_offset":158404,"ancestor_record_offset":137988,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":140628,"parent_record_offset":140592,"ancestor_record_offset":137988,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":153512,"parent_record_offset":153472,"ancestor_record_offset":137988,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":154080,"parent_record_offset":153808,"ancestor_record_offset":137988,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":187,"record_offset":140772,"parent_record_offset":140744,"ancestor_record_offset":137988,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":140848,"parent_record_offset":140700,"ancestor_record_offset":137988,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":187,"record_offset":141108,"parent_record_offset":140848,"ancestor_record_offset":137988,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":187,"record_offset":143664,"parent_record_offset":143632,"ancestor_record_offset":137988,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":187,"record_offset":161084,"parent_record_offset":161036,"ancestor_record_offset":137988,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":187,"record_offset":140800,"parent_record_offset":140772,"ancestor_record_offset":137988,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":187,"record_offset":140700,"parent_record_offset":140664,"ancestor_record_offset":137988,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":187,"record_offset":140664,"parent_record_offset":140628,"ancestor_record_offset":137988,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":148252,"parent_record_offset":148208,"ancestor_record_offset":137988,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":148376,"parent_record_offset":148208,"ancestor_record_offset":137988,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":187,"record_offset":147352,"parent_record_offset":147308,"ancestor_record_offset":137988,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$6::reinterpret_as_curve2::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":187,"record_offset":147308,"parent_record_offset":147276,"ancestor_record_offset":137988,"inlinee":9584,"source_line":1139,"archive":"_pdb/B/modules/00187.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E5E
// 首条位置: {"module":104,"inlinee":11870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":104,"record_offset":92908,"parent_record_offset":92884,"ancestor_record_offset":92884,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":104,"record_offset":92940,"parent_record_offset":92908,"ancestor_record_offset":92884,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":104,"record_offset":92964,"parent_record_offset":92940,"ancestor_record_offset":92884,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":104,"record_offset":92988,"parent_record_offset":92964,"ancestor_record_offset":92884,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3790
// 首条位置: {"module":184,"inlinee":14224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":20528,"parent_record_offset":20472,"ancestor_record_offset":20472,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":20556,"parent_record_offset":20528,"ancestor_record_offset":20472,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":20584,"parent_record_offset":20556,"ancestor_record_offset":20472,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E74
// 首条位置: {"module":104,"inlinee":11892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 576
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3791
// 首条位置: {"module":184,"inlinee":14225}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":26092,"parent_record_offset":25828,"ancestor_record_offset":25828,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":184,"record_offset":25924,"parent_record_offset":25828,"ancestor_record_offset":25828,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":184,"record_offset":25952,"parent_record_offset":25828,"ancestor_record_offset":25828,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":184,"record_offset":25896,"parent_record_offset":25828,"ancestor_record_offset":25828,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":26268,"parent_record_offset":26232,"ancestor_record_offset":25828,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":184,"record_offset":26060,"parent_record_offset":25952,"ancestor_record_offset":25828,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":184,"record_offset":26004,"parent_record_offset":25952,"ancestor_record_offset":25828,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":184,"record_offset":25976,"parent_record_offset":25952,"ancestor_record_offset":25828,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":184,"record_offset":26204,"parent_record_offset":26164,"ancestor_record_offset":25828,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":26128,"parent_record_offset":26092,"ancestor_record_offset":25828,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":26164,"parent_record_offset":26128,"ancestor_record_offset":25828,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":26232,"parent_record_offset":26164,"ancestor_record_offset":25828,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 605
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E63
// 首条位置: {"module":104,"inlinee":11875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":104,"record_offset":112536,"parent_record_offset":112480,"ancestor_record_offset":112480,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":104,"record_offset":112508,"parent_record_offset":112480,"ancestor_record_offset":112480,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":112624,"parent_record_offset":112600,"ancestor_record_offset":112480,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":112648,"parent_record_offset":112624,"ancestor_record_offset":112480,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":104,"record_offset":112688,"parent_record_offset":112536,"ancestor_record_offset":112480,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":104,"record_offset":112716,"parent_record_offset":112536,"ancestor_record_offset":112480,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":104,"record_offset":112600,"parent_record_offset":112576,"ancestor_record_offset":112480,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":104,"record_offset":112576,"parent_record_offset":112536,"ancestor_record_offset":112480,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>
// 记录形式: compiled_procedure; instances=1
// 源码行: 607
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":183,"record_offset":2572,"rva":4764448,"va":5373473568}
// 编译实例 RVAs: 0x48b320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140153bc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3373115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e8c587ccff","size":5,"soft_fail":false,"target_va":5370100672,"va":5373473782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1728712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e8389fe5ff","size":5,"soft_fail":false,"target_va":5371746832,"va":5373475539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1605903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e80f811800","size":5,"soft_fail":false,"target_va":5375081504,"va":5373475596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3984723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e853cd3c00","size":5,"soft_fail":false,"target_va":5377460160,"va":5373475432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8460038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e806178100","size":5,"soft_fail":false,"target_va":5381934528,"va":5373474485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38374208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e8408b4902","size":5,"soft_fail":false,"target_va":5411848096,"va":5373473883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t38066582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e896d94402","size":5,"soft_fail":false,"target_va":5411542296,"va":5373475709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4266627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e8831a4100","size":5,"soft_fail":false,"target_va":5377742032,"va":5373475400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8407907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373473568,"raw_hex":"e8634b8000","size":5,"soft_fail":false,"target_va":5381881936,"va":5373474024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":183,"record_offset":3640,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":183,"record_offset":3612,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$121::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":183,"record_offset":6944,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":14177,"source_line":57,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":183,"record_offset":8456,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":183,"record_offset":3668,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":183,"record_offset":7152,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":14180,"source_line":706,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":3752,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":6844,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":7000,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":5348,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":13651,"source_line":1297,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2840,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":6752,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":2928,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":183,"record_offset":3792,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":183,"record_offset":4296,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":3584,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":183,"record_offset":3016,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":14175,"source_line":610,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":8428,"parent_record_offset":2572,"ancestor_record_offset":2572,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":183,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":2572,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":183,"record_offset":8620,"parent_record_offset":8528,"ancestor_record_offset":2572,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":183,"record_offset":8556,"parent_record_offset":8528,"ancestor_record_offset":2572,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":183,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":2572,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":183,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":2572,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":183,"record_offset":3124,"parent_record_offset":3060,"ancestor_record_offset":2572,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":183,"record_offset":7060,"parent_record_offset":7028,"ancestor_record_offset":2572,"inlinee":14178,"source_line":829,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":183,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":2572,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":183,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":2572,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":183,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":2572,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":183,"record_offset":8588,"parent_record_offset":8556,"ancestor_record_offset":2572,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2572,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":2572,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2572,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":2572,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":183,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":2572,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":183,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":2572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":183,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":2572,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":183,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":2572,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":183,"record_offset":3880,"parent_record_offset":3832,"ancestor_record_offset":2572,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":183,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":2572,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":183,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":2572,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":183,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":2572,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":183,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":2572,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":183,"record_offset":3832,"parent_record_offset":3792,"ancestor_record_offset":2572,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":183,"record_offset":3912,"parent_record_offset":3832,"ancestor_record_offset":2572,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":183,"record_offset":8112,"parent_record_offset":7192,"ancestor_record_offset":2572,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":183,"record_offset":7192,"parent_record_offset":7152,"ancestor_record_offset":2572,"inlinee":14182,"source_line":571,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":183,"record_offset":7296,"parent_record_offset":7256,"ancestor_record_offset":2572,"inlinee":14185,"source_line":554,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":183,"record_offset":7256,"parent_record_offset":7192,"ancestor_record_offset":2572,"inlinee":14184,"source_line":721,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":2572,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":2572,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5208,"parent_record_offset":5176,"ancestor_record_offset":2572,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":2572,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5152,"parent_record_offset":4380,"ancestor_record_offset":2572,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5124,"parent_record_offset":4380,"ancestor_record_offset":2572,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5008,"parent_record_offset":4716,"ancestor_record_offset":2572,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5064,"parent_record_offset":4716,"ancestor_record_offset":2572,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":4952,"parent_record_offset":4716,"ancestor_record_offset":2572,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":6872,"parent_record_offset":6844,"ancestor_record_offset":2572,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":8256,"parent_record_offset":8220,"ancestor_record_offset":2572,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":2572,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":7348,"parent_record_offset":7296,"ancestor_record_offset":2572,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":183,"record_offset":7812,"parent_record_offset":7680,"ancestor_record_offset":2572,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":183,"record_offset":4000,"parent_record_offset":3792,"ancestor_record_offset":2572,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":5296,"parent_record_offset":5264,"ancestor_record_offset":2572,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":2572,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3220,"parent_record_offset":3060,"ancestor_record_offset":2572,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":2572,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x375F
// 首条位置: {"module":183,"inlinee":14175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5152,"parent_record_offset":4380,"ancestor_record_offset":4380,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5124,"parent_record_offset":4380,"ancestor_record_offset":4380,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":3016,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":183,"record_offset":3124,"parent_record_offset":3060,"ancestor_record_offset":3016,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":4380,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4380,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":183,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4380,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":183,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":3016,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":183,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":3016,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":4380,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4380,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5208,"parent_record_offset":5176,"ancestor_record_offset":4380,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":4380,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5008,"parent_record_offset":4716,"ancestor_record_offset":4380,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":5064,"parent_record_offset":4716,"ancestor_record_offset":4380,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":183,"record_offset":4952,"parent_record_offset":4716,"ancestor_record_offset":4380,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":183,"record_offset":5296,"parent_record_offset":5264,"ancestor_record_offset":4380,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":3016,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":183,"record_offset":3220,"parent_record_offset":3060,"ancestor_record_offset":3016,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00183.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 865
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x378C
// 首条位置: {"module":184,"inlinee":14220}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":16160,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":184,"record_offset":16424,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":184,"record_offset":16352,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":184,"record_offset":16508,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":184,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":16104,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":184,"record_offset":16384,"parent_record_offset":16352,"ancestor_record_offset":16104,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":16188,"parent_record_offset":16160,"ancestor_record_offset":16104,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":184,"record_offset":16216,"parent_record_offset":16188,"ancestor_record_offset":16104,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 881
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E64
// 首条位置: {"module":104,"inlinee":11876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 977
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E79
// 首条位置: {"module":104,"inlinee":11897}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls
// 信息 ID: d833a1e3a43dc9f2ea5797e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=187, inline_record=137988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$1
// 信息 ID: 7474c3387bbc8266bdbb47c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$1<false> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$1<false> > >; module=104, inline_record=92884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$2
// 信息 ID: e2331accc7720e1509d467be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<enum2$<core::option::Option<f32> >,usize>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<enum2$<core::option::Option<f32> >,usize>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<true> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<enum2$<core::option::Option<f32> >,usize>,core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<enum2$<core::option::Option<f32> >,usize>,core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<enum2$<core::option::Option<f32> >,usize>,core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<enum2$<core::option::Option<f32> >,usize>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<usize,country_core::resources::stairs_walls::StairArchType> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$2<false> > > > >; module=184, inline_record=20472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$3
// 信息 ID: c82a0770311abb8b07f27a4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec2::Vec2>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$3<true> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec2::Vec2>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$3<true> > >; module=104, inline_record=127796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$4
// 信息 ID: e7f7ce98484719dda5ae1195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<true> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4 <- core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<f32,alloc::vec::impl$20::extend_trusted::closure_env$0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$4<false> > > > >; module=184, inline_record=25828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$5
// 信息 ID: 686afa45123f0eace3b6b3f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$5<false> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$5<false> > >; module=104, inline_record=112480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6::closure$0
// 信息 ID: ef766c809b6b2416f8aecfb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>; module=183, inline_record=3016; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>; module=183, inline_record=4380; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>; module=183, inline_record=5408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6<true>
// 信息 ID: 24d8f949c6644f8af0e926ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$6 -> ["true"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6 <- system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> > @ 0x48b0d0 -> 0x48b320 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> > @ 0x48b206 -> 0x48b320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$7
// 信息 ID: 5dffeff1aef6335f73e6b50e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<false> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec2::Vec2> >,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$7<true> > > > >; module=184, inline_record=16104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$8
// 信息 ID: 3e959da928f3baa7a2bad12d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$8 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$8<false> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$8<false> > >; module=104, inline_record=114200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$9
// 信息 ID: 140c12117aa7ffd710c3f055
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > >; module=104, inline_record=135536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true>
// 信息 ID: f8be083e456600819f039f6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls -> ["true"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> > > @ 0x479a32 -> 0x496800 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> > @ 0x495509 -> 0x496800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve
// 信息 ID: 8c02880a7ce9702479b5a101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> @ 0x497288 -> 0x780e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x49d622 -> 0x780e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0
// 信息 ID: 64db793e81faaffd27958ab0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::preprocess_curve @ 0x78107d -> 0x782f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::preprocess_curve @ 0x78109d -> 0x782f50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::preprocess_curve @ 0x781797 -> 0x782f50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$0
// 信息 ID: ddb600ae391ce15ea5dec5f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0; module=409, inline_record=14652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0::closure$1
// 信息 ID: cffb76c9fa17609cd531e2ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$0; module=409, inline_record=15568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$1
// 信息 ID: a66fb016c260778fc8f1dd61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1>,tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1> > > >; module=1418, inline_record=1668; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1> > > >; module=1418, inline_record=2260; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1> > > >; module=1418, inline_record=2844; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env$1>,tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >; module=1452, inline_record=7404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$2
// 信息 ID: 8d023a559a265e1099974c31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::preprocess_curve; module=409, inline_record=1764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure$3
// 信息 ID: 6698501ad93244760998bd91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_playermade_stairs::preprocess_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::preprocess_curve; module=409, inline_record=6372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
