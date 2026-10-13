// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/decorator_plugin.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f2e445fa413f3dba6ad3331beb490e45620c6de434a8f22a10400e14ead11244
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_plugin.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 232
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use indexmap; // namespace candidate
//   package identities: [{"name":"indexmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.13.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::decorator_plugin::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":232,"record_offset":44804,"rva":6226976,"va":5374936096}
// 编译实例 RVAs: 0x5f0420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=56
//     evidence={"assembly":"\tcallq\t9706525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e81d1c9400","size":5,"soft_fail":false,"target_va":5384644912,"va":5374938382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=8
//     target_aliases=118 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5748292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8bc49a8ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5374953951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140079590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5739400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8786ca8ff","size":5,"soft_fail":false,"target_va":5369206160,"va":5374945555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400796a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5739189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e84b6da8ff","size":5,"soft_fail":false,"target_va":5369206432,"va":5374945616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081d60
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5710007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e849dfa8ff","size":5,"soft_fail":false,"target_va":5369240928,"va":5374950930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087a30
//     owner=unresolved_namespace; count=4
//     target_aliases=49 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5687735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e84936a9ff","size":5,"soft_fail":false,"target_va":5369264688,"va":5374952418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140088a20
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5684935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e83941a9ff","size":5,"soft_fail":false,"target_va":5369268768,"va":5374953698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5680580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e83c52a9ff","size":5,"soft_fail":false,"target_va":5369271584,"va":5374952159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008b5f0
//     owner=unresolved_namespace; count=5
//     target_aliases=28 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5668759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e86980a9ff","size":5,"soft_fail":false,"target_va":5369279984,"va":5374948738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008dc30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5659863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e829a3a9ff","size":5,"soft_fail":false,"target_va":5369289776,"va":5374949634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090880
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5649159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8f9cca9ff","size":5,"soft_fail":false,"target_va":5369301120,"va":5374950274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140092c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5639863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e849f1a9ff","size":5,"soft_fail":false,"target_va":5369310288,"va":5374950146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5638231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8a9f7a9ff","size":5,"soft_fail":false,"target_va":5369311792,"va":5374950018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093c80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5636247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e869ffa9ff","size":5,"soft_fail":false,"target_va":5369314432,"va":5374950674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400945d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5632695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8490daaff","size":5,"soft_fail":false,"target_va":5369316816,"va":5374949506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140094cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5633687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e86909aaff","size":5,"soft_fail":false,"target_va":5369318608,"va":5374952290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140095770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5631735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e80911aaff","size":5,"soft_fail":false,"target_va":5369321328,"va":5374953058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140095870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5627671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8e920aaff","size":5,"soft_fail":false,"target_va":5369321584,"va":5374949250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140095c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5629959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8f917aaff","size":5,"soft_fail":false,"target_va":5369322592,"va":5374952546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140095d60
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5626279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e85926aaff","size":5,"soft_fail":false,"target_va":5369322848,"va":5374949122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400964a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5628231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8b91eaaff","size":5,"soft_fail":false,"target_va":5369324704,"va":5374952930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140096990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5622903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e88933aaff","size":5,"soft_fail":false,"target_va":5369325968,"va":5374948866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140097060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5621287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8d939aaff","size":5,"soft_fail":false,"target_va":5369327712,"va":5374948994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5451767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e809d0acff","size":5,"soft_fail":false,"target_va":5369500000,"va":5374951762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5438551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8a903adff","size":5,"soft_fail":false,"target_va":5369512000,"va":5374950546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=10
//     target_aliases=34 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5279999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8016fafff","size":5,"soft_fail":false,"target_va":5369660368,"va":5374940362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5164840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8d830b1ff","size":5,"soft_fail":false,"target_va":5369774336,"va":5374939171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140190a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4595522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8bee0b9ff","size":5,"soft_fail":false,"target_va":5370350112,"va":5374945629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a5040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4513458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e84e21bbff","size":5,"soft_fail":false,"target_va":5370433600,"va":5374947053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4348768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8a0a4bdff","size":5,"soft_fail":false,"target_va":5370597904,"va":5374946667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ce5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4342624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8a0bcbdff","size":5,"soft_fail":false,"target_va":5370602928,"va":5374945547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ce970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4343191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e869babdff","size":5,"soft_fail":false,"target_va":5370603888,"va":5374947074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cf3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4340481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8ffc4bdff","size":5,"soft_fail":false,"target_va":5370606560,"va":5374947036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cf5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4338504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8b8ccbdff","size":5,"soft_fail":false,"target_va":5370607040,"va":5374945539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cf7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4337996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8b4cebdff","size":5,"soft_fail":false,"target_va":5370607520,"va":5374945511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cfb40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4337151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e801d2bdff","size":5,"soft_fail":false,"target_va":5370608448,"va":5374945594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402310b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3938865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8cfe5c3ff","size":5,"soft_fail":false,"target_va":5371007152,"va":5374946012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140245bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3854765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8532ec5ff","size":5,"soft_fail":false,"target_va":5371091888,"va":5374946648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033bbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2846072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e88892d4ff","size":5,"soft_fail":false,"target_va":5372099536,"va":5374945603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033c070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2845153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e81f96d4ff","size":5,"soft_fail":false,"target_va":5372100720,"va":5374945868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403c9a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2270522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8c65addff","size":5,"soft_fail":false,"target_va":5372680848,"va":5374951365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cc500
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2258714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8e688ddff","size":5,"soft_fail":false,"target_va":5372691712,"va":5374950421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ccfb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2257466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8c68dddff","size":5,"soft_fail":false,"target_va":5372694448,"va":5374951909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cd1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2256106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e81693ddff","size":5,"soft_fail":false,"target_va":5372694976,"va":5374951077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14044eed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1719182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e872c4e5ff","size":5,"soft_fail":false,"target_va":5373226704,"va":5374945881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140450360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1713892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e81cd9e5ff","size":5,"soft_fail":false,"target_va":5373231968,"va":5374945855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140578ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-499627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e85560f8ff","size":5,"soft_fail":false,"target_va":5374446272,"va":5374945894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-373677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8534cfaff","size":5,"soft_fail":false,"target_va":5374573152,"va":5374946824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ec660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8299cffff","size":5,"soft_fail":false,"target_va":5374920288,"va":5374945842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067e200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t571596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8ccb80800","size":5,"soft_fail":false,"target_va":5375517184,"va":5374945583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42116045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8cda38202","size":5,"soft_fail":false,"target_va":5417057056,"va":5374941006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=39
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-975819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8351cf1ff","size":5,"soft_fail":false,"target_va":5373962560,"va":5374938374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t387894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e836eb0500","size":5,"soft_fail":false,"target_va":5375324304,"va":5374936405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::after
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e83f4b0000","size":5,"soft_fail":false,"target_va":5374964768,"va":5374945500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-4356316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e82487bdff","size":5,"soft_fail":false,"target_va":5370590304,"va":5374946615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8554d0000","size":5,"soft_fail":false,"target_va":5374965328,"va":5374945526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::impl$10::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e871aeffff","size":5,"soft_fail":false,"target_va":5374922112,"va":5374942986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e8dfe6ffff","size":5,"soft_fail":false,"target_va":5374932288,"va":5374938716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=56
//     evidence={"assembly":"\tcallq\t9779541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e855399500","size":5,"soft_fail":false,"target_va":5384726688,"va":5374947142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t41670780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374936096,"raw_hex":"e87cd87b02","size":5,"soft_fail":false,"target_va":5416619360,"va":5374948575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":232,"record_offset":45076,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":232,"record_offset":105952,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":232,"record_offset":106064,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":232,"record_offset":80144,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17178,"source_line":10,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_assembly_params::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":232,"record_offset":82336,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17210,"source_line":21,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_collider::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_collider.rs
//     evidence={"module":232,"record_offset":91240,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17180,"source_line":7,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_removed_supports::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_removed_supports.rs
//     evidence={"module":232,"record_offset":92772,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17226,"source_line":8,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$33::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":232,"record_offset":89800,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17224,"source_line":73,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::structural_graph::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/structural_graph.rs
//     evidence={"module":232,"record_offset":97288,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17242,"source_line":23,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":232,"record_offset":90520,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17182,"source_line":32,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":232,"record_offset":87576,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17216,"source_line":39,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":232,"record_offset":87604,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17218,"source_line":82,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":232,"record_offset":87632,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17220,"source_line":102,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":232,"record_offset":87520,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17212,"source_line":14,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs_walls.rs
//     evidence={"module":232,"record_offset":87548,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17214,"source_line":38,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":232,"record_offset":64700,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17198,"source_line":10,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$179::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":232,"record_offset":60088,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17193,"source_line":809,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":232,"record_offset":74560,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17176,"source_line":4,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":232,"record_offset":60176,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17154,"source_line":414,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":232,"record_offset":69144,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17204,"source_line":78,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::impl$30::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":232,"record_offset":69864,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17206,"source_line":131,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":232,"record_offset":68396,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17174,"source_line":17,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":232,"record_offset":89080,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17222,"source_line":35,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":232,"record_offset":65508,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17172,"source_line":38,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::backup::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/backup.rs
//     evidence={"module":232,"record_offset":75328,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17208,"source_line":17,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":73088,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17158,"source_line":276,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":73808,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17160,"source_line":281,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::deco_differential_state::impl$2::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/deco_differential_state.rs
//     evidence={"module":232,"record_offset":80864,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":13534,"source_line":63,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":63208,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17164,"source_line":76,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":232,"record_offset":62488,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17162,"source_line":17,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_decorator_affordance_dispatch::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs
//     evidence={"module":232,"record_offset":69116,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17202,"source_line":76,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":232,"record_offset":103064,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17244,"source_line":61,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":232,"record_offset":104512,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17246,"source_line":71,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_focus_stairs::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs
//     evidence={"module":232,"record_offset":105232,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17248,"source_line":79,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_move_decorator::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_move_decorator.rs
//     evidence={"module":232,"record_offset":65452,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17200,"source_line":85,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_place_decorator::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs
//     evidence={"module":232,"record_offset":65480,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17170,"source_line":60,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_stairs_viz::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":93492,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":17228,"source_line":105,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":46472,"parent_record_offset":46448,"ancestor_record_offset":44804,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":46520,"parent_record_offset":46496,"ancestor_record_offset":44804,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":46496,"parent_record_offset":46472,"ancestor_record_offset":44804,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":46448,"parent_record_offset":46424,"ancestor_record_offset":44804,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":46424,"parent_record_offset":46384,"ancestor_record_offset":44804,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":82464,"parent_record_offset":82440,"ancestor_record_offset":44804,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":82440,"parent_record_offset":82336,"ancestor_record_offset":44804,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":46384,"parent_record_offset":46360,"ancestor_record_offset":44804,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":232,"record_offset":45100,"parent_record_offset":45076,"ancestor_record_offset":44804,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":232,"record_offset":45160,"parent_record_offset":45128,"ancestor_record_offset":44804,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":232,"record_offset":106104,"parent_record_offset":106064,"ancestor_record_offset":44804,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":232,"record_offset":46692,"parent_record_offset":46668,"ancestor_record_offset":44804,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":232,"record_offset":46668,"parent_record_offset":46644,"ancestor_record_offset":44804,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":232,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":44804,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":232,"record_offset":46180,"parent_record_offset":46156,"ancestor_record_offset":44804,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":232,"record_offset":46156,"parent_record_offset":46132,"ancestor_record_offset":44804,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":232,"record_offset":46360,"parent_record_offset":46336,"ancestor_record_offset":44804,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":232,"record_offset":45128,"parent_record_offset":45100,"ancestor_record_offset":44804,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":232,"record_offset":46108,"parent_record_offset":46084,"ancestor_record_offset":44804,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":232,"record_offset":45184,"parent_record_offset":45160,"ancestor_record_offset":44804,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":232,"record_offset":45832,"parent_record_offset":45808,"ancestor_record_offset":44804,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":232,"record_offset":45860,"parent_record_offset":45832,"ancestor_record_offset":44804,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":60772,"parent_record_offset":60744,"ancestor_record_offset":44804,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=122
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":60672,"parent_record_offset":60468,"ancestor_record_offset":44804,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":74844,"parent_record_offset":74788,"ancestor_record_offset":44804,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":232,"record_offset":45400,"parent_record_offset":45376,"ancestor_record_offset":44804,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":232,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":44804,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=178
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":232,"record_offset":60636,"parent_record_offset":60612,"ancestor_record_offset":44804,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":232,"record_offset":60800,"parent_record_offset":60744,"ancestor_record_offset":44804,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":232,"record_offset":45632,"parent_record_offset":45604,"ancestor_record_offset":44804,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":232,"record_offset":45604,"parent_record_offset":45576,"ancestor_record_offset":44804,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=130
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":232,"record_offset":46272,"parent_record_offset":46208,"ancestor_record_offset":44804,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":46084,"parent_record_offset":45128,"ancestor_record_offset":44804,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":46296,"parent_record_offset":46272,"ancestor_record_offset":44804,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":46620,"parent_record_offset":45128,"ancestor_record_offset":44804,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":46336,"parent_record_offset":46156,"ancestor_record_offset":44804,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":60500,"parent_record_offset":60468,"ancestor_record_offset":44804,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":232,"record_offset":74760,"parent_record_offset":74732,"ancestor_record_offset":44804,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":45424,"parent_record_offset":45400,"ancestor_record_offset":44804,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":62372,"parent_record_offset":60176,"ancestor_record_offset":44804,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":232,"record_offset":60588,"parent_record_offset":60468,"ancestor_record_offset":44804,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":232,"record_offset":45748,"parent_record_offset":45724,"ancestor_record_offset":44804,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":60612,"parent_record_offset":60588,"ancestor_record_offset":44804,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":232,"record_offset":46208,"parent_record_offset":46180,"ancestor_record_offset":44804,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":232,"record_offset":46244,"parent_record_offset":46208,"ancestor_record_offset":44804,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":232,"record_offset":46564,"parent_record_offset":46384,"ancestor_record_offset":44804,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":232,"record_offset":78444,"parent_record_offset":75328,"ancestor_record_offset":44804,"inlinee":17198,"source_line":10,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::cached_decorator_transforms::impl$5::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/cached_decorator_transforms.rs
//     evidence={"module":232,"record_offset":64736,"parent_record_offset":64700,"ancestor_record_offset":44804,"inlinee":13446,"source_line":16,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$179::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":232,"record_offset":79916,"parent_record_offset":75328,"ancestor_record_offset":44804,"inlinee":17193,"source_line":809,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$21::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":232,"record_offset":61656,"parent_record_offset":60176,"ancestor_record_offset":44804,"inlinee":17156,"source_line":342,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":232,"record_offset":75408,"parent_record_offset":75328,"ancestor_record_offset":44804,"inlinee":17154,"source_line":414,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":232,"record_offset":79200,"parent_record_offset":75328,"ancestor_record_offset":44804,"inlinee":17204,"source_line":78,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":232,"record_offset":74608,"parent_record_offset":74560,"ancestor_record_offset":44804,"inlinee":11146,"source_line":286,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":232,"record_offset":45808,"parent_record_offset":45784,"ancestor_record_offset":44804,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":45544,"parent_record_offset":45516,"ancestor_record_offset":44804,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":45576,"parent_record_offset":45544,"ancestor_record_offset":44804,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":45516,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":232,"record_offset":45376,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":232,"record_offset":45936,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":232,"record_offset":45460,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":232,"record_offset":45220,"parent_record_offset":45184,"ancestor_record_offset":44804,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":232,"record_offset":45784,"parent_record_offset":45676,"ancestor_record_offset":44804,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45724,"parent_record_offset":45700,"ancestor_record_offset":44804,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":46028,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45700,"parent_record_offset":45676,"ancestor_record_offset":44804,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45272,"parent_record_offset":45248,"ancestor_record_offset":44804,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45248,"parent_record_offset":45220,"ancestor_record_offset":44804,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45296,"parent_record_offset":45272,"ancestor_record_offset":44804,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45348,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":62444,"parent_record_offset":62420,"ancestor_record_offset":44804,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":45676,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":232,"record_offset":45908,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::with_capacity_and_hasher
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":232,"record_offset":83948,"parent_record_offset":83244,"ancestor_record_offset":44804,"inlinee":5345,"source_line":165,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] indexmap::map::IndexMap::with_hasher
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"indexmap","version":"2.13.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/indexmap-2.13.0/src/map.rs
//     evidence={"module":232,"record_offset":83972,"parent_record_offset":83948,"ancestor_record_offset":44804,"inlinee":5352,"source_line":180,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":232,"record_offset":60140,"parent_record_offset":60116,"ancestor_record_offset":44804,"inlinee":17196,"source_line":370,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":232,"record_offset":60116,"parent_record_offset":60088,"ancestor_record_offset":44804,"inlinee":17195,"source_line":1223,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::with_capacity
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":232,"record_offset":83244,"parent_record_offset":83208,"ancestor_record_offset":44804,"inlinee":17152,"source_line":204,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$19::default
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":232,"record_offset":83208,"parent_record_offset":82336,"ancestor_record_offset":44804,"inlinee":17151,"source_line":1012,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":232,"record_offset":60912,"parent_record_offset":60236,"ancestor_record_offset":44804,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":232,"record_offset":60236,"parent_record_offset":60176,"ancestor_record_offset":44804,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":232,"record_offset":62524,"parent_record_offset":62488,"ancestor_record_offset":44804,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":60292,"parent_record_offset":60264,"ancestor_record_offset":44804,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":232,"record_offset":60548,"parent_record_offset":60524,"ancestor_record_offset":44804,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":232,"record_offset":60408,"parent_record_offset":60380,"ancestor_record_offset":44804,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":60744,"parent_record_offset":60348,"ancestor_record_offset":44804,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":60264,"parent_record_offset":60236,"ancestor_record_offset":44804,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":232,"record_offset":60436,"parent_record_offset":60408,"ancestor_record_offset":44804,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":232,"record_offset":60468,"parent_record_offset":60436,"ancestor_record_offset":44804,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":232,"record_offset":60348,"parent_record_offset":60320,"ancestor_record_offset":44804,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=66
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":232,"record_offset":60320,"parent_record_offset":60292,"ancestor_record_offset":44804,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":73844,"parent_record_offset":73808,"ancestor_record_offset":44804,"inlinee":17158,"source_line":276,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":63260,"parent_record_offset":63208,"ancestor_record_offset":44804,"inlinee":17166,"source_line":29,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":63980,"parent_record_offset":63208,"ancestor_record_offset":44804,"inlinee":17168,"source_line":40,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96488,"parent_record_offset":93492,"ancestor_record_offset":44804,"inlinee":17234,"source_line":61,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96516,"parent_record_offset":93492,"ancestor_record_offset":44804,"inlinee":17236,"source_line":68,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96544,"parent_record_offset":93492,"ancestor_record_offset":44804,"inlinee":17238,"source_line":75,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96572,"parent_record_offset":93492,"ancestor_record_offset":44804,"inlinee":17240,"source_line":82,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96432,"parent_record_offset":93492,"ancestor_record_offset":44804,"inlinee":17230,"source_line":89,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::ui_stairs_viz::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs
//     evidence={"module":232,"record_offset":96456,"parent_record_offset":96432,"ancestor_record_offset":44804,"inlinee":17232,"source_line":1212,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::decorator_plugin::decorators_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":232,"record_offset":3992,"rva":6214064,"va":5374923184}
// 编译实例 RVAs: 0x5ed1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=16
//     evidence={"assembly":"\tcallq\t9720942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e86e549400","size":5,"soft_fail":false,"target_va":5384644912,"va":5374923965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t753898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8ea800b00","size":5,"soft_fail":false,"target_va":5375678704,"va":5374924801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007dba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5702260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e88cfda8ff","size":5,"soft_fail":false,"target_va":5369224096,"va":5374926351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081d60
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5690766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8722aa9ff","size":5,"soft_fail":false,"target_va":5369240928,"va":5374931689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008b5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5648830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e842cea9ff","size":5,"soft_fail":false,"target_va":5369279984,"va":5374928809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008dc30
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5639742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8c2f1a9ff","size":5,"soft_fail":false,"target_va":5369289776,"va":5374929513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f490
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5630630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e85a15aaff","size":5,"soft_fail":false,"target_va":5369296016,"va":5374926641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5629166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8121baaff","size":5,"soft_fail":false,"target_va":5369301120,"va":5374930281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093b80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5614747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e86553aaff","size":5,"soft_fail":false,"target_va":5369314176,"va":5374928918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093c80
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5617038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8724aaaff","size":5,"soft_fail":false,"target_va":5369314432,"va":5374931465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5612235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8355daaff","size":5,"soft_fail":false,"target_va":5369314704,"va":5374926934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140094980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5609802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8b666aaff","size":5,"soft_fail":false,"target_va":5369317760,"va":5374927557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140096990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5603278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e83280aaff","size":5,"soft_fail":false,"target_va":5369325968,"va":5374929241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140097060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5602302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e80284aaff","size":5,"soft_fail":false,"target_va":5369327712,"va":5374930009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bb300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5451189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e84bd2acff","size":5,"soft_fail":false,"target_va":5369475840,"va":5374927024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5441053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8e3f9acff","size":5,"soft_fail":false,"target_va":5369486752,"va":5374927800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400be380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5436832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8600aadff","size":5,"soft_fail":false,"target_va":5369488256,"va":5374925083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400be7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5436375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8290cadff","size":5,"soft_fail":false,"target_va":5369489328,"va":5374925698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5424264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8783badff","size":5,"soft_fail":false,"target_va":5369500000,"va":5374924259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c2a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5421032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e81848adff","size":5,"soft_fail":false,"target_va":5369506336,"va":5374927363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c2ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5417991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8f953adff","size":5,"soft_fail":false,"target_va":5369506720,"va":5374924706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5419147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8754fadff","size":5,"soft_fail":false,"target_va":5369512000,"va":5374931142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=11
//     target_aliases=34 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5265534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e882a7afff","size":5,"soft_fail":false,"target_va":5369660368,"va":5374925897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5150865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e86f67b1ff","size":5,"soft_fail":false,"target_va":5369774336,"va":5374925196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cc500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2238705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e80fd7ddff","size":5,"soft_fail":false,"target_va":5372691712,"va":5374930412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ccfb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2230047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8e1f8ddff","size":5,"soft_fail":false,"target_va":5372694448,"va":5374924490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41424423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e827167802","size":5,"soft_fail":false,"target_va":5416353040,"va":5374928612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: decorator_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42132322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e862e38202","size":5,"soft_fail":false,"target_va":5417057056,"va":5374924729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::impl$10::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8e2ecffff","size":5,"soft_fail":false,"target_va":5374922112,"va":5374927001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6732051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e813b96600","size":5,"soft_fail":false,"target_va":5381660000,"va":5374927944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::impl$1::default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8b01d0000","size":5,"soft_fail":false,"target_va":5374932288,"va":5374924683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t9798590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e8be839500","size":5,"soft_fail":false,"target_va":5384726688,"va":5374928093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t41690726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e866267c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5374928629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2608456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374923184,"raw_hex":"e848cd2700","size":5,"soft_fail":false,"target_va":5377536432,"va":5374927971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":232,"record_offset":6508,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":232,"record_offset":26664,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":11390,"source_line":861,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":7336,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::dormer_tiles::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/dormer_tiles.rs
//     evidence={"module":232,"record_offset":19144,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17178,"source_line":10,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_collider::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_collider.rs
//     evidence={"module":232,"record_offset":22076,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17180,"source_line":7,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::visual::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/visual.rs
//     evidence={"module":232,"record_offset":24472,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17182,"source_line":32,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_collision::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_collision.rs
//     evidence={"module":232,"record_offset":16056,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17176,"source_line":4,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":232,"record_offset":4196,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17154,"source_line":414,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":232,"record_offset":17948,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":15448,"source_line":15,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window_lights::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window_lights.rs
//     evidence={"module":232,"record_offset":14768,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17174,"source_line":17,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::audio::op_confirm_audio::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/audio/op_confirm_audio.rs
//     evidence={"module":232,"record_offset":11824,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17172,"source_line":38,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":6564,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17158,"source_line":276,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":7476,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17160,"source_line":281,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::deco_differential_state::impl$2::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/deco_differential_state.rs
//     evidence={"module":232,"record_offset":21220,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":13534,"source_line":63,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual_state::impl$9::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":10136,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17164,"source_line":76,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::deferred_decorator_history::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/deferred_decorator_history.rs
//     evidence={"module":232,"record_offset":8500,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17162,"source_line":17,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::ui_place_decorator::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs
//     evidence={"module":232,"record_offset":11740,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17170,"source_line":60,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":232,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":3992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":232,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":3992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":232,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":3992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":232,"record_offset":9480,"parent_record_offset":9388,"ancestor_record_offset":3992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":26784,"parent_record_offset":26756,"ancestor_record_offset":3992,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":26868,"parent_record_offset":26844,"ancestor_record_offset":3992,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":3992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":3992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":26844,"parent_record_offset":26812,"ancestor_record_offset":3992,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":3992,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":232,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":3992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":232,"record_offset":26756,"parent_record_offset":26720,"ancestor_record_offset":3992,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":232,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":3992,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":232,"record_offset":26692,"parent_record_offset":26664,"ancestor_record_offset":3992,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":232,"record_offset":26720,"parent_record_offset":26692,"ancestor_record_offset":3992,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":3992,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":4692,"parent_record_offset":4488,"ancestor_record_offset":3992,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":232,"record_offset":16340,"parent_record_offset":16284,"ancestor_record_offset":3992,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":232,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":3992,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=59
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":232,"record_offset":4656,"parent_record_offset":4632,"ancestor_record_offset":3992,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":232,"record_offset":9880,"parent_record_offset":9852,"ancestor_record_offset":3992,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":232,"record_offset":9908,"parent_record_offset":9880,"ancestor_record_offset":3992,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":232,"record_offset":9448,"parent_record_offset":9416,"ancestor_record_offset":3992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":232,"record_offset":4820,"parent_record_offset":4764,"ancestor_record_offset":3992,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":232,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":3992,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":232,"record_offset":20480,"parent_record_offset":20452,"ancestor_record_offset":3992,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":232,"record_offset":4400,"parent_record_offset":4368,"ancestor_record_offset":3992,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":232,"record_offset":4520,"parent_record_offset":4488,"ancestor_record_offset":3992,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":232,"record_offset":20268,"parent_record_offset":20184,"ancestor_record_offset":3992,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":232,"record_offset":16256,"parent_record_offset":16228,"ancestor_record_offset":3992,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":6392,"parent_record_offset":4196,"ancestor_record_offset":3992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":232,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":3992,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":232,"record_offset":4608,"parent_record_offset":4488,"ancestor_record_offset":3992,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":232,"record_offset":9944,"parent_record_offset":9820,"ancestor_record_offset":3992,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":232,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":3992,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":232,"record_offset":5676,"parent_record_offset":4196,"ancestor_record_offset":3992,"inlinee":17156,"source_line":342,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":232,"record_offset":16104,"parent_record_offset":16056,"ancestor_record_offset":3992,"inlinee":11146,"source_line":286,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":20416,"parent_record_offset":20380,"ancestor_record_offset":3992,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":20452,"parent_record_offset":20416,"ancestor_record_offset":3992,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":232,"record_offset":20548,"parent_record_offset":20380,"ancestor_record_offset":3992,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":232,"record_offset":20380,"parent_record_offset":20336,"ancestor_record_offset":3992,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":232,"record_offset":20212,"parent_record_offset":20184,"ancestor_record_offset":3992,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20784,"parent_record_offset":20092,"ancestor_record_offset":3992,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20636,"parent_record_offset":20336,"ancestor_record_offset":3992,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20184,"parent_record_offset":20128,"ancestor_record_offset":3992,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20336,"parent_record_offset":20304,"ancestor_record_offset":3992,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":9820,"parent_record_offset":9788,"ancestor_record_offset":3992,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20156,"parent_record_offset":20128,"ancestor_record_offset":3992,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20092,"parent_record_offset":20032,"ancestor_record_offset":3992,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":3992,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":9788,"parent_record_offset":9732,"ancestor_record_offset":3992,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":232,"record_offset":9760,"parent_record_offset":9732,"ancestor_record_offset":3992,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20128,"parent_record_offset":20092,"ancestor_record_offset":3992,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":9852,"parent_record_offset":9820,"ancestor_record_offset":3992,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":6464,"parent_record_offset":6440,"ancestor_record_offset":3992,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":232,"record_offset":20304,"parent_record_offset":20092,"ancestor_record_offset":3992,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":232,"record_offset":4932,"parent_record_offset":4256,"ancestor_record_offset":3992,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":232,"record_offset":4256,"parent_record_offset":4196,"ancestor_record_offset":3992,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":232,"record_offset":8528,"parent_record_offset":8500,"ancestor_record_offset":3992,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":4312,"parent_record_offset":4284,"ancestor_record_offset":3992,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":232,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":3992,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":232,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":3992,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":4764,"parent_record_offset":4368,"ancestor_record_offset":3992,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":232,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":3992,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":232,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":3992,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":232,"record_offset":4488,"parent_record_offset":4456,"ancestor_record_offset":3992,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":232,"record_offset":4368,"parent_record_offset":4340,"ancestor_record_offset":3992,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":232,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3992,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::blueprint::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":232,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":3992,"inlinee":17158,"source_line":276,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":10188,"parent_record_offset":10136,"ancestor_record_offset":3992,"inlinee":17166,"source_line":29,"archive":"_pdb/C/modules/00232.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual_state::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual_state.rs
//     evidence={"module":232,"record_offset":10908,"parent_record_offset":10136,"ancestor_record_offset":3992,"inlinee":17168,"source_line":40,"archive":"_pdb/C/modules/00232.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::mod_editor::ModEditorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::startup::load_glade_tod_and_weather::SnowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::egui_input::EguiInputPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::grass2::Grass2Plugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::mod_editor::ModEditorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::startup::load_glade_tod_and_weather::SnowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::egui_input::EguiInputPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::grass2::Grass2Plugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: 8f33ca9036dd90e88a57d269
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 28c90495bd362606daff0715: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)> > @ 0x294aed -> 0x5ed1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,bevy_ecs::change_detection::ResMut<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::system::commands::Commands>)> @ 0x546ac2 -> 0x5ed1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::decorator_plugin::impl$0::build
// 信息 ID: 8000fea5d79cca90e83d3fd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
